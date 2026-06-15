// SPDX-License-Identifier: CC0-1.0

//! Translate a parsed OpenRPC [`Spec`] into Rust source for the four output files.

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use serde_json::Value;

use crate::names::{method_to_pascal, method_to_snake, to_pascal, to_rust_field};
use crate::spec::{AdditionalProperties, Method, Param, Schema, SchemaType, Spec};

const DERIVES: &str = "#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]\n\
     #[cfg_attr(feature = \"serde-deny-unknown-fields\", serde(deny_unknown_fields))]";

const MODEL_DERIVES: &str = "#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]";

/// PascalCase names that collide with std/prelude items; rewritten to avoid the collision.
const RESERVED_TYPE_NAMES: &[(&str, &str)] = &[
    ("Send", "SendResult"),
    ("Sync", "SyncResult"),
    ("Drop", "DropResult"),
    ("Box", "BoxResult"),
    ("Vec", "VecResult"),
    ("Option", "OptionResult"),
    ("Result", "ResultResponse"),
];

/// Field wire names Core emits as a C++ `double` (always fractional, never an amount).
const FLOAT_FIELDS: &[&str] = &[
    "difficulty",
    "verificationprogress",
    "networkhashps",
    "txrate",
    "pingtime",
    "minping",
    "pingwait",
];

/// Generic names that also appear as signed ints elsewhere are handled per-parent in [`classify_number`].
const U64_FIELDS: &[&str] = &[
    "addr_processed",
    "addr_rate_limited",
    "ancestorcount",
    "ancestorsize",
    "bogosize",
    "bytes",
    "bytes_left_in_cycle",
    "bytesrecv",
    "bytesrecv_per_msg",
    "bytessent",
    "bytessent_per_msg",
    "coins_db_cache_bytes",
    "coins_loaded",
    "coins_tip_cache_bytes",
    "coins_written",
    "connections",
    "connections_in",
    "connections_out",
    "descendantcount",
    "descendantsize",
    "disk_size",
    "estimated_vsize",
    "keypoolsize",
    "keypoolsize_hd_internal",
    "last_inv_sequence",
    "mempool_sequence",
    "nchaintx",
    "pooledtx",
    "prune_target_size",
    "services",
    "size_on_disk",
    "strippedsize",
    "subtype",
    "target",
    "totalbytesrecv",
    "totalbytessent",
    "transactions",
    "txcount",
    "txouts",
    "txs",
    "unbroadcastcount",
    "usage",
    "window_tx_count",
];

/// Amount fields Core omits the unit from; each is a `CAmount` rendered as fractional BTC.
const AMOUNT_STRING_FIELDS: &[&str] = &[
    "amount",
    "balance_change",
    "fee",
    "minrelaytxfee",
    "origfee",
    "total_amount",
    "total_unspendable_amount",
];

/// Param names that are always integers despite the spec declaring them as `type: number`.
pub static INTEGER_PARAM_NAMES: &[&str] = &[
    "height",
    "verbosity",
    "verbose",
    "minconf",
    "maxconf",
    "conf_target",
    "nblocks",
    "blocks",
    "count",
    "num_blocks",
    "n",
    "version",
    "locktime",
    "port",
    "timeout",
    "millis",
    "block_timeout",
    "node_id",
    "rescan_height",
    "start_height",
    "stop_height",
    "depth",
    "index",
    "nout",
    "vout",
    "skip",
    "nodeid",
    "id",
    "uid",
    "nrequired",
    "fee_delta",
];

/// One emitted Rust definition; `nested` holds helper types it depends on.
#[derive(Debug)]
struct GenType {
    name: String,
    body: String,
    nested: Vec<GenType>,
    ir: TypeIr,
}

/// Response types live in `corepc-types` with model counterparts; Param types live in `corepc-client`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Origin {
    Response,
    Param,
}

/// Shape of a generated raw type, used to emit the model layer without re-walking the spec.
#[derive(Debug, Clone)]
enum TypeIr {
    /// `pub struct Name { fields }`.
    Struct(Vec<FieldIr>),
    /// `pub struct Name(pub inner);`, the inner Rust type string.
    Newtype(String),
    /// `pub enum Name { Variant(inner), ... }`, `(variant ident, inner Rust type)` pairs.
    Enum(Vec<(String, String)>),
    /// `pub type Name = Target;`.
    Alias(String),
}

/// One field of a generated struct captured for model emission; `rust_type` is pre-`Option` wrap.
#[derive(Debug, Clone)]
struct FieldIr {
    rust_name: String,
    rust_type: String,
    optional: bool,
    doc: Option<String>,
}

/// Output bundle: every type, method, and options struct, ready to be written to disk.
pub struct Modules {
    types: Vec<(String, Origin, GenType)>,
    methods: Vec<MethodOut>,
    type_names: BTreeSet<String>,
}

#[derive(Debug)]
struct MethodOut {
    method_name: String, // wire name (`getblockheader`)
    snake: String,       // Rust fn name (`get_block_header`)
    pascal: String,      // Rust return-type ident (`GetBlockHeader`)
    description: String,
    return_type: String,
    params: Vec<ParamOut>,
    category: String,
    /// True when the method's sole optional parameter is a JSON object whose properties have been
    /// flattened into the `*Options` struct. The struct is then sent as one positional argument
    /// rather than its fields being spread.
    object_options: bool,
    /// Non-empty when the result is a verbose-selected union: instead of one method, emit one
    /// method per variant, each returning a concrete type and hardcoding the selector argument.
    verbose_variants: Vec<VerboseVariant>,
    /// Positional index of the selector within the original spec params, set only for verbose
    /// methods. The selector is dropped from `params`, so the emitter reinserts its hardcoded
    /// literal at this index.
    selector_idx: usize,
}

#[derive(Debug)]
struct ParamOut {
    rust_name: String,
    wire_name: String, // JSON key, for serde rename when an options object is serialised whole
    rust_type: String,
    description: String,
    required: bool,
    default: Option<Value>,
}

impl Modules {
    /// Number of generated types across all categories.
    pub fn types_count(&self) -> usize { self.types.len() }
    /// Number of generated RPC methods.
    pub fn methods_count(&self) -> usize { self.methods.len() }
    /// Number of methods that emit an `*Options` struct.
    pub fn option_count(&self) -> usize { self.methods.iter().filter(|m| m.has_optional()).count() }

    /// Write all generated files across both crates, split into per-category modules.
    pub fn write(&self, types_dir: &Path, client_dir: &Path, version: &str) -> Result<(), String> {
        let categories = self.categories();
        let fallible = self.fallible_types();
        let model_dir = types_dir.join("model");
        fs::create_dir_all(&model_dir)
            .map_err(|e| format!("mkdir {}: {e}", model_dir.display()))?;

        write_file(&types_dir.join("mod.rs"), &emit_types_mod_rs(version, &categories))?;
        write_file(&model_dir.join("mod.rs"), &emit_model_mod_rs(version, &categories))?;
        write_file(&client_dir.join("mod.rs"), &emit_client_mod_rs(version, &categories))?;

        for cat in &categories {
            let module = category_module(cat);
            write_file(
                &types_dir.join(format!("{module}.rs")),
                &self.emit_types_category(version, cat),
            )?;
            write_file(
                &model_dir.join(format!("{module}.rs")),
                &self.emit_model_category(version, cat, &fallible),
            )?;
            write_file(
                &client_dir.join(format!("{module}.rs")),
                &self.emit_client_category(version, cat),
            )?;
        }
        Ok(())
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

/// Result of classifying a raw field type for the model layer.
struct Built {
    model_type: String,
    expr: String,
    converts: bool,
    err: Option<String>,
}

/// Return the rust-bitcoin hash type for a field name, if unambiguous (`txid` -> `Txid`, etc.).
fn hash_model(name: &str) -> Option<&'static str> {
    let n: String = name.chars().filter(|c| *c != '_').flat_map(char::to_lowercase).collect();
    match n.as_str() {
        "txid" | "txids" => Some("Txid"),
        "wtxid" | "wtxids" => Some("Wtxid"),
        "blockhash" | "bestblockhash" | "previousblockhash" | "nextblockhash" => Some("BlockHash"),
        _ => None,
    }
}

/// Strip a generic wrapper: `strip_wrap("Option<X>", "Option<")` -> `"X"`.
fn strip_wrap<'a>(ty: &'a str, open: &str) -> Option<&'a str> {
    ty.strip_prefix(open)?.strip_suffix('>')
}

/// If `expr` is `PATH(var)`, return `PATH` so callers can use `.map(PATH)` instead of a closure.
fn fn_path_applied_to<'a>(expr: &'a str, var: &str) -> Option<&'a str> {
    let inner = expr.strip_suffix(&format!("({var})"))?;
    (!inner.is_empty() && !inner.contains(|c: char| !c.is_alphanumeric() && c != '_' && c != ':'))
        .then_some(inner)
}

/// Whether a field's documented unit marks it as a BTC amount (fee rates are excluded).
fn is_btc_amount(desc: Option<&str>) -> bool {
    desc.is_some_and(|d| d.contains("BTC") && !d.contains("/kvB") && !d.contains("/kB"))
}

/// Unambiguous address field names (the value parses to `Address<NetworkUnchecked>`).
fn is_address(name: &str) -> bool {
    let n: String = name.chars().filter(|c| *c != '_').flat_map(char::to_lowercase).collect();
    matches!(n.as_str(), "address" | "addresses")
}

