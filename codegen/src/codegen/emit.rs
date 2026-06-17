// SPDX-License-Identifier: CC0-1.0

//! Back end: turn the IR ([`Modules`]) into Rust source and write the per-category files across the
//! two consumer crates. Method/options emission, doc rendering, and the `mod.rs` assemblers.

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use super::*;
use crate::spec::{Method, Param};

impl Modules {
    /// Write all generated files across both crates, split into per-category modules.
    ///
    /// Response types go to `types/.../generated/`. Categories that produce `into_model`
    /// conversions (see [`crate::into_model`]) become directory modules with a sibling `into.rs`;
    /// the rest stay flat. The shared `compatibility.rs` (manual override shims) is always emitted.
    pub fn write(&self, types_dir: &Path, client_dir: &Path, version: &str) -> Result<(), String> {
        let categories = self.categories();

        // Blocking facade: reuse the sync client's macro surface (its `mod.rs`) over the async
        // transport. Only emitted when a sync client exists for this version.
        let sync_mod = client_dir
            .parent()
            .and_then(Path::parent)
            .map(|src| src.join("client_sync").join(format!("v{version}")).join("mod.rs"));
        let blocking_src = sync_mod.as_ref().and_then(|p| fs::read_to_string(p).ok());
        if let Some(src) = &blocking_src {
            write_file(
                &client_dir.join("blocking.rs"),
                &super::blocking::emit_blocking(version, src),
            )?;
        }

        write_file(&types_dir.join("mod.rs"), &emit_types_mod_rs(version, &categories))?;
        write_file(
            &client_dir.join("mod.rs"),
            &emit_client_mod_rs(version, &categories, blocking_src.is_some()),
        )?;
        write_file(
            &types_dir.join("compatibility.rs"),
            &crate::into_model::emit_compatibility(version),
        )?;

        // The canonical model source the `into_model` generator reads its target shapes from.
        let model_dir = types_dir
            .parent()
            .and_then(Path::parent)
            .map(|src| src.join("model"))
            .ok_or_else(|| format!("cannot locate model dir from {}", types_dir.display()))?;

        for cat in &categories {
            let module = category_module(cat);
            let raw = self.emit_types_category(version, cat);
            let infos = self.all_response_infos(cat);
            let generated = crate::into_model::generate_category(version, cat, &infos, &model_dir);
            if generated.has_roots {
                let dir = types_dir.join(&module);
                fs::create_dir_all(&dir).map_err(|e| format!("mkdir {}: {e}", dir.display()))?;
                write_file(&dir.join("mod.rs"), &inject_into_decl(&raw, &generated.error_names))?;
                write_file(&dir.join("into.rs"), &generated.source)?;
            } else {
                write_file(&types_dir.join(format!("{module}.rs")), &raw)?;
            }
            write_file(
                &client_dir.join(format!("{module}.rs")),
                &self.emit_client_category(version, cat),
            )?;
        }
        Ok(())
    }

    /// Every response type in `category` as the simplified `RawTypeInfo` the `into_model` generator
    /// consumes (name + full Rust type per field). The generator seeds a root from every response
    /// type and walks nested types from those, so it needs the whole set.
    fn all_response_infos(&self, category: &str) -> Vec<crate::into_model::RawTypeInfo> {
        let mut out = Vec::new();
        for (c, origin, gt) in &self.types {
            if c != category || *origin != Origin::Response || gt.body.is_empty() {
                continue;
            }
            let shape = match &gt.ir {
                TypeIr::Struct(fields) => crate::into_model::RawShape::Struct(
                    fields
                        .iter()
                        .map(|f| crate::into_model::RawField {
                            name: f.rust_name.clone(),
                            ty: if f.optional {
                                format!("Option<{}>", f.rust_type)
                            } else {
                                f.rust_type.clone()
                            },
                        })
                        .collect(),
                ),
                TypeIr::Newtype(inner) => crate::into_model::RawShape::Newtype(inner.clone()),
                TypeIr::Enum(variants) => crate::into_model::RawShape::Enum(
                    variants.iter().map(|(v, inner)| (v.clone(), inner.clone())).collect(),
                ),
                _ => continue,
            };
            out.push(crate::into_model::RawTypeInfo { raw_name: gt.name.clone(), shape });
        }
        out.sort_by(|a, b| a.raw_name.cmp(&b.raw_name));
        out
    }