/// Map a raw field type to its model type and conversion expression.
fn convert(
    raw: &str,
    access: &str,
    name: &str,
    desc: Option<&str>,
    type_names: &BTreeSet<String>,
    fallible: &BTreeSet<String>,
) -> Built {
    if let Some(inner) = strip_wrap(raw, "Option<") {
        let b = convert(inner, "v", name, desc, type_names, fallible);
        let model_type = format!("Option<{}>", b.model_type);
        if !b.converts {
            return Built { model_type, expr: access.to_owned(), converts: false, err: None };
        }
        let mapped = match fn_path_applied_to(&b.expr, "v") {
            Some(path) => format!("{access}.map({path})"),
            None => format!("{access}.map(|v| {})", b.expr),
        };
        let expr = match &b.err {
            Some(_) => format!("{mapped}.transpose()"),
            None => mapped,
        };
        return Built { model_type, expr, converts: true, err: b.err };
    }
    if let Some(inner) = strip_wrap(raw, "Vec<") {
        let b = convert(inner, "x", name, desc, type_names, fallible);
        let model_type = format!("Vec<{}>", b.model_type);
        if !b.converts {
            return Built { model_type, expr: access.to_owned(), converts: false, err: None };
        }
        let expr = match &b.err {
            Some(_) => {
                format!("{access}.into_iter().map(|x| {}).collect::<Result<Vec<_>, _>>()", b.expr)
            }
            None => format!("{access}.into_iter().map(|x| {}).collect()", b.expr),
        };
        return Built { model_type, expr, converts: true, err: b.err };
    }
    if let Some(value) = strip_wrap(raw, "std::collections::BTreeMap<String, ") {
        let b = convert(value, "v", "", None, type_names, fallible);
        if !b.converts {
            return Built {
                model_type: raw.to_owned(),
                expr: access.to_owned(),
                converts: false,
                err: None,
            };
        }
        let model_type = format!("std::collections::BTreeMap<String, {}>", b.model_type);
        let expr = match &b.err {
            Some(_) => format!(
                "{access}.into_iter().map(|(k, v)| {}.map(|m| (k, m))).collect::<Result<std::collections::BTreeMap<_, _>, _>>()",
                b.expr
            ),
            None => format!("{access}.into_iter().map(|(k, v)| (k, {})).collect()", b.expr),
        };
        return Built { model_type, expr, converts: true, err: b.err };
    }
    if raw == "f64" && is_btc_amount(desc) {
        return Built {
            model_type: "Amount".to_owned(),
            expr: format!("Amount::from_btc({access})"),
            converts: true,
            err: Some("bitcoin::amount::ParseAmountError".to_owned()),
        };
    }
    if raw == "String" {
        if is_address(name) {
            return Built {
                model_type: "Address<NetworkUnchecked>".to_owned(),
                expr: format!("{access}.parse::<Address<NetworkUnchecked>>()"),
                converts: true,
                err: Some("bitcoin::address::ParseError".to_owned()),
            };
        }
        if let Some(h) = hash_model(name) {
            return Built {
                model_type: h.to_owned(),
                expr: format!("{access}.parse::<{h}>()"),
                converts: true,
                err: Some("bitcoin::hex::HexToArrayError".to_owned()),
            };
        }
    }
    if type_names.contains(raw) {
        let err = fallible.contains(raw).then(|| format!("{raw}Error"));
        return Built {
            model_type: raw.to_owned(),
            expr: format!("{access}.into_model()"),
            converts: true,
            err,
        };
    }
    Built { model_type: raw.to_owned(), expr: access.to_owned(), converts: false, err: None }
}

/// Whether converting a raw type to its model type is fallible.
fn conv_fallible(
    raw: &str,
    name: &str,
    desc: Option<&str>,
    by_name: &std::collections::BTreeMap<String, TypeIr>,
    memo: &mut std::collections::BTreeMap<String, bool>,
    stack: &mut BTreeSet<String>,
) -> bool {
    if let Some(inner) = strip_wrap(raw, "Option<") {
        return conv_fallible(inner, name, desc, by_name, memo, stack);
    }
    if let Some(inner) = strip_wrap(raw, "Vec<") {
        return conv_fallible(inner, name, desc, by_name, memo, stack);
    }
    if let Some(value) = strip_wrap(raw, "std::collections::BTreeMap<String, ") {
        return conv_fallible(value, "", None, by_name, memo, stack);
    }
    if raw == "f64" && is_btc_amount(desc) {
        return true;
    }
    if raw == "String" {
        return is_address(name) || hash_model(name).is_some();
    }
    if by_name.contains_key(raw) {
        return type_fallible(raw, by_name, memo, stack);
    }
    false
}

/// Whether a generated type's `into_model` is fallible (memoized, cycle-safe).
fn type_fallible(
    name: &str,
    by_name: &std::collections::BTreeMap<String, TypeIr>,
    memo: &mut std::collections::BTreeMap<String, bool>,
    stack: &mut BTreeSet<String>,
) -> bool {
    if let Some(&v) = memo.get(name) {
        return v;
    }
    if stack.contains(name) {
        return false;
    }
    stack.insert(name.to_owned());
    let res = match by_name.get(name) {
        None => false,
        Some(TypeIr::Struct(fields)) => fields.clone().iter().any(|f| {
            let t =
                if f.optional { format!("Option<{}>", f.rust_type) } else { f.rust_type.clone() };
            conv_fallible(&t, &f.rust_name, f.doc.as_deref(), by_name, memo, stack)
        }),
        Some(TypeIr::Newtype(inner)) =>
            conv_fallible(&inner.clone(), "", None, by_name, memo, stack),
        Some(TypeIr::Enum(vs)) =>
            vs.clone().iter().any(|(_, t)| conv_fallible(t, "", None, by_name, memo, stack)),
        Some(TypeIr::Alias(t)) => type_fallible(&t.clone(), by_name, memo, stack),
    };
    stack.remove(name);
    memo.insert(name.to_owned(), res);
    res
}

impl Modules {
    /// Map of every generated type name to its IR (response types only, first real body wins).
    fn ir_by_name(&self) -> std::collections::BTreeMap<String, TypeIr> {
        let mut m = std::collections::BTreeMap::new();
        for (_, origin, gt) in &self.types {
            if *origin == Origin::Response && !gt.body.is_empty() {
                m.entry(gt.name.clone()).or_insert_with(|| gt.ir.clone());
            }
        }
        m
    }

    /// Set of generated types whose `into_model` is fallible.
    fn fallible_types(&self) -> BTreeSet<String> {
        let by_name = self.ir_by_name();
        let mut memo = std::collections::BTreeMap::new();
        let mut stack = BTreeSet::new();
        for name in by_name.keys() {
            type_fallible(name, &by_name, &mut memo, &mut stack);
        }
        memo.into_iter().filter(|(_, v)| *v).map(|(k, _)| k).collect()
    }

    /// Emit one category's model module.
    fn emit_model_category(
        &self,
        version: &str,
        category: &str,
        fallible: &BTreeSet<String>,
    ) -> String {
        let module = category_module(category);
        let mut s = String::with_capacity(32 * 1024);
        s.push_str(&format!(
            "// SPDX-License-Identifier: CC0-1.0\n\n\
             //! Auto-generated model types for Bitcoin Core `{version}` - {category}.\n//!\n\
             //! Produced by `codegen`. Do not edit by hand, re-run\n\
             //! `just codegen` to regenerate. These are the version-nonspecific, strongly typed\n\
             //! counterparts of the raw types, with `into_model` conversions and their error enums.\n\n\
             #![allow(non_camel_case_types, non_snake_case, missing_docs, unused_imports, clippy::large_enum_variant)]\n\n\
             use core::fmt;\n\n\
             use bitcoin::{{hex, Address, Amount, BlockHash, FeeRate, Transaction, Txid, Wtxid}};\n\
             use bitcoin::address::NetworkUnchecked;\n\
             use serde::{{Deserialize, Serialize}};\n\n\
             use crate::error::write_err;\n\
             use crate::v{version}::generated::{module} as raw;\n\
             use crate::v{version}::generated::model::{module} as model;\n\
             use super::*;\n\n"
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
        for gt in sorted {
            s.push_str(&emit_model_type(gt, &self.type_names, fallible));
        }
        s
    }
}

/// Emit a model struct definition plus its `into_model` impl.
fn emit_model_struct(
    name: &str,
    err_name: &str,
    fields: &[FieldIr],
    is_fallible: bool,
    type_names: &BTreeSet<String>,
    fallible: &BTreeSet<String>,
) -> (String, Vec<(String, String, String)>) {
    let mut used: BTreeSet<String> = BTreeSet::new();
    let mut model_fields = String::new();
    let mut ctor = String::new();
    let mut err_variants: Vec<(String, String, String)> = Vec::new();
    for f in fields {
        let full =
            if f.optional { format!("Option<{}>", f.rust_type) } else { f.rust_type.clone() };
        let b = convert(
            &full,
            &format!("self.{}", f.rust_name),
            &f.rust_name,
            f.doc.as_deref(),
            type_names,
            fallible,
        );
        if let Some(d) = &f.doc {
            model_fields.push_str(&format!("    /// {}\n", d.replace('\n', "\n    /// ")));
        }
        model_fields.push_str(&format!("    pub {}: {},\n", f.rust_name, b.model_type));
        match b.err {
            Some(inner) => {
                let variant = uniquify(&mut used, &to_pascal(&f.rust_name));
                ctor.push_str(&format!(
                    "            {}: {}.map_err({err_name}::{variant})?,\n",
                    f.rust_name, b.expr
                ));
                err_variants.push((variant, inner, f.rust_name.clone()));
            }
            None => ctor.push_str(&format!("            {}: {},\n", f.rust_name, b.expr)),
        }
    }
    let mut out = format!("{MODEL_DERIVES}\npub struct {name} {{\n{model_fields}}}\n\n");
    if is_fallible {
        out.push_str(&format!(
            "impl raw::{name} {{\n    /// Converts the raw type into the model type.\n    pub fn into_model(self) -> Result<model::{name}, {err_name}> {{\n        Ok(model::{name} {{\n{ctor}        }})\n    }}\n}}\n\n"
        ));
    } else {
        out.push_str(&format!(
            "impl raw::{name} {{\n    /// Converts the raw type into the model type.\n    pub fn into_model(self) -> model::{name} {{\n        model::{name} {{\n{ctor}        }}\n    }}\n}}\n\n"
        ));
    }
    (out, err_variants)
}

/// Emit a model newtype definition plus its `into_model` impl.
fn emit_model_newtype(
    name: &str,
    err_name: &str,
    inner: &str,
    type_names: &BTreeSet<String>,
    fallible: &BTreeSet<String>,
) -> (String, Vec<(String, String, String)>) {
    let b = convert(inner, "self.0", "", None, type_names, fallible);
    let mut out = format!("{MODEL_DERIVES}\npub struct {name}(pub {});\n\n", b.model_type);
    out.push_str(&format!(
        "impl std::ops::Deref for {name} {{\n    type Target = {};\n    fn deref(&self) -> &Self::Target {{ &self.0 }}\n}}\n\n",
        b.model_type
    ));
    let mut err_variants: Vec<(String, String, String)> = Vec::new();
    match b.err {
        Some(inner_err) => {
            out.push_str(&format!(
                "impl raw::{name} {{\n    /// Converts the raw type into the model type.\n    pub fn into_model(self) -> Result<model::{name}, {err_name}> {{\n        Ok(model::{name}({}.map_err({err_name}::Inner)?))\n    }}\n}}\n\n",
                b.expr
            ));
            err_variants.push(("Inner".to_owned(), inner_err, "inner value".to_owned()));
        }
        None => out.push_str(&format!(
            "impl raw::{name} {{\n    /// Converts the raw type into the model type.\n    pub fn into_model(self) -> model::{name} {{\n        model::{name}({})\n    }}\n}}\n\n",
            b.expr
        )),
    }
    (out, err_variants)
}

/// Emit a model enum definition plus its `into_model` impl.
fn emit_model_enum(
    name: &str,
    err_name: &str,
    variants: &[(String, String)],
    is_fallible: bool,
    type_names: &BTreeSet<String>,
    fallible: &BTreeSet<String>,
) -> (String, Vec<(String, String, String)>) {
    let mut model_arms = String::new();
    let mut match_arms = String::new();
    let mut err_variants: Vec<(String, String, String)> = Vec::new();
    for (variant, inner) in variants {
        let b = convert(inner, "x", "", None, type_names, fallible);
        model_arms.push_str(&format!("    {variant}({}),\n", b.model_type));
        match b.err {
            Some(inner_err) => {
                match_arms.push_str(&format!(
                    "            raw::{name}::{variant}(x) => model::{name}::{variant}({}.map_err({err_name}::{variant})?),\n",
                    b.expr
                ));
                err_variants.push((variant.clone(), inner_err, variant.clone()));
            }
            None => match_arms.push_str(&format!(
                "            raw::{name}::{variant}(x) => model::{name}::{variant}({}),\n",
                b.expr
            )),
        }
    }
    let mut out =
        format!("{MODEL_DERIVES}\n#[serde(untagged)]\npub enum {name} {{\n{model_arms}}}\n\n");
    if is_fallible {
        out.push_str(&format!(
            "impl raw::{name} {{\n    /// Converts the raw type into the model type.\n    pub fn into_model(self) -> Result<model::{name}, {err_name}> {{\n        Ok(match self {{\n{match_arms}        }})\n    }}\n}}\n\n"
        ));
    } else {
        out.push_str(&format!(
            "impl raw::{name} {{\n    /// Converts the raw type into the model type.\n    pub fn into_model(self) -> model::{name} {{\n        match self {{\n{match_arms}        }}\n    }}\n}}\n\n"
        ));
    }
    (out, err_variants)
}

/// Emit the model type definition, its `into_model` impl, and (when fallible) its error enum.
fn emit_model_type(
    gt: &GenType,
    type_names: &BTreeSet<String>,
    fallible: &BTreeSet<String>,
) -> String {
    let name = &gt.name;
    let is_fallible = fallible.contains(name);
    let err_name = format!("{name}Error");

    let (body, err_variants) = match &gt.ir {
        TypeIr::Alias(target) => return format!("pub type {name} = {target};\n\n"),
        TypeIr::Struct(fields) =>
            emit_model_struct(name, &err_name, fields, is_fallible, type_names, fallible),
        TypeIr::Newtype(inner) => emit_model_newtype(name, &err_name, inner, type_names, fallible),
        TypeIr::Enum(variants) =>
            emit_model_enum(name, &err_name, variants, is_fallible, type_names, fallible),
    };

    let mut out = body;
    if !err_variants.is_empty() {
        out.push_str(&emit_error_enum(&err_name, name, &err_variants));
    }
    out
}

/// Emit a conversion error enum with `Debug`, `Display`, and `std::error::Error` impls.
fn emit_error_enum(err_name: &str, ty: &str, variants: &[(String, String, String)]) -> String {
    let mut out = String::new();
    out.push_str(&format!("/// Error when converting a `{ty}` type into the model type.\n"));
    out.push_str("#[derive(Debug)]\n");
    out.push_str(&format!("pub enum {err_name} {{\n"));
    for (variant, inner, field) in variants {
        out.push_str(&format!("    /// Conversion of the `{field}` field failed.\n"));
        out.push_str(&format!("    {variant}({inner}),\n"));
    }
    out.push_str("}\n\n");

    out.push_str(&format!("impl fmt::Display for {err_name} {{\n    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {{\n        match *self {{\n"));
    for (variant, _, field) in variants {
        out.push_str(&format!(
            "            Self::{variant}(ref e) => write_err!(f, \"conversion of the `{field}` field failed\"; e),\n"
        ));
    }
    out.push_str("        }\n    }\n}\n\n");

    out.push_str(&format!("#[cfg(feature = \"std\")]\nimpl std::error::Error for {err_name} {{\n    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {{\n        match *self {{\n"));
    for (variant, _, _) in variants {
        out.push_str(&format!("            Self::{variant}(ref e) => Some(e),\n"));
    }
    out.push_str("        }\n    }\n}\n\n");
    out
}

impl MethodOut {
    /// Whether this method has at least one optional parameter.
    fn has_optional(&self) -> bool { self.params.iter().any(|p| !p.required) }
    /// Name of the `*Options` struct for the `_with` variant of this method.
    fn options_struct_name(&self) -> String { format!("{}Options", self.pascal) }
}

/// Map a PascalCase method name to its generated type name, avoiding std/prelude collisions.
fn safe_type_name(pascal: &str) -> String {
    for &(needle, replacement) in RESERVED_TYPE_NAMES {
        if pascal == needle {
            return replacement.to_owned();
        }
    }
    pascal.to_owned()
}

/// Top-level lowering: produces every type and every method from the [`Spec`].
pub fn lower(spec: &Spec) -> Modules {
    let mut types: Vec<(String, Origin, GenType)> = Vec::new();
    let mut methods: Vec<MethodOut> = Vec::new();
    let mut seen: BTreeSet<String> = BTreeSet::new();
    for m in &spec.methods {
        let category = m.category.clone();
        let pascal = safe_type_name(&method_to_pascal(&m.name));

        let mut local: Vec<GenType> = Vec::new();
        if let Some(gt) = generate_return_type(m, &mut seen) {
            collect(gt, &mut local);
        }
        for gt in local {
            types.push((category.clone(), Origin::Response, gt));
        }

        let method_name = m.name.clone();
        let snake = method_to_snake(&m.name);
        let description = m.description.trim().to_owned();
        let return_type = return_type_ident(m);
        let (mut params, param_types, mut object_options) = lower_params(m, &pascal, &mut seen);
        let mut local: Vec<GenType> = Vec::new();
        for gt in param_types {
            collect(gt, &mut local);
        }
        for gt in local {
            types.push((category.clone(), Origin::Param, gt));
        }

        let verbose = verbose_variants(m).unwrap_or_default();
        let selector_idx = m.params.iter().position(|p| is_selector(&p.name)).unwrap_or(0);
        if !verbose.is_empty() {
            // The selector is hardcoded per emitted method, so it never appears as an argument.
            params.retain(|p| !is_selector(&p.wire_name));
            object_options = false;
        }

        methods.push(MethodOut {
            method_name,
            snake,
            pascal,
            description,
            return_type,
            params,
            category,
            object_options,
            verbose_variants: verbose,
            selector_idx,
        });
    }
    Modules { types, methods, type_names: seen }
}

/// Whether a schema is a JSON object carrying real (non-commentary) properties.
fn is_object_with_props(schema: &Schema) -> bool {
    schema.primary_kind() == Some("object") && schema.properties.is_some() && schema.has_props()
}

/// Lower a method's parameters into [`ParamOut`]s and any helper types they spawn.
fn lower_params(
    method: &Method,
    pascal: &str,
    seen: &mut BTreeSet<String>,
) -> (Vec<ParamOut>, Vec<GenType>, bool) {
    let opt: Vec<&Param> = method.params.iter().filter(|p| !p.required).collect();
    let object_options = opt.len() == 1 && is_object_with_props(&opt[0].schema);
    let options_name = format!("{pascal}Options");

    let mut params: Vec<ParamOut> = Vec::new();
    let mut nested: Vec<GenType> = Vec::new();

    if object_options {
        for p in method.params.iter().filter(|p| p.required) {
            let (ty, n) =
                param_to_type(&p.schema, pascal, &p.name, Some(&p.name), &options_name, seen);
            nested.extend(n);
            params.push(ParamOut {
                rust_name: to_rust_field(&p.name),
                wire_name: p.name.clone(),
                rust_type: ty,
                description: param_description(p),
                required: true,
                default: p.schema.default.clone(),
            });
        }
        params.extend(flatten_object_fields(&opt[0].schema, &options_name, seen, &mut nested));
        return (params, nested, true);
    }

    for p in &method.params {
        let (ty, n) = param_to_type(&p.schema, pascal, &p.name, Some(&p.name), &options_name, seen);
        nested.extend(n);
        params.push(ParamOut {
            rust_name: to_rust_field(&p.name),
            wire_name: p.name.clone(),
            rust_type: ty,
            description: param_description(p),
            required: p.required,
            default: p.schema.default.clone(),
        });
    }
    (params, nested, false)
}

/// Flatten an options object's properties into optional [`ParamOut`]s, skipping prose-only entries.
fn flatten_object_fields(
    schema: &Schema,
    parent: &str,
    seen: &mut BTreeSet<String>,
    nested: &mut Vec<GenType>,
) -> Vec<ParamOut> {
    let mut out: Vec<ParamOut> = Vec::new();
    let Some(props) = schema.properties.as_ref() else { return out };
    let mut used: BTreeSet<String> = BTreeSet::new();
    let mut keys: Vec<&String> = props.keys().collect();
    keys.sort();
    for k in keys {
        let v = &props[k];
        if v.is_string() {
            continue;
        }
        let Ok(field_schema) = serde_json::from_value::<Schema>(v.clone()) else {
            continue;
        };
        let (ty, n) = param_to_type(&field_schema, parent, k, Some(k), parent, seen);
        nested.extend(n);
        out.push(ParamOut {
            rust_name: uniquify(&mut used, &to_rust_field(k)),
            wire_name: k.clone(),
            rust_type: ty,
            description: field_schema.description.clone().unwrap_or_default().trim().to_owned(),
            required: false,
            default: field_schema.default.clone(),
        });
    }
    out
}

/// Map a parameter schema to a Rust type and the helper types it spawns.
fn param_to_type(
    schema: &Schema,
    parent: &str,
    field: &str,
    name: Option<&str>,
    reserved: &str,
    seen: &mut BTreeSet<String>,
) -> (String, Vec<GenType>) {
    if let Some(variants) = schema.one_of.as_ref().or(schema.any_of.as_ref()) {
        let tn = unique_type_name(&union_type_name(parent, field), reserved, seen);
        return enum_type(&tn, variants, schema.description.as_deref(), seen);
    }
    let is_map = schema.dynamic
        || matches!(&schema.additional_properties, Some(AdditionalProperties::Schema(_)))
            && !schema.has_props();
    match schema.primary_kind() {
        Some("object") if !is_map && schema.properties.is_some() => {
            let tn = unique_type_name(&format!("{parent}{}", to_pascal(field)), reserved, seen);
            let gt = struct_type(&tn, schema, schema.description.as_deref(), seen);
            (tn, vec![gt])
        }
        Some("object") => object_field(schema, parent, field, seen),
        Some("array") => match schema.array_items() {
            Some(items) => {
                let (it, n) = param_to_type(items, parent, field, None, reserved, seen);
                (format!("Vec<{it}>"), n)
            }
            None => ("Vec<serde_json::Value>".to_owned(), vec![]),
        },
        _ => (param_type(schema, name), vec![]),
    }
}

/// `base` if unique, otherwise `base` + `Arg`/`Arg2`/... to avoid clashing with `reserved` or seen names.
fn unique_type_name(base: &str, reserved: &str, seen: &BTreeSet<String>) -> String {
    let collides = |n: &str| n == reserved || seen.contains(n);
    if !collides(base) {
        return base.to_owned();
    }
    let arg = format!("{base}Arg");
    if !collides(&arg) {
        return arg;
    }
    (2..).map(|i| format!("{base}Arg{i}")).find(|n| !collides(n)).unwrap()
}

/// Flatten a `GenType` tree into a single `Vec` in DFS pre-order.
fn collect(gt: GenType, out: &mut Vec<GenType>) {
    for n in gt.nested {
        collect(n, out);
    }
    out.push(GenType { name: gt.name, body: gt.body, nested: Vec::new(), ir: gt.ir });
}

/// One variant of a verbose-selected union return.
#[derive(Debug, Clone)]
struct VerboseVariant {
    word: String,      // verbosity level digit: "0", "1", ...
    type_name: String, // concrete variant return type, e.g. "GetBlockVerbose1"
    selector: String,  // JSON literal for the selector argument: "0", "1", "true", "false"
}

/// Whether a parameter name is the verbosity selector.
fn is_selector(name: &str) -> bool {
    let n = name.to_ascii_lowercase();
    n == "verbose" || n == "verbosity"
}

/// Return per-variant methods if the result is cleanly selected by a `verbose`/`verbosity` param, else `None`.
fn verbose_variants(method: &Method) -> Option<Vec<VerboseVariant>> {
    let s = &method.result.schema;
    let variants = s.one_of.as_ref().or(s.any_of.as_ref())?;
    let sel_idx = method.params.iter().position(|p| is_selector(&p.name))?;
    let is_bool = method.params[sel_idx].schema.primary_kind() == Some("boolean");
    let pascal = safe_type_name(&method_to_pascal(&method.name));
    let labels = ["Verbose0", "Verbose1", "Verbose2", "Verbose3", "Verbose4"];
    let words = ["0", "1", "2", "3", "4"];
    let mut out = Vec::with_capacity(variants.len());
    for (i, v) in variants.iter().enumerate() {
        let cond = v.condition.clone().unwrap_or_else(|| v.description.clone().unwrap_or_default());
        if cond.contains(" and ") {
            return None;
        }
        let suffix = verbose_suffix(&cond, i);
        let level = labels.iter().position(|l| *l == suffix).unwrap_or(i).min(4);
        let selector = match (is_bool, level) {
            (true, 0) => "false".to_owned(),
            (true, _) => "true".to_owned(),
            (false, l) => l.to_string(),
        };
        out.push(VerboseVariant {
            word: words[level].to_owned(),
            type_name: format!("{pascal}{suffix}"),
            selector,
        });
    }
    Some(out)
}

/// Produce the Rust return-type identifier the method's `Result<>` wraps.
fn return_type_ident(method: &Method) -> String {
    let s = &method.result.schema;
    if s.returns_null() {
        return "()".to_owned();
    }
    safe_type_name(&method_to_pascal(&method.name))
}

/// Generate the top-level return type for a method, or `None` if the method returns null.
fn generate_return_type(method: &Method, seen: &mut BTreeSet<String>) -> Option<GenType> {
    let s = &method.result.schema;
    if s.returns_null() {
        return None;
    }
    let name = safe_type_name(&method_to_pascal(&method.name));
    let doc = method_doc(method);

    if s.is_simple() && !s.dynamic {
        return wrapper(&name, &simple_type(s, &name, "")?, Some(&doc), seen);
    }
    if let Some(variants) = s.one_of.as_ref().or(s.any_of.as_ref()) {
        if verbose_variants(method).is_some() {
            return one_of(&name, variants, method.description.as_deref_opt(), seen);
        }
        let (_, gts) = enum_type(&name, variants, Some(&doc), seen);
        return gts.into_iter().next();
    }
    if is_map(s) {
        return Some(map_type(&name, s, Some(&doc), seen));
    }
    if s.primary_kind() == Some("array") {
        return Some(array_type(&name, s, Some(&doc), seen));
    }
    Some(struct_type(&name, s, method.description.as_deref_opt(), seen))
}

/// Like `as_deref` but maps an empty string to `None`.
trait OptStrExt {
    fn as_deref_opt(&self) -> Option<&str>;
}
impl OptStrExt for String {
    fn as_deref_opt(&self) -> Option<&str> {
        if self.is_empty() {
            None
        } else {
            Some(self.as_str())
        }
    }
}

/// Emit a newtype wrapping a scalar Rust type.
fn wrapper(
    name: &str,
    inner: &str,
    doc: Option<&str>,
    seen: &mut BTreeSet<String>,
) -> Option<GenType> {
    seen.insert(name.to_owned());
    let mut body = fmt_doc(doc);
    body.push_str(DERIVES);
    body.push('\n');
    body.push_str(&format!("pub struct {name}(pub {inner});\n"));
    body.push_str(&format!(
        "\nimpl std::ops::Deref for {name} {{\n    type Target = {inner};\n    fn deref(&self) -> &Self::Target {{ &self.0 }}\n}}\n"
    ));
    Some(GenType {
        name: name.to_owned(),
        body,
        nested: vec![],
        ir: TypeIr::Newtype(inner.to_owned()),
    })
}

/// Emit a newtype wrapping a `Vec`.
fn array_type(
    name: &str,
    schema: &Schema,
    doc: Option<&str>,
    seen: &mut BTreeSet<String>,
) -> GenType {
    seen.insert(name.to_owned());
    let (item_ty, nested) = array_item(schema, name, seen);
    let mut body = fmt_doc(doc);
    body.push_str(DERIVES);
    body.push('\n');
    body.push_str(&format!("pub struct {name}(pub Vec<{item_ty}>);\n"));
    GenType { name: name.to_owned(), body, nested, ir: TypeIr::Newtype(format!("Vec<{item_ty}>")) }
}

/// Whether a schema is a heterogeneous fixed-length tuple (rendered as `Vec<serde_json::Value>`).
fn is_tuple(schema: &Schema) -> bool { schema.items.is_none() && schema.prefix_items.is_some() }

/// Whether the description signals the field may be absent despite appearing in `required`.
fn desc_marks_optional(desc: Option<&str>) -> bool {
    desc.is_some_and(|d| {
        let d = d.to_ascii_lowercase();
        d.contains("omitted")
            || d.contains("only present")
            || d.contains("only available")
            || d.contains("only when")
            || d.contains("only if")
            || d.contains("only shown")
            || d.contains("only returned")
    })
}

/// Resolve the element type of an array schema.
fn array_item(
    schema: &Schema,
    parent: &str,
    seen: &mut BTreeSet<String>,
) -> (String, Vec<GenType>) {
    if is_tuple(schema) {
        return ("serde_json::Value".to_owned(), vec![]);
    }
    let Some(items) = schema.array_items() else {
        return ("String".to_owned(), vec![]);
    };
    if items.primary_kind() == Some("object") && items.properties.is_some() {
        return inner_type(&format!("{parent}Item"), items, seen);
    }
    schema_to_type(items, parent, "", seen)
}

/// Emit a newtype wrapping a `BTreeMap<String, V>`.
fn map_type(
    name: &str,
    schema: &Schema,
    doc: Option<&str>,
    seen: &mut BTreeSet<String>,
) -> GenType {
    seen.insert(name.to_owned());
    let (vt, nested) = map_value(schema, name, seen);
    let desc = schema.description.as_deref().unwrap_or("Map entries");
    let mut body = fmt_doc(doc);
    body.push_str(DERIVES);
    body.push_str(&format!(
        "\npub struct {name}(\n    /// {desc}\n    pub std::collections::BTreeMap<String, {vt}>,\n);\n"
    ));
    GenType {
        name: name.to_owned(),
        body,
        nested,
        ir: TypeIr::Newtype(format!("std::collections::BTreeMap<String, {vt}>")),
    }
}

/// Resolve the value type for a map schema, falling back to `Value`.
fn map_value(schema: &Schema, parent: &str, seen: &mut BTreeSet<String>) -> (String, Vec<GenType>) {
    if let Some(AdditionalProperties::Schema(ap)) = &schema.additional_properties {
        let suffix = if parent.ends_with("Entry") { "Item" } else { "Entry" };
        return inner_type(&format!("{parent}{suffix}"), ap, seen);
    }
    ("serde_json::Value".to_owned(), vec![])
}

/// Lower a verbose-selected union into one concrete struct per verbosity level.
fn one_of(
    name: &str,
    variants: &[Schema],
    doc: Option<&str>,
    seen: &mut BTreeSet<String>,
) -> Option<GenType> {
    if variants.len() == 1 {
        return dispatch(name, &variants[0], doc, seen);
    }
    // Accumulate object arms so each verbosity level inherits fields from lower levels.
    let mut emitted: Vec<GenType> = Vec::new();
    let mut base_props = serde_json::Map::new();
    let mut base_required: Vec<String> = Vec::new();
    for (i, v) in variants.iter().enumerate() {
        let cond = v.condition.clone().unwrap_or_else(|| v.description.clone().unwrap_or_default());
        let suffix =
            if cond.contains(" and ") { verbose_suffix("", i) } else { verbose_suffix(&cond, i) };

        let merged;
        let arm: &Schema = if let Some(props) =
            v.properties.as_ref().filter(|_| v.primary_kind() == Some("object"))
        {
            for (k, val) in props {
                base_props.insert(k.clone(), val.clone());
            }
            if let Some(required) = &v.required {
                for n in required {
                    if !base_required.contains(n) {
                        base_required.push(n.clone());
                    }
                }
            }
            merged = Schema {
                kind: Some(SchemaType::One("object".to_owned())),
                properties: Some(base_props.clone()),
                required: Some(base_required.clone()),
                ..Default::default()
            };
            &merged
        } else {
            v
        };
        if let Some(g) = dispatch(&format!("{name}{suffix}"), arm, doc, seen) {
            emitted.push(g);
        }
    }
    if emitted.is_empty() {
        return None;
    }
    let mut primary = emitted.remove(0);
    for extra in emitted {
        primary.nested.extend(extra.nested);
        primary.nested.push(GenType {
            name: extra.name,
            body: extra.body,
            nested: vec![],
            ir: extra.ir,
        });
    }
    Some(primary)
}

/// Generate a `#[serde(untagged)]` enum for a data union (non-verbose `oneOf`/`anyOf`).
fn enum_type(
    name: &str,
    variants: &[Schema],
    doc: Option<&str>,
    seen: &mut BTreeSet<String>,
) -> (String, Vec<GenType>) {
    if !seen.insert(name.to_owned()) {
        return (name.to_owned(), vec![]);
    }
    let mut nested: Vec<GenType> = Vec::new();
    let mut variant_lines: Vec<String> = Vec::new();
    let mut variant_pairs: Vec<(String, String)> = Vec::new();
    let mut used: BTreeSet<String> = BTreeSet::new();
    for (i, arm) in variants.iter().enumerate() {
        let is_plain_number = arm.one_of.is_none()
            && arm.any_of.is_none()
            && arm.bitcoin_type.is_none()
            && arm.primary_kind() == Some("number");
        let (ty, n) = if is_plain_number {
            ("f64".to_owned(), vec![])
        } else {
            inner_type(&format!("{name}Variant{i}"), arm, seen)
        };
        nested.extend(n);
        let ident = uniquify(&mut used, arm_variant_ident(arm));
        variant_lines.push(format!("    {ident}({ty}),"));
        variant_pairs.push((ident, ty));
    }
    let mut body = fmt_doc(doc);
    body.push_str("#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]\n");
    body.push_str("#[serde(untagged)]\n");
    body.push_str(&format!("pub enum {name} {{\n{}\n}}\n", variant_lines.join("\n")));
    (
        name.to_owned(),
        vec![GenType { name: name.to_owned(), body, nested, ir: TypeIr::Enum(variant_pairs) }],
    )
}

/// First-choice variant identifier for a union arm by JSON kind; callers suffix on collision.
fn arm_variant_ident(arm: &Schema) -> &'static str {
    if arm.one_of.is_some() || arm.any_of.is_some() {
        return "Nested";
    }
    match arm.primary_kind() {
        Some("string") => "Text",
        Some("boolean") => "Bool",
        Some("integer") | Some("number") => "Number",
        Some("array") => "List",
        Some("object") => "Object",
        Some("null") => "Null",
        _ => "Value",
    }
}

/// Name for a union type: `{Parent}{Field}`, or `{Parent}` when there is no field context.
fn union_type_name(parent: &str, field: &str) -> String {
    if field.is_empty() {
        parent.to_owned()
    } else {
        format!("{parent}{}", to_pascal(field))
    }
}

/// Insert `base` into `used`, appending `2`, `3`, ... until unique.
fn uniquify(used: &mut BTreeSet<String>, base: &str) -> String {
    if used.insert(base.to_owned()) {
        return base.to_owned();
    }
    (2..).map(|n| format!("{base}{n}")).find(|c| used.insert(c.clone())).unwrap()
}

/// Whether an `object` schema is a dynamic string-keyed map rather than a fixed struct.
fn is_map(schema: &Schema) -> bool {
    schema.dynamic
        || (matches!(&schema.additional_properties, Some(AdditionalProperties::Schema(_)))
            && !schema.has_props())
}

/// Dispatch a schema to the right type-generator by its primary kind.
fn dispatch(
    name: &str,
    schema: &Schema,
    doc: Option<&str>,
    seen: &mut BTreeSet<String>,
) -> Option<GenType> {
    match schema.primary_kind() {
        Some("object") => Some(if is_map(schema) {
            map_type(name, schema, doc, seen)
        } else {
            struct_type(name, schema, doc, seen)
        }),
        Some("array") => Some(array_type(name, schema, doc, seen)),
        _ => wrapper(name, &simple_type(schema, name, "")?, doc, seen),
    }
}

/// Emit a `pub struct` from an object schema.
fn struct_type(
    name: &str,
    schema: &Schema,
    doc: Option<&str>,
    seen: &mut BTreeSet<String>,
) -> GenType {
    seen.insert(name.to_owned());
    let props_map = match &schema.properties {
        Some(m) => m,
        None => &serde_json::Map::new(),
    };
    let required: BTreeSet<&str> = schema
        .required
        .as_ref()
        .map(|v| v.iter().map(String::as_str).collect())
        .unwrap_or_default();

    let mut field_lines: Vec<String> = Vec::new();
    let mut field_irs: Vec<FieldIr> = Vec::new();
    let mut nested: Vec<GenType> = Vec::new();
    let mut commentary_only: Vec<&str> = Vec::new();
    let mut used_fields: BTreeSet<String> = BTreeSet::new();

    let mut keys: Vec<&String> = props_map.keys().collect();
    keys.sort();
    for k in keys {
        let v = &props_map[k];
        if v.is_string() {
            commentary_only.push(v.as_str().unwrap());
            continue;
        }
        let Ok(field_schema) = serde_json::from_value::<Schema>(v.clone()) else {
            continue;
        };
        let optional = !required.contains(k.as_str())
            || field_schema.bitcoin_optional
            || desc_marks_optional(field_schema.description.as_deref());
        let (ty, nested_ty) = schema_to_type(&field_schema, name, k, seen);
        nested.extend(nested_ty);
        let rust_name = uniquify(&mut used_fields, &to_rust_field(k));
        let mut serde_args: Vec<String> = Vec::new();
        if rust_name != *k {
            serde_args.push(format!("rename = \"{k}\""));
        }
        if optional {
            serde_args.push("skip_serializing_if = \"Option::is_none\"".to_owned());
        }
        let attr = if serde_args.is_empty() {
            String::new()
        } else {
            format!("    #[serde({})]\n", serde_args.join(", "))
        };
        let final_ty = if optional { format!("Option<{ty}>") } else { ty.clone() };
        let fdoc_text = field_schema.description.as_deref().filter(|s| !s.is_empty()).map(esc_doc);
        let fdoc = fdoc_text
            .as_deref()
            .map(|d| format!("    /// {}\n", d.replace('\n', "\n    /// ")))
            .unwrap_or_default();
        field_lines.push(format!("{fdoc}{attr}    pub {rust_name}: {final_ty},"));
        field_irs.push(FieldIr { rust_name, rust_type: ty, optional, doc: fdoc_text });
    }

    let header = fmt_doc(doc);

    if field_lines.is_empty()
        && commentary_only.iter().any(|s| s.to_lowercase().contains("decoderawtransaction"))
    {
        let body = format!("{header}pub type {name} = DecodeRawTransaction;\n");
        return GenType {
            name: name.to_owned(),
            body,
            nested,
            ir: TypeIr::Alias("DecodeRawTransaction".to_owned()),
        };
    }

    // `additionalProperties: true` or getblock tx items: capture extra fields in a flattened map.
    let open = matches!(&schema.additional_properties, Some(AdditionalProperties::Bool(true)))
        || (name.starts_with("GetBlockVerbose") && name.ends_with("TxItem"));
    let derives =
        if open { "#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]" } else { DERIVES };
    let body = if field_lines.is_empty() && !open {
        format!("{header}{derives}\npub struct {name} {{}}\n")
    } else {
        if open {
            field_lines.push(
                "    #[serde(flatten)]\n    pub extra: std::collections::BTreeMap<String, serde_json::Value>,"
                    .to_owned(),
            );
        }
        format!("{header}{derives}\npub struct {name} {{\n{}\n}}\n", field_lines.join("\n"))
    };
    GenType { name: name.to_owned(), body, nested, ir: TypeIr::Struct(field_irs) }
}

/// Resolve a nested schema to a Rust type and any helper types it requires.
fn inner_type(name: &str, schema: &Schema, seen: &mut BTreeSet<String>) -> (String, Vec<GenType>) {
    if schema.properties.is_some() {
        let gt = struct_type(name, schema, schema.description.as_deref(), seen);
        let result_name = gt.name.clone();
        return (result_name, vec![gt]);
    }
    if schema.dynamic {
        if let Some(AdditionalProperties::Schema(ap)) = &schema.additional_properties {
            let (it, n) = inner_type(&format!("{name}Entry"), ap, seen);
            return (format!("std::collections::BTreeMap<String, {it}>"), n);
        }
    }
    schema_to_type(schema, name, "", seen)
}

/// Map a schema used as a struct field or array element to a Rust type.
fn schema_to_type(
    schema: &Schema,
    parent: &str,
    field: &str,
    seen: &mut BTreeSet<String>,
) -> (String, Vec<GenType>) {
    if let Some(variants) = schema.one_of.as_ref().or(schema.any_of.as_ref()) {
        let name = union_type_name(parent, field);
        return enum_type(&name, variants, schema.description.as_deref(), seen);
    }
    if matches!(schema.bitcoin_type.as_deref(), Some("hex")) {
        return ("String".to_owned(), vec![]);
    }
    if matches!(schema.bitcoin_type.as_deref(), Some("amount")) {
        return ("f64".to_owned(), vec![]);
    }
    if schema.primary_kind() == Some("object")
        && schema.description.as_deref().is_some_and(|d| d.contains("or false"))
    {
        return ("serde_json::Value".to_owned(), vec![]);
    }
    match schema.primary_kind() {
        Some("string") if is_amount_string(parent, field, schema.description.as_deref()) =>
            ("f64".to_owned(), vec![]),
        Some("string") => ("String".to_owned(), vec![]),
        Some("boolean") => ("bool".to_owned(), vec![]),
        Some("integer") | Some("number") => (
            classify_number(parent, field, schema.description.as_deref()).rust_type().to_owned(),
            vec![],
        ),
        Some("null") => ("()".to_owned(), vec![]),
        Some("array") => array_field(schema, parent, field, seen),
        Some("object") => object_field(schema, parent, field, seen),
        _ => ("serde_json::Value".to_owned(), vec![]),
    }
}

/// Lower an array-typed struct field.
fn array_field(
    schema: &Schema,
    parent: &str,
    field: &str,
    seen: &mut BTreeSet<String>,
) -> (String, Vec<GenType>) {
    if is_tuple(schema) {
        return ("Vec<serde_json::Value>".to_owned(), vec![]);
    }
    let Some(items) = schema.array_items() else {
        return ("Vec<serde_json::Value>".to_owned(), vec![]);
    };
    if items.primary_kind() == Some("object") && items.properties.is_some() {
        let iname = format!("{parent}{}Item", to_pascal(field));
        let (tn, n) = inner_type(&iname, items, seen);
        return (format!("Vec<{tn}>"), n);
    }
    let (it, n) = schema_to_type(items, parent, field, seen);
    (format!("Vec<{it}>"), n)
}

/// Lower an object-typed struct field.
fn object_field(
    schema: &Schema,
    parent: &str,
    field: &str,
    seen: &mut BTreeSet<String>,
) -> (String, Vec<GenType>) {
    if is_map(schema) {
        if let Some(AdditionalProperties::Schema(ap)) = &schema.additional_properties {
            let (vt, n) = if ap.primary_kind() == Some("object") && ap.properties.is_some() {
                inner_type(&format!("{parent}{}", to_pascal(field)), ap, seen)
            } else {
                schema_to_type(ap, parent, field, seen)
            };
            return (format!("std::collections::BTreeMap<String, {vt}>"), n);
        }
        return ("std::collections::BTreeMap<String, serde_json::Value>".to_owned(), vec![]);
    }
    if schema.properties.is_some() {
        let nname = format!("{parent}{}", to_pascal(field));
        let gt = struct_type(&nname, schema, schema.description.as_deref(), seen);
        return (nname, vec![gt]);
    }
    ("serde_json::Value".to_owned(), vec![])
}

/// Scalar type for a newtype wrapper or union arm.
fn simple_type(schema: &Schema, parent: &str, field: &str) -> Option<String> {
    Some(match schema.primary_kind()? {
        "string" if is_amount_string(parent, field, schema.description.as_deref()) =>
            "f64".to_owned(),
        "string" => "String".to_owned(),
        "boolean" => "bool".to_owned(),
        "number" | "integer" =>
            classify_number(parent, field, schema.description.as_deref()).rust_type().to_owned(),
        _ => return None,
    })
}

/// Concrete integer/float kind for a JSON `number` field; unrecognized names default to `i64`.
#[derive(Clone, Copy, PartialEq, Debug)]
enum NumKind {
    I64,
    U64,
    F64,
}

impl NumKind {
    /// The Rust primitive name.
    fn rust_type(self) -> &'static str {
        match self {
            NumKind::I64 => "i64",
            NumKind::U64 => "u64",
            NumKind::F64 => "f64",
        }
    }
}