    /// Distinct help categories present across all methods, sorted.
    fn categories(&self) -> Vec<String> {
        let mut set: BTreeSet<String> = BTreeSet::new();
        for m in &self.methods {
            set.insert(m.category.clone());
        }
        set.into_iter().collect()
    }

    /// Emit one category's response types into a `corepc-types` module file.
    fn emit_types_category(&self, version: &str, category: &str) -> String {
        let mut s = String::with_capacity(32 * 1024);
        s.push_str(&format!(
            "// SPDX-License-Identifier: CC0-1.0\n\n\
             //! Auto-generated types for Bitcoin Core `{version}` - {category}.\n//!\n\
             //! Produced by `codegen`. Do not edit by hand, re-run\n\
             //! `just codegen` to regenerate. Holds the RPC return types for this section; the\n\
             //! `*Options` request structs live with the call surface in `corepc-client`.\n\n\
             #![allow(non_camel_case_types, non_snake_case, clippy::large_enum_variant)]\n\n\
             use serde::{{Deserialize, Serialize}};\n\n"
        ));
        let mut sorted: Vec<&GenType> = self
            .types
            .iter()
            .filter(|(c, origin, gt)| {
                c == category && *origin == Origin::Response && !gt.body.is_empty()
            })
            .map(|(_, _, gt)| gt)
            .collect();
        sorted.sort_by(|a, b| a.name.cmp(&b.name));
        for ty in sorted {
            s.push_str(&ty.body);
            s.push('\n');
        }
        s
    }

    /// Emit one category's call surface into a `corepc-client` module file.
    fn emit_client_category(&self, version: &str, category: &str) -> String {
        let mut ms: Vec<&MethodOut> =
            self.methods.iter().filter(|m| m.category == category).collect();
        ms.sort_by(|a, b| a.method_name.cmp(&b.method_name));

        let imports = self.client_imports(&ms);
        let types_use = if imports.is_empty() {
            String::new()
        } else if imports.len() == 1 {
            format!("use types::v{version}::generated::{};\n\n", imports[0])
        } else {
            let list = imports.iter().map(|n| format!("    {n},")).collect::<Vec<_>>().join("\n");
            format!("use types::v{version}::generated::{{\n{list}\n}};\n\n")
        };

        let mut s = String::with_capacity(32 * 1024);
        s.push_str(&format!(
            "// SPDX-License-Identifier: CC0-1.0\n\n\
             //! Auto-generated method wrappers for Bitcoin Core `{version}` - {category}.\n//!\n\
             //! Produced by `codegen`. Do not edit by hand, re-run\n\
             //! `just codegen` to regenerate. Defines the `*Options` request structs these methods\n\
             //! consume; the response types live in the `corepc-types` crate\n\
             //! (`types::v{version}::generated`).\n\n\
             #![allow(unused_imports, clippy::needless_pass_by_value, clippy::too_many_arguments)]\n\n\
             use serde::{{Deserialize, Serialize}};\n\
             use serde_json::json;\n\n\
             {types_use}\
             use crate::client_async::error::Result;\n\
             use crate::client_async::Client;\n\n"
        ));

        let mut params: Vec<&GenType> = self
            .types
            .iter()
            .filter(|(c, origin, gt)| {
                c == category && *origin == Origin::Param && !gt.body.is_empty()
            })
            .map(|(_, _, gt)| gt)
            .collect();
        params.sort_by(|a, b| a.name.cmp(&b.name));
        for ty in params {
            s.push_str(&ty.body.replace(
                "\n#[cfg_attr(feature = \"serde-deny-unknown-fields\", serde(deny_unknown_fields))]",
                "",
            ));
            s.push('\n');
        }

        let mut opt_ms: Vec<&&MethodOut> = ms.iter().filter(|m| m.has_optional()).collect();
        opt_ms.sort_by(|a, b| a.method_name.cmp(&b.method_name));
        for m in &opt_ms {
            s.push_str(&emit_options_struct(m));
            s.push('\n');
        }

        s.push_str("impl Client {\n");
        for m in &ms {
            s.push_str(&emit_method(m));
            s.push('\n');
        }
        s.push_str("}\n");
        s
    }