/// Pick the integer/float kind for a `type: number` field.
fn classify_number(parent: &str, field: &str, desc: Option<&str>) -> NumKind {
    if desc.is_some_and(|d| d.contains("BTC")) {
        return NumKind::F64;
    }
    if matches!(field, "minfeefilter" | "chunkfee") {
        return NumKind::F64;
    }

    if FLOAT_FIELDS.contains(&field) {
        return NumKind::F64;
    }
    if field == "progress" && parent.contains("Scanning") {
        return NumKind::F64;
    }
    if field.is_empty() && matches!(parent, "GetDifficulty" | "GetNetworkHashPs") {
        return NumKind::F64;
    }
    if field == "size" {
        return if is_tx_context(parent) { NumKind::I64 } else { NumKind::U64 };
    }
    if field == "txouts" {
        return if parent.starts_with("GetTxOutSetInfo") { NumKind::U64 } else { NumKind::I64 };
    }
    if field.is_empty() && parent == "GetConnectionCount" {
        return NumKind::U64;
    }
    if parent.contains("Locked")
        && matches!(field, "total" | "used" | "free" | "locked" | "chunks_free" | "chunks_used")
    {
        return NumKind::U64;
    }
    if U64_FIELDS.contains(&field) {
        return NumKind::U64;
    }
    NumKind::I64
}

/// Whether a `type: string` field is actually a BTC amount (Core tags them `string` in the export).
fn is_amount_string(parent: &str, field: &str, desc: Option<&str>) -> bool {
    if desc.is_some_and(|d| d.contains("BTC")) {
        return true;
    }
    if AMOUNT_STRING_FIELDS.contains(&field) {
        return true;
    }
    if field.is_empty()
        && matches!(parent, "GetBalance" | "GetReceivedByAddress" | "GetReceivedByLabel")
    {
        return true;
    }
    if matches!(parent, "GetBalancesMine" | "GetBalancesWatchonly") {
        return true;
    }
    parent.starts_with("GetTxOutSetInfoBlockInfo")
}

/// Whether a type name denotes a transaction context, where `size` is `unsigned int` not `size_t`.
fn is_tx_context(parent: &str) -> bool {
    parent.contains("Tx")
        || parent.contains("Transaction")
        || parent.contains("Utxo")
        || parent.contains("Psbt")
        || parent.contains("Decode")
}

/// Map a verbosity-arm condition string to a PascalCase suffix ("verbose=1" -> "Verbose1").
fn verbose_suffix(condition: &str, index: usize) -> String {
    let c = condition.to_ascii_lowercase();
    let patterns: &[(&str, &[&str])] = &[
        (
            "Verbose0",
            &[
                "verbose=false",
                "verbose=0",
                "verbose is not set",
                "verbose is false",
                "verbosity=0",
            ],
        ),
        (
            "Verbose1",
            &[
                "verbose=true",
                "verbose=1",
                "verbose is set to true",
                "verbose is set to 1",
                "verbosity=1",
            ],
        ),
        ("Verbose2", &["verbose=2", "verbosity=2"]),
        ("Verbose3", &["verbose=3", "verbosity=3"]),
    ];
    let normalised = c.replace(' ', "");
    for (label, needles) in patterns {
        if needles.iter().any(|n| normalised.contains(&n.replace(' ', ""))) {
            return (*label).to_owned();
        }
    }
    ["Verbose0", "Verbose1", "Verbose2", "Verbose3", "Verbose4"][index.min(4)].to_owned()
}