    /// Sorted deduplicated list of response-type names a category's client file needs to import.
    fn client_imports(&self, ms: &[&MethodOut]) -> Vec<String> {
        let response: BTreeSet<&str> = self
            .types
            .iter()
            .filter(|(_, origin, _)| *origin == Origin::Response)
            .map(|(_, _, gt)| gt.name.as_str())
            .collect();
        let mut needed: BTreeSet<String> = BTreeSet::new();
        for m in ms {
            if m.verbose_variants.is_empty() {
                if response.contains(m.return_type.as_str()) {
                    needed.insert(m.return_type.clone());
                }
            } else {
                for v in &m.verbose_variants {
                    if response.contains(v.type_name.as_str()) {
                        needed.insert(v.type_name.clone());
                    }
                }
            }
        }
        needed.into_iter().collect()
    }
}

pub(crate) fn emit_method(m: &MethodOut) -> String {
    if !m.verbose_variants.is_empty() {
        return emit_verbose_methods(m);
    }
    let mut out = String::new();
    let req: Vec<&ParamOut> = m.params.iter().filter(|p| p.required).collect();
    let opt: Vec<&ParamOut> = m.params.iter().filter(|p| !p.required).collect();

    let req_args = req
        .iter()
        .map(|p| format!("{}: {}", p.rust_name, p.rust_type))
        .collect::<Vec<_>>()
        .join(", ");
    let bare_sig =
        if req_args.is_empty() { "&self".to_owned() } else { format!("&self, {req_args}") };

    out.push_str(&doc_block(
        &[&format!("`{}` with required arguments only.", m.method_name), "", &m.description],
        "    ",
    ));
    out.push_str(&format!(
        "    pub async fn {}({bare_sig}) -> Result<{}> {{\n",
        m.snake, m.return_type
    ));
    out.push_str(&format!(
        "        self.call_raw(\"{}\", {}).await\n    }}\n",
        m.method_name,
        params_array_required_only(&m.params)
    ));

    if !opt.is_empty() {
        let opts_name = m.options_struct_name();
        let with_args = if req_args.is_empty() {
            format!("&self, opts: {opts_name}")
        } else {
            format!("&self, {req_args}, opts: {opts_name}")
        };
        out.push('\n');
        out.push_str(&doc_block(
            &[
                &format!("`{}` with all optional arguments via [`{opts_name}`].", m.method_name),
                "",
                &m.description,
            ],
            "    ",
        ));
        out.push_str(&format!(
            "    pub async fn {}_with({with_args}) -> Result<{}> {{\n",
            m.snake, m.return_type
        ));
        out.push_str(&format!(
            "        self.call_raw(\"{}\", {}).await\n    }}\n",
            m.method_name,
            params_array_with_opts(&m.params, m.object_options)
        ));
    }

    out
}

/// Emit, per verbosity level, a required-only method and (when the method has non-selector
/// optionals) a `_with` companion taking the `*Options` struct. The selector is hardcoded in each.
fn emit_verbose_methods(m: &MethodOut) -> String {
    let req: Vec<&ParamOut> = m.params.iter().filter(|p| p.required).collect();
    let has_opts = m.has_optional();
    let opts_name = m.options_struct_name();
    let selector_slot = m.selector_idx.min(m.params.len());

    let req_args = req
        .iter()
        .map(|p| format!("{}: {}", p.rust_name, p.rust_type))
        .collect::<Vec<_>>()
        .join(", ");
    let prefix =
        if req_args.is_empty() { "&self".to_owned() } else { format!("&self, {req_args}") };

    // Positional args in spec order with the selector reinserted at its real index. `with_opts`
    // selects whether optionals read from the `opts` struct or are sent as `null`.
    let call_array = |selector: &str, with_opts: bool| {
        let mut items: Vec<String> = m
            .params
            .iter()
            .map(|p| {
                if p.required {
                    format!("json!({})", p.rust_name)
                } else if with_opts {
                    format!("json!(opts.{})", p.rust_name)
                } else {
                    "json!(null)".to_owned()
                }
            })
            .collect();
        items.insert(selector_slot, format!("json!({selector})"));
        format!("&[{}]", items.join(", "))
    };

    let mut out = String::new();
    for v in &m.verbose_variants {
        let base = format!("{}_verbose_{}", m.snake, v.word);
        let summary =
            format!("`{}` with the result selected for verbosity `{}`.", m.method_name, v.selector);

        out.push_str(&doc_block(&[&summary, "", &m.description], "    "));
        out.push_str(&format!("    pub async fn {base}({prefix}) -> Result<{}> {{\n", v.type_name));
        out.push_str(&format!(
            "        self.call_raw(\"{}\", {}).await\n    }}\n\n",
            m.method_name,
            call_array(&v.selector, false)
        ));

        if has_opts {
            let with_args = format!("{prefix}, opts: {opts_name}");
            out.push_str(&doc_block(
                &[
                    &format!("{summary} With all optional arguments via [`{opts_name}`]."),
                    "",
                    &m.description,
                ],
                "    ",
            ));
            out.push_str(&format!(
                "    pub async fn {base}_with({with_args}) -> Result<{}> {{\n",
                v.type_name
            ));
            out.push_str(&format!(
                "        self.call_raw(\"{}\", {}).await\n    }}\n\n",
                m.method_name,
                call_array(&v.selector, true)
            ));
        }
    }
    out
}

/// Positional arguments for the required-only method (optionals before the last required become `null`).
fn params_array_required_only(params: &[ParamOut]) -> String {
    let Some(last_required) = params.iter().rposition(|p| p.required) else {
        return "&[(); 0] as &[()]".to_owned();
    };
    let items: Vec<String> =
        params[..=last_required]
            .iter()
            .map(|p| {
                if p.required {
                    format!("json!({})", p.rust_name)
                } else {
                    "json!(null)".to_owned()
                }
            })
            .collect();
    format!("&[{}]", items.join(", "))
}

/// Positional arguments for the `_with` method (optionals from the options struct, in spec order).
fn params_array_with_opts(params: &[ParamOut], object_options: bool) -> String {
    if object_options {
        let mut items: Vec<String> = params
            .iter()
            .filter(|p| p.required)
            .map(|p| format!("json!({})", p.rust_name))
            .collect();
        items.push("json!(opts)".to_owned());
        return format!("&[{}]", items.join(", "));
    }
    let items: Vec<String> = params
        .iter()
        .map(|p| {
            if p.required {
                format!("json!({})", p.rust_name)
            } else {
                format!("json!(opts.{})", p.rust_name)
            }
        })
        .collect();
    if items.is_empty() {
        return "&[(); 0] as &[()]".to_owned();
    }
    format!("&[{}]", items.join(", "))
}

/// Emit the `*Options` struct for a method's `_with` variant.
fn emit_options_struct(m: &MethodOut) -> String {
    let name = m.options_struct_name();
    let mut s = String::new();
    s.push_str(&doc_block(
        &[&format!(
            "Optional parameters for the `{}` JSON-RPC method (consumed by `Client::{}_with`).",
            m.method_name, m.snake
        )],
        "",
    ));
    s.push_str("#[derive(Clone, Debug, Default, serde::Serialize)]\n");
    if !m.object_options {
        s.push_str("#[serde(rename_all = \"camelCase\")]\n");
    }
    s.push_str(&format!("pub struct {name} {{\n"));
    for p in m.params.iter().filter(|p| !p.required) {
        let default_line = p
            .default
            .as_ref()
            .map(|d| format!("Default in Bitcoin Core: `{}`.", format_default_value(d)));
        let mut lines: Vec<&str> = vec![p.description.as_str()];
        if let Some(line) = default_line.as_deref() {
            lines.push("");
            lines.push(line);
        }
        s.push_str(&doc_block(&lines, "    "));
        if m.object_options {
            s.push_str("    #[serde(skip_serializing_if = \"Option::is_none\"");
            if p.rust_name != p.wire_name {
                s.push_str(&format!(", rename = \"{}\"", p.wire_name));
            }
            s.push_str(")]\n");
        }
        s.push_str(&format!("    pub {}: Option<{}>,\n", p.rust_name, p.rust_type));
    }
    s.push_str("}\n");
    s
}