/// Emit the async method(s) for one RPC entry.
fn emit_method(m: &MethodOut) -> String {
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
fn param_description(p: &Param) -> String {
    if !p.schema.description.as_deref().unwrap_or("").is_empty() {
        return p.schema.description.clone().unwrap_or_default().trim().to_owned();
    }
    p.description.trim().to_owned()
}

/// Map a parameter schema to a Rust type for the method signature.
fn param_type(schema: &Schema, name: Option<&str>) -> String {
    if matches!(schema.bitcoin_type.as_deref(), Some("amount")) {
        return "f64".to_owned();
    }
    if matches!(schema.bitcoin_type.as_deref(), Some("hex")) {
        return "String".to_owned();
    }
    if schema.one_of.is_some() || schema.any_of.is_some() {
        return "serde_json::Value".to_owned();
    }
    match schema.primary_kind() {
        Some("string") => "String".to_owned(),
        Some("boolean") => "bool".to_owned(),
        Some("integer") => "i64".to_owned(),
        Some("number") => {
            if let Some(Value::Number(n)) = &schema.default {
                if n.is_i64() || n.is_u64() {
                    return "i64".to_owned();
                }
            }
            if let Some(name) = name {
                if INTEGER_PARAM_NAMES.iter().any(|n| n.eq_ignore_ascii_case(name)) {
                    return "i64".to_owned();
                }
            }
            "f64".to_owned()
        }
        Some("array") => {
            let item = match schema.array_items() {
                Some(items) => param_type(items, None),
                None => "serde_json::Value".to_owned(),
            };
            format!("Vec<{item}>")
        }
        Some("object") => "serde_json::Value".to_owned(),
        _ => "serde_json::Value".to_owned(),
    }
}

/// Format lines as an indented `///`-prefixed doc-comment block, stripping trailing blank lines.
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
fn fmt_doc(doc: Option<&str>) -> String {
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
fn method_doc(method: &Method) -> String {
    let summary_lines: Vec<String> = method.description.lines().map(esc_doc).collect();
    let body = summary_lines.join("\n/// > ");
    format!(
        "/// Result of the JSON-RPC method `{}`.\n///\n/// > {}\n/// >\n/// > {}",
        method.name, method.name, body
    )
}

/// Escape characters that have special meaning in rustdoc/Markdown to avoid `-D warnings` failures.
fn esc_doc(s: &str) -> String {
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
    s.push_str("\npub mod model;\n\n");
    for cat in categories {
        s.push_str(&format!("pub use self::{}::*;\n", category_module(cat)));
    }
    s
}

/// Emit `mod.rs` for `corepc-types/generated/model/`.
fn emit_model_mod_rs(version: &str, categories: &[String]) -> String {
    let mut s = String::new();
    s.push_str(&format!(
        "// SPDX-License-Identifier: CC0-1.0\n\n\
         //! Auto-generated model types for Bitcoin Core `{version}`, split by API section.\n//!\n\
         //! Generated by `codegen`. Do not edit any file in this module by hand.\n\
         //! Re-run `just codegen` from the workspace root to regenerate.\n\n"
    ));
    for cat in categories {
        s.push_str(&format!("pub mod {};\n", category_module(cat)));
    }
    s.push('\n');
    for cat in categories {
        s.push_str(&format!("pub use self::{}::*;\n", category_module(cat)));
    }
    s
}

/// Emit `mod.rs` for `corepc-client/v{N}/`.
fn emit_client_mod_rs(version: &str, categories: &[String]) -> String {
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
    s.push('\n');
    for cat in categories {
        s.push_str(&format!("pub use self::{}::*;\n", category_module(cat)));
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classify_number_resolves_the_real_collisions() {
        use NumKind::*;
        assert_eq!(classify_number("GetPeerinfoItem", "minfeefilter", None), F64);
        assert_eq!(classify_number("ScanBlocksVerbose2", "progress", None), I64);
        assert_eq!(classify_number("GetBlockVerbose2", "size", None), U64);
        assert_eq!(classify_number("GetMempoolInfo", "size", None), U64);
        assert_eq!(classify_number("GetBlockVerbose2TxItem", "size", None), I64);
        assert_eq!(classify_number("GetTxOutSetInfo", "txouts", None), U64);
        assert_eq!(classify_number("ScanTxOutSetVerbose0", "txouts", None), I64);
        assert_eq!(classify_number("GetMemoryInfoVerbose0Locked", "total", None), U64);
    }

    #[test]
    fn esc_doc_escapes_angle_brackets() {
        assert_eq!(esc_doc("see <wallet name> on disk"), "see \\<wallet name\\> on disk");
    }

    #[test]
    fn esc_doc_passes_through_plain_text() {
        assert_eq!(esc_doc("hello world"), "hello world");
    }

    #[test]
    fn esc_doc_escapes_square_brackets() {
        assert_eq!(esc_doc("either end or [begin,end]"), "either end or \\[begin,end\\]");
    }

    #[test]
    fn esc_doc_handles_urls() {
        assert_eq!(
            esc_doc("See https://en.bitcoin.it/wiki/BIP_0022 for full specification."),
            "See <https://en.bitcoin.it/wiki/BIP_0022> for full specification."
        );
        assert_eq!(
            esc_doc("design document (<https://github.com/bitcoin/bitcoin/blob/master/doc/design/assumeutxo.md>)."),
            "design document (<https://github.com/bitcoin/bitcoin/blob/master/doc/design/assumeutxo.md>)."
        );
        assert_eq!(esc_doc("read https://example.com."), "read <https://example.com>.");
    }

    #[test]
    fn esc_doc_preserves_intra_doc_code_links() {
        assert_eq!(
            esc_doc("with all optional arguments via [`GetBlockStatsOptions`]."),
            "with all optional arguments via [`GetBlockStatsOptions`]."
        );
        assert_eq!(esc_doc("a [bare] bracket"), "a \\[bare\\] bracket");
    }

    #[test]
    fn integer_param_names_includes_well_known_counts() {
        for name in ["height", "verbosity", "minconf", "nblocks"] {
            assert!(
                INTEGER_PARAM_NAMES.iter().any(|n| n.eq_ignore_ascii_case(name)),
                "{name} should be flagged as integer-only"
            );
        }
    }

    #[test]
    fn verbose_suffix_zero_and_one() {
        assert_eq!(verbose_suffix("verbose=0", 0), "Verbose0");
        assert_eq!(verbose_suffix("verbosity=1", 1), "Verbose1");
        assert_eq!(verbose_suffix("verbose is set to true", 0), "Verbose1");
    }

    fn schema(json: Value) -> Schema {
        serde_json::from_value(json).expect("schema should deserialise")
    }

    #[test]
    fn param_type_number_with_integer_name_becomes_i64() {
        let s = schema(serde_json::json!({ "type": "number" }));
        assert_eq!(param_type(&s, Some("conf_target")), "i64");
        assert_eq!(param_type(&s, Some("height")), "i64");
        assert_eq!(param_type(&s, Some("HEIGHT")), "i64");
    }

    #[test]
    fn param_type_number_with_unknown_name_stays_f64() {
        let s = schema(serde_json::json!({ "type": "number" }));
        assert_eq!(param_type(&s, Some("feerate")), "f64");
        assert_eq!(param_type(&s, None), "f64");
    }

    #[test]
    fn param_type_number_with_integer_default_becomes_i64() {
        let s = schema(serde_json::json!({ "type": "number", "default": 6 }));
        assert_eq!(param_type(&s, Some("totallyunknownparam")), "i64");

        let fractional = schema(serde_json::json!({ "type": "number", "default": 0.5 }));
        assert_eq!(param_type(&fractional, Some("totallyunknownparam")), "f64");
    }

    #[test]
    fn param_type_bitcoin_overrides_win_over_json_type() {
        let amount = schema(serde_json::json!({ "type": "string", "x-bitcoin-type": "amount" }));
        assert_eq!(param_type(&amount, Some("amount")), "f64");

        let hex = schema(serde_json::json!({ "type": "number", "x-bitcoin-type": "hex" }));
        assert_eq!(param_type(&hex, Some("data")), "String");
    }

    #[test]
    fn param_type_union_falls_back_to_value() {
        let s =
            schema(serde_json::json!({ "oneOf": [{ "type": "string" }, { "type": "number" }] }));
        assert_eq!(param_type(&s, Some("x")), "serde_json::Value");
    }

    #[test]
    fn param_type_array_wraps_item_type() {
        let s = schema(serde_json::json!({ "type": "array", "items": { "type": "string" } }));
        assert_eq!(param_type(&s, Some("addresses")), "Vec<String>");
    }

    #[test]
    fn safe_type_name_rewrites_only_reserved_idents() {
        assert_eq!(safe_type_name("Send"), "SendResult");
        assert_eq!(safe_type_name("Result"), "ResultResponse");
        assert_eq!(safe_type_name("GetBlockHeader"), "GetBlockHeader");
    }

    fn method(json: Value) -> Method {
        serde_json::from_value(json).expect("method should deserialise")
    }

    #[test]
    fn return_type_ident_maps_each_result_shape() {
        let null = method(serde_json::json!({
            "name": "stop", "result": { "schema": { "type": "null" } }
        }));
        assert_eq!(return_type_ident(&null), "()");

        let union = method(serde_json::json!({
            "name": "getblock",
            "result": { "schema": { "oneOf": [{ "type": "string" }, { "type": "object" }] } }
        }));
        assert_eq!(return_type_ident(&union), "GetBlock");

        let named = method(serde_json::json!({
            "name": "getblockheader", "result": { "schema": { "type": "object" } }
        }));
        assert_eq!(return_type_ident(&named), "GetBlockHeader");

        let reserved = method(serde_json::json!({
            "name": "send", "result": { "schema": { "type": "object" } }
        }));
        assert_eq!(return_type_ident(&reserved), "SendResult");
    }

    #[test]
    fn null_result_generates_no_return_type() {
        let m = method(serde_json::json!({
            "name": "stop", "result": { "schema": { "type": "null" } }
        }));
        let mut seen = BTreeSet::new();
        assert!(generate_return_type(&m, &mut seen).is_none());
    }

    #[test]
    fn lower_handles_the_committed_spec() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/specs/v31_0_0_openrpc.json");
        let raw = std::fs::read_to_string(path).expect("spec file present");
        let spec: Spec = serde_json::from_str(&raw).expect("spec parses");
        let n_methods = spec.methods.len();
        assert!(n_methods > 100, "sanity: spec should describe the full RPC surface");

        let modules = lower(&spec);
        assert_eq!(modules.methods_count(), n_methods, "one MethodOut per spec method");
        assert!(modules.types_count() > 0);
        assert!(modules.option_count() > 0, "some methods have optional positional params");
    }

    #[test]
    fn enum_type_emits_untagged_variants() {
        let variants: Vec<Schema> = vec![
            schema(serde_json::json!({ "type": "number" })),
            schema(serde_json::json!({ "type": "string" })),
        ];
        let mut seen = BTreeSet::new();
        let (name, gts) = enum_type("FeeRate", &variants, None, &mut seen);
        assert_eq!(name, "FeeRate");
        let body = &gts[0].body;
        assert!(body.contains("#[serde(untagged)]"));
        assert!(body.contains("pub enum FeeRate"));
        assert!(body.contains("Number(f64)"), "first arm: {body}");
        assert!(body.contains("Text(String)"));
    }

    #[test]
    fn verbose_variants_numeric_verbosity() {
        let m = method(serde_json::json!({
            "name": "getblock",
            "params": [
                { "name": "blockhash", "required": true, "schema": { "type": "string" } },
                { "name": "verbosity", "required": false, "schema": { "type": "number" } }
            ],
            "result": { "schema": { "oneOf": [
                { "type": "string", "x-bitcoin-condition": "verbosity=0" },
                { "type": "object", "x-bitcoin-condition": "verbosity=1" },
                { "type": "object", "x-bitcoin-condition": "verbosity=2" }
            ] } }
        }));
        let vs = verbose_variants(&m).expect("verbosity-selected");
        let pairs: Vec<(&str, &str)> =
            vs.iter().map(|v| (v.word.as_str(), v.selector.as_str())).collect();
        assert_eq!(pairs, vec![("0", "0"), ("1", "1"), ("2", "2")]);
    }

    #[test]
    fn verbose_variants_boolean_respects_condition_order() {
        let m = method(serde_json::json!({
            "name": "getblockheader",
            "params": [
                { "name": "blockhash", "required": true, "schema": { "type": "string" } },
                { "name": "verbose", "required": false, "schema": { "type": "boolean" } }
            ],
            "result": { "schema": { "oneOf": [
                { "type": "object", "x-bitcoin-condition": "verbose=true" },
                { "type": "string", "x-bitcoin-condition": "verbose=false" }
            ] } }
        }));
        let vs = verbose_variants(&m).expect("verbose-selected");
        let one = vs.iter().find(|v| v.word == "1").unwrap();
        let zero = vs.iter().find(|v| v.word == "0").unwrap();
        assert_eq!(one.selector, "true");
        assert_eq!(zero.selector, "false");
    }

    #[test]
    fn verbose_variants_keep_optionals_preceding_the_selector() {
        let m = method(serde_json::json!({
            "name": "sendtoaddress",
            "params": [
                { "name": "address", "required": true, "schema": { "type": "string" } },
                { "name": "comment", "required": false, "schema": { "type": "string" } },
                { "name": "verbose", "required": false, "schema": { "type": "boolean" } }
            ],
            "result": { "schema": { "oneOf": [
                { "type": "string", "x-bitcoin-condition": "verbose=false" },
                { "type": "object", "x-bitcoin-condition": "verbose=true" }
            ] } }
        }));

        assert!(verbose_variants(&m).is_some());
        let spec = Spec { methods: vec![m] };
        let modules = lower(&spec);
        let out = &modules.methods[0];
        assert!(
            out.params.iter().all(|p| p.wire_name != "verbose"),
            "selector dropped from params"
        );
        assert!(out.params.iter().any(|p| p.wire_name == "comment"), "optional retained");

        let code = emit_method(out);
        // Level 0 is explicitly suffixed (no bare default name).
        assert!(code.contains("pub async fn send_to_address_verbose_0(&self, address: String)"));
        assert!(code.contains("pub async fn send_to_address_verbose_1(&self, address: String)"));
        // Each level gets a `_with` companion taking the options struct.
        assert!(code.contains(
            "pub async fn send_to_address_verbose_0_with(&self, address: String, opts: SendToAddressOptions)"
        ));
        assert!(code.contains(
            "pub async fn send_to_address_verbose_1_with(&self, address: String, opts: SendToAddressOptions)"
        ));
        // Bare methods send the optional as null; `_with` reads it from opts. Selector hardcoded.
        assert!(code.contains("json!(address), json!(null), json!(false)"), "v0 bare: {code}");
        assert!(
            code.contains("json!(address), json!(opts.comment), json!(false)"),
            "v0 with: {code}"
        );
        assert!(
            code.contains("json!(address), json!(opts.comment), json!(true)"),
            "v1 with: {code}"
        );
    }

    #[test]
    fn verbose_variants_bails_on_multi_parameter_condition() {
        let m = method(serde_json::json!({
            "name": "getrawmempool",
            "params": [
                { "name": "verbose", "required": false, "schema": { "type": "boolean" } },
                { "name": "mempool_sequence", "required": false, "schema": { "type": "boolean" } }
            ],
            "result": { "schema": { "oneOf": [
                { "type": "array", "x-bitcoin-condition": "verbose=false" },
                { "type": "object", "x-bitcoin-condition": "verbose=true" },
                { "type": "object", "x-bitcoin-condition": "verbose=false and mempool_sequence=true" }
            ] } }
        }));
        assert!(verbose_variants(&m).is_none());
    }

    #[test]
    fn single_object_option_is_flattened() {
        let m = method(serde_json::json!({
            "name": "bumpfee",
            "params": [
                { "name": "txid", "required": true, "schema": { "type": "string" } },
                { "name": "options", "required": false, "schema": { "type": "object", "properties": {
                    "conf_target": { "type": "number" },
                    "replaceable": { "type": "boolean" }
                } } }
            ],
            "result": { "schema": { "type": "object" } }
        }));
        let mut seen = BTreeSet::new();
        let (params, _types, object_options) = lower_params(&m, "BumpFee", &mut seen);
        assert!(object_options, "single object option must flatten");
        let opt_fields: Vec<&str> =
            params.iter().filter(|p| !p.required).map(|p| p.rust_name.as_str()).collect();
        assert!(opt_fields.contains(&"conf_target"), "inner keys become fields: {opt_fields:?}");
        assert!(opt_fields.contains(&"replaceable"));
        assert!(params.iter().any(|p| p.rust_name == "txid" && p.required));
    }
}