/// Render a JSON default value as a human-readable string.
fn format_default_value(v: &Value) -> String {
    match v {
        Value::String(s) => format!("'{s}'"),
        other => other.to_string(),
    }
}

/// Extract the best available description for a parameter.
pub(crate) fn param_description(p: &Param) -> String {
    if !p.schema.description.as_deref().unwrap_or("").is_empty() {
        return p.schema.description.clone().unwrap_or_default().trim().to_owned();
    }
    p.description.trim().to_owned()
}

/// Map a parameter schema to a Rust type for the method signature.

fn doc_block(lines: &[&str], indent: &str) -> String {
    let mut filtered: Vec<&str> = lines.to_vec();
    while filtered.last().map(|s| s.is_empty()).unwrap_or(false) {
        filtered.pop();
    }
    let mut out = String::new();
    for line in filtered {
        if line.is_empty() {
            out.push_str(indent);
            out.push_str("///\n");
            continue;
        }
        for sub in line.lines() {
            out.push_str(indent);
            out.push_str("/// ");
            out.push_str(&esc_doc(sub));
            out.push('\n');
        }
    }
    out
}

/// Format a doc string as `/// ...` lines, returning an empty string when there is nothing.
pub(crate) fn fmt_doc(doc: Option<&str>) -> String {
    let raw = match doc {
        None | Some("") => return String::new(),
        Some(d) => d,
    };
    let mut s = String::with_capacity(raw.len() + 16);
    if raw.starts_with("///") {
        for (i, line) in raw.lines().enumerate() {
            if i > 0 {
                s.push('\n');
            }
            if line.starts_with("///") {
                s.push_str(line);
            } else {
                s.push_str("/// ");
                s.push_str(line);
            }
        }
        s.push('\n');
        return s;
    }
    for (i, line) in raw.lines().enumerate() {
        if i > 0 {
            s.push('\n');
        }
        s.push_str("/// ");
        s.push_str(&esc_doc(line));
    }
    s.push('\n');
    s
}

/// Build the `/// Result of ...` doc-comment for a return type.
pub(crate) fn method_doc(method: &Method) -> String {
    let summary_lines: Vec<String> = method.description.lines().map(esc_doc).collect();
    let body = summary_lines.join("\n/// > ");
    format!(
        "/// Result of the JSON-RPC method `{}`.\n///\n/// > {}\n/// >\n/// > {}",
        method.name, method.name, body
    )
}

/// Escape characters that have special meaning in rustdoc/Markdown to avoid `-D warnings` failures.
pub(crate) fn esc_doc(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 8);
    let mut rest = s;
    let mut in_token = false;
    while !rest.is_empty() {
        if rest.starts_with("<http://") || rest.starts_with("<https://") {
            if let Some(close) = rest.find('>') {
                out.push_str(&rest[..=close]);
                rest = &rest[close + 1..];
                continue;
            }
        }
        if rest.starts_with("[`") {
            if let Some(close) = rest.find("`]") {
                out.push_str(&rest[..close + 2]);
                rest = &rest[close + 2..];
                continue;
            }
        }
        if rest.starts_with("http://") || rest.starts_with("https://") {
            let span = rest.find(char::is_whitespace).unwrap_or(rest.len());
            let mut url_end = span;
            while url_end > 0
                && matches!(
                    rest.as_bytes()[url_end - 1],
                    b'.' | b',' | b';' | b':' | b')' | b'?' | b'!'
                )
            {
                url_end -= 1;
            }
            out.push('<');
            out.push_str(&rest[..url_end]);
            out.push('>');
            rest = &rest[url_end..];
            continue;
        }
        let ch = rest.chars().next().expect("rest is non-empty");
        match ch {
            '<' if !in_token => {
                in_token = true;
                out.push_str("\\<");
            }
            '>' if in_token => {
                in_token = false;
                out.push_str("\\>");
            }
            '[' => out.push_str("\\["),
            ']' => out.push_str("\\]"),
            _ => out.push(ch),
        }
        rest = &rest[ch.len_utf8()..];
    }
    out
}

/// Write `content` to `path`, printing a clickable absolute path on success.
fn write_file(path: &Path, content: &str) -> Result<(), String> {
    fs::write(path, content).map_err(|e| format!("write {}: {e}", path.display()))?;
    let shown = fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
    println!("  wrote {}:1", shown.display());
    Ok(())
}

/// Map a Bitcoin Core help category to its Rust module name.
fn category_module(category: &str) -> String {
    match category {
        "rawtransactions" => "raw_transactions".to_owned(),
        other => other.to_owned(),
    }
}

/// Emit `mod.rs` for `corepc-types/generated/`.
fn emit_types_mod_rs(version: &str, categories: &[String]) -> String {
    let mut s = String::new();
    s.push_str(&format!(
        "// SPDX-License-Identifier: CC0-1.0\n\n\
         //! Auto-generated return types for Bitcoin Core `{version}`, split by API section.\n//!\n\
         //! Generated by `codegen`. Do not edit any file in this module by hand.\n\
         //! Re-run `just codegen` from the workspace root to regenerate.\n\n"
    ));
    for cat in categories {
        s.push_str(&format!("pub mod {};\n", category_module(cat)));
    }
    s.push_str(
        "\n// Hand-maintained override shims for conversions the canonical `crate::model` types\n\
         // get wrong; emitted by codegen from a fixed table. See `corepc_bugs_backlog.md`.\n\
         pub mod compatibility;\n\n",
    );
    for cat in categories {
        s.push_str(&format!("pub use self::{}::*;\n", category_module(cat)));
    }
    s
}

/// Insert `mod into;` and the category's error-type re-exports into a raw category file, just
/// before its serde import, turning it into the `mod.rs` of a directory module with an `into.rs`.
fn inject_into_decl(raw: &str, errs: &[String]) -> String {
    let reexport = if errs.is_empty() {
        String::new()
    } else {
        format!("pub use self::into::{{{}}};\n\n", errs.join(", "))
    };
    let anchor = "use serde::{Deserialize, Serialize};";
    match raw.find(anchor) {
        Some(idx) => format!("{}mod into;\n\n{}{}", &raw[..idx], reexport, &raw[idx..]),
        None => format!("mod into;\n\n{}{}", reexport, raw),
    }
}

/// Emit `mod.rs` for `corepc-client/v{N}/`.
fn emit_client_mod_rs(version: &str, categories: &[String], has_blocking: bool) -> String {
    let mut s = String::new();
    s.push_str(&format!(
        "// SPDX-License-Identifier: CC0-1.0\n\n\
         //! Auto-generated client bindings for Bitcoin Core `{version}`, split by API section.\n//!\n\
         //! Generated by `codegen`. Do not edit any file in this module by hand.\n\
         //! Re-run `just codegen` from the workspace root to regenerate.\n//!\n\
         //! Each module adds an `impl Client` block of method wrappers plus the `*Options` request\n\
         //! structs for that section. The response types live in the `corepc-types` crate\n\
         //! (`types::v{version}::generated`).\n\n\
         #![allow(unused_imports)]\n\n"
    ));
    for cat in categories {
        s.push_str(&format!("pub mod {};\n", category_module(cat)));
    }
    if has_blocking {
        s.push_str(
            "\n// The blocking, sync-API facade over this version's async client. Reuses the sync\n\
             // client's method macros, so the integration tests run unchanged against the async\n\
             // transport.\n\
             #[cfg(feature = \"blocking\")]\n\
             pub mod blocking;\n",
        );
    }
    s.push('\n');
    for cat in categories {
        s.push_str(&format!("pub use self::{}::*;\n", category_module(cat)));
    }
    s
}
