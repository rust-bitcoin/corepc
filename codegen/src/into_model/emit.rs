// SPDX-License-Identifier: CC0-1.0

//! Emission: parse the canonical `crate::model` shapes, build each `into_model` impl and its error
//! enum, and walk the type graph from every root ([`generate_category`]). Also emits the shared
//! `compatibility.rs` shims ([`emit_compatibility`]).

use std::collections::{BTreeSet, VecDeque};
use std::fs;
use std::path::Path;

use super::convert::*;
use super::tables::*;
use super::*;

/// The parsed shape of a canonical `crate::model` type.
enum CanonShape {
    /// `pub struct X { .. }`: `(field_name, field_type)` pairs in declaration order.
    Struct(Vec<(String, String)>),
    /// `pub struct X(pub Inner);`, the inner Rust type.
    Newtype(String),
    /// `pub enum X { Variant(Inner), .. }`: `(variant ident, inner type)` pairs in order.
    Enum(Vec<(String, String)>),
}

/// Parse the canonical model type `name` from the model source under `model_dir`.
fn parse_canonical(model_dir: &Path, name: &str) -> Option<CanonShape> {
    let brace = format!("pub struct {name} {{");
    let tuple = format!("pub struct {name}(pub ");
    let enum_decl = format!("pub enum {name} {{");
    for entry in fs::read_dir(model_dir).ok()?.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("rs") {
            continue;
        }
        let src = fs::read_to_string(&path).ok()?;
        if let Some(start) = src.find(&tuple) {
            let rest = &src[start + tuple.len()..];
            let end = rest.find(");")?;
            return Some(CanonShape::Newtype(rest[..end].trim().to_owned()));
        }
        // `pub enum X { Variant(Inner), .. }`: one tuple variant per activity/union arm.
        if let Some(start) = src.find(&enum_decl) {
            let body = &src[start + enum_decl.len()..];
            let end = body.find("\n}")?;
            let mut variants = Vec::new();
            for line in body[..end].lines() {
                let line = line.trim();
                let Some(open) = line.find('(') else { continue };
                let Some(close) = line.find(')') else { continue };
                if close <= open + 1 {
                    continue;
                }
                let variant = line[..open].trim();
                if variant.is_empty() || !variant.chars().all(|c| c.is_alphanumeric() || c == '_') {
                    continue;
                }
                variants.push((variant.to_owned(), line[open + 1..close].trim().to_owned()));
            }
            return Some(CanonShape::Enum(variants));
        }
        let Some(start) = src.find(&brace) else { continue };
        let body = &src[start + brace.len()..];
        let end = body.find("\n}")?;
        let mut fields = Vec::new();
        for line in body[..end].lines() {
            let line = line.trim();
            let Some(rest) = line.strip_prefix("pub ") else { continue };
            let Some((fname, tail)) = rest.split_once(':') else { continue };
            // type is up to the field-terminating comma, ignoring a trailing `// comment`.
            let ty = tail.split("//").next().unwrap_or("").trim().trim_end_matches(',').trim();
            fields.push((fname.trim().to_owned(), ty.to_owned()));
        }
        return Some(CanonShape::Struct(fields));
    }
    None
}

/// Find the raw field a canonical field maps to: a [`FIELD_ALIAS`] override, else normalized name.
pub(crate) fn match_raw<'a>(
    canon_type: &str,
    cfield: &str,
    rfields: &'a [RawField],
) -> Option<&'a RawField> {
    // A type-specific alias wins outright.
    if let Some((_, _, raw_name)) =
        FIELD_ALIAS.iter().find(|(t, f, _)| *t == canon_type && *f == cfield)
    {
        return rfields.iter().find(|f| f.name == *raw_name);
    }
    // Then the normalized name, so a wildcard alias never shadows a real same-named raw field.
    if let Some(rf) = rfields.iter().find(|f| norm(&f.name) == norm(cfield)) {
        return Some(rf);
    }
    // Finally a `"*"` wildcard alias (a Core-wide field rename, e.g. `parent_descs`).
    if let Some((_, _, raw_name)) = FIELD_ALIAS.iter().find(|(t, f, _)| *t == "*" && *f == cfield) {
        return rfields.iter().find(|f| f.name == *raw_name);
    }
    None
}

/// Build the constructor body for a canonical struct. `lookup(cfield) -> (access expr, raw type)`
/// resolves where each canonical field's value comes from (shared by the struct and the
/// newtype->one-field-struct shapes).
fn struct_ctor(
    ctx: &Ctx,
    canon_type: &str,
    cfields: &[(String, String)],
    errs: &mut Vec<(String, String)>,
    numeric: &mut bool,
    nested: &mut Vec<(String, String)>,
    mut lookup: impl FnMut(&str) -> Option<(String, String)>,
) -> String {
    let mut ctor = String::new();
    for (cfield, cty) in cfields {
        // A multi-raw-field reconstruction: emit the hand-written compose expression verbatim.
        //
        // Skip it when the matched raw field is already a typed `Vec<GeneratedItem>`: converting that
        // is the generic nested path's job, and a rule shaped for an untyped `Vec<serde_json::Value>`
        // array (e.g. `listsinceblock.removed`, untyped in v30 but typed in v31) would mis-decode it.
        // Reconstructions that compose from scalar fields are unaffected (their matched field, if
        // any, is not a `Vec` of a generated type).
        let reconstruct_superseded = lookup(cfield).is_some_and(|(_, rty)| {
            let inner = strip_wrap(&rty, "Option<").map(str::to_owned).unwrap_or(rty);
            strip_wrap(&inner, "Vec<").is_some_and(|el| ctx.raw.contains_key(el.trim()))
        });
        if let Some(r) =
            RECONSTRUCT.iter().find(|r| r.canon_type == canon_type && r.field == cfield)
        {
            if !reconstruct_superseded {
                for (v, ty) in r.errs {
                    errs.push(((*v).to_owned(), (*ty).to_owned()));
                }
                if r.numeric {
                    *numeric = true;
                }
                for (raw, canon) in r.nested {
                    nested.push(((*raw).to_owned(), (*canon).to_owned()));
                }
                ctor.push_str(&format!("            {cfield}: {},\n", r.expr));
                continue;
            }
        }
        // Known-buggy canonical field: route through a compatibility shim.
        if let Some(c) = COMPAT.iter().find(|c| c.canon_type == canon_type && c.field == cfield) {
            let access = lookup(cfield).map(|(a, _)| a).unwrap_or_else(|| "()".into());
            ctor.push_str(&format!(
                "            // codegen-correct conversion ({}): `{}`.\n            // TODO(compat): {}. Routed through a wrong placeholder in `compatibility` to compile.\n",
                c.input, c.correct, c.reason
            ));
            ctor.push_str(&format!("            {cfield}: compatibility::{}({access}),\n", c.shim));
            continue;
        }
        match lookup(cfield) {
            Some((access, rty)) => {
                let (expr, known) = convert(ctx, &rty, cty, cfield, &access, errs, numeric, nested);
                if !known {
                    ctor.push_str(&format!(
                        "            // TODO(codegen): no rule for `{rty}` -> `{cty}`.\n"
                    ));
                }
                ctor.push_str(&format!("            {cfield}: {expr},\n"));
            }
            None if cty.starts_with("Option<") => ctor.push_str(&format!(
                "            {cfield}: None, // no raw field; canonical is optional\n"
            )),
            None if cty.starts_with("Vec<") => ctor.push_str(&format!(
                "            {cfield}: vec![], // no raw field; canonical is a list\n"
            )),
            None if cty.starts_with("BTreeMap<") || cty.starts_with("std::collections::BTreeMap<") =>
                ctor.push_str(&format!(
                    "            {cfield}: Default::default(), // no raw field; canonical is a map\n"
                )),
            None => ctor.push_str(&format!(
                "            // TODO(codegen): canonical field `{cfield}: {cty}` has no raw counterpart; needs manual reconstruction.\n            {cfield}: todo!(\"no raw field for `{cfield}`\"),\n"
            )),
        }
    }
    ctor
}

/// Emit one `(raw, canonical)` pair's `into_model` impl plus its error enum. Returns the source, the
/// error-enum name (for the module re-export), and any nested pairs to also emit.
fn emit_type(
    ctx: &Ctx,
    raw_name: &str,
    canon_name: &str,
) -> (String, String, String, Vec<(String, String)>) {
    let err = format!("{canon_name}Error");
    let info = ctx.raw[raw_name];

    let Some(canon) = parse_canonical(ctx.model_dir, canon_name) else {
        return (
            format!("compile_error!(\"codegen: canonical model type `{canon_name}` not found for into_model\");\n\n"),
            String::new(),
            err,
            vec![],
        );
    };

    let mut errs: Vec<(String, String)> = Vec::new();
    let mut numeric = false;
    let mut nested: Vec<(String, String)> = Vec::new();

    let body = if let Some(er) = ENUM_RECONSTRUCT.iter().find(|e| e.canon_type == canon_name) {
        for (v, ty) in er.errs {
            errs.push(((*v).to_owned(), (*ty).to_owned()));
        }
        for (r, c) in er.nested {
            nested.push(((*r).to_owned(), (*c).to_owned()));
        }
        er.body.to_owned()
    } else if let Some(tr) = TYPE_RECONSTRUCT.iter().find(|t| t.canon_type == canon_name) {
        for (v, ty) in tr.errs {
            errs.push(((*v).to_owned(), (*ty).to_owned()));
        }
        format!("Ok(model::{canon_name}({}))", tr.inner_expr)
    } else {
        match (&info.shape, &canon) {
        (RawShape::Struct(rfields), CanonShape::Struct(cfields)) => {
            let ctor =
                struct_ctor(ctx, canon_name, cfields, &mut errs, &mut numeric, &mut nested, |cf| {
                    match_raw(canon_name, cf, rfields)
                        .map(|rf| (format!("self.{}", rf.name), rf.ty.clone()))
                });
            format!("Ok(model::{canon_name} {{\n{ctor}        }})")
        }
        (RawShape::Newtype(rinner), CanonShape::Newtype(cinner)) => {
            let (expr, known) = convert(
                ctx, rinner, cinner, "inner", "self.0", &mut errs, &mut numeric, &mut nested,
            );
            let todo = if known {
                String::new()
            } else {
                format!("// TODO(codegen): no rule for `{rinner}` -> `{cinner}`.\n        ")
            };
            format!("{todo}Ok(model::{canon_name}({expr}))")
        }
        // A raw newtype around a list/scalar feeding a canonical struct with a single field.
        (RawShape::Newtype(rinner), CanonShape::Struct(cfields)) if cfields.len() == 1 => {
            let ctor =
                struct_ctor(ctx, canon_name, cfields, &mut errs, &mut numeric, &mut nested, |_| {
                    Some(("self.0".to_owned(), rinner.clone()))
                });
            format!("Ok(model::{canon_name} {{\n{ctor}        }})")
        }
        // A raw struct feeding a canonical newtype: if the newtype wraps a model struct, build it
        // inline from the raw fields. Otherwise it is a genuine reconstruction (e.g. `Transaction`
        // from decoded fields); leave that honest and half-done so the gap is visible.
        (RawShape::Struct(rfields), CanonShape::Newtype(cinner)) => {
            match parse_canonical(ctx.model_dir, cinner) {
                Some(CanonShape::Struct(cfields)) => {
                    let ctor = struct_ctor(
                        ctx, cinner, &cfields, &mut errs, &mut numeric, &mut nested, |cf| {
                            match_raw(cinner, cf, rfields)
                                .map(|rf| (format!("self.{}", rf.name), rf.ty.clone()))
                        },
                    );
                    format!("Ok(model::{canon_name}(model::{cinner} {{\n{ctor}        }}))")
                }
                _ => format!(
                    "// TODO(codegen): reconstruct `{cinner}` from the decoded fields of `{raw_name}`.\n        // The canonical `{canon_name}` is a newtype around `{cinner}`, but the raw response is the\n        // already-decoded value with no single field to convert. Needs manual assembly.\n        Ok(model::{canon_name}(todo!(\"reconstruct {cinner} from decoded fields\")))"
                ),
            }
        }
        // A raw newtype feeding a multi-field canonical struct can't be bridged structurally.
        (RawShape::Newtype(_), CanonShape::Struct(_)) => format!(
            "compile_error!(\"codegen: cannot map newtype `{raw_name}` onto multi-field struct `{canon_name}`\");\n        unreachable!()"
        ),
        // An untagged raw enum onto the canonical enum: convert each variant's inner via its own
        // `into_model`, pairing raw and canonical variants by declaration order. The inner pairs are
        // queued so their `into_model` is emitted too.
        (RawShape::Enum(rvariants), CanonShape::Enum(cvariants)) => {
            let mut arms = String::new();
            for ((rv, rinner), (cv, cinner)) in rvariants.iter().zip(cvariants.iter()) {
                nested.push((rinner.clone(), cinner.clone()));
                errs.push((cv.clone(), format!("{cinner}Error")));
                arms.push_str(&format!(
                    "            {raw_name}::{rv}(x) => model::{canon_name}::{cv}(x.into_model().map_err(E::{cv})?),\n"
                ));
            }
            format!("Ok(match self {{\n{arms}        }})")
        }
        // Any other shape pairing (a raw enum onto a struct/newtype, etc.) has no structural bridge.
        _ => format!(
            "compile_error!(\"codegen: cannot bridge the shape of `{raw_name}` onto `{canon_name}`\");\n        unreachable!()"
        ),
        }
    };

    let mut imp = String::new();
    imp.push_str(&format!("impl {raw_name} {{\n"));
    imp.push_str("    /// Converts the raw type into the version-nonspecific model type.\n");
    imp.push_str(&format!(
        "    pub fn into_model(self) -> Result<model::{canon_name}, {err}> {{\n"
    ));
    imp.push_str(&format!("        use {err} as E;\n\n"));
    imp.push_str(&format!("        {body}\n    }}\n}}\n\n"));

    // A reconstruction may reference one error variant from several field expressions (the witness
    // compose declares `WitnessVersion` from both `witness_version` and `witness_program`). Keep the
    // first of each name so the emitted error enum has no duplicate variants.
    let mut seen_variants: BTreeSet<String> = BTreeSet::new();
    errs.retain(|(v, _)| seen_variants.insert(v.clone()));

    let error_enum = emit_error_enum(&err, canon_name, &errs, numeric);
    (imp, error_enum, err, nested)
}

/// Emit the error enum for a converted type: one variant per named parse error, plus a shared
/// `Numeric` if any narrowing happened.
pub(crate) fn emit_error_enum(
    err: &str,
    canon: &str,
    named: &[(String, String)],
    numeric: bool,
) -> String {
    let mut out = String::new();
    out.push_str(&format!("/// Error when converting a `{canon}` type into the model type.\n"));
    out.push_str("#[derive(Debug)]\n");
    out.push_str(&format!("pub enum {err} {{\n"));
    for (variant, ty) in named {
        out.push_str(&format!(
            "    /// Conversion of the `{variant}` field failed.\n    {variant}({ty}),\n"
        ));
    }
    if numeric {
        out.push_str("    /// Conversion of a numeric type to the expected type failed.\n    Numeric(crate::NumericError),\n");
    }
    out.push_str("}\n\n");

    // An infallible conversion has an uninhabited (empty) error enum, so `match *self {}` never
    // touches the formatter; name it `_f` to avoid an unused-variable warning.
    let fparam = if named.is_empty() && !numeric { "_f" } else { "f" };
    out.push_str(&format!(
        "impl fmt::Display for {err} {{\n    fn fmt(&self, {fparam}: &mut fmt::Formatter) -> fmt::Result {{\n        match *self {{\n"
    ));
    for (variant, _) in named {
        out.push_str(&format!(
            "            Self::{variant}(ref e) => write_err!(f, \"conversion of the `{variant}` field failed\"; e),\n"
        ));
    }
    if numeric {
        out.push_str("            Self::Numeric(ref e) => write_err!(f, \"numeric conversion failed\"; e),\n");
    }
    out.push_str("        }\n    }\n}\n\n");

    out.push_str(&format!(
        "#[cfg(feature = \"std\")]\nimpl std::error::Error for {err} {{\n    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {{\n        match *self {{\n"
    ));
    for (variant, _) in named {
        out.push_str(&format!("            Self::{variant}(ref e) => Some(e),\n"));
    }
    if numeric {
        out.push_str("            Self::Numeric(ref e) => Some(e),\n");
    }
    out.push_str("        }\n    }\n}\n\n");

    if numeric {
        out.push_str(&format!(
            "impl From<crate::NumericError> for {err} {{\n    fn from(e: crate::NumericError) -> Self {{ Self::Numeric(e) }}\n}}\n\n"
        ));
    }
    out
}

/// The result of generating a category's `into.rs`.
pub struct Generated {
    pub source: String,
    /// Error-enum names to re-export from the category `mod.rs`.
    pub error_names: Vec<String>,
    /// Whether the category had any root to generate (else it stays a flat module).
    pub has_roots: bool,
}

/// Generate the `into.rs` for one category: walk the type graph from every root in `all` (every
/// response type in the category), emitting each reachable `(raw, canonical)` pair.
pub fn generate_category(
    version: &str,
    category: &str,
    all: &[RawTypeInfo],
    model_dir: &Path,
) -> Generated {
    let ctx = Ctx { raw: all.iter().map(|i| (i.raw_name.clone(), i)).collect(), model_dir };

    // Generate every response type that has a `crate::model` counterpart. A raw type whose canonical
    // name has no model (control RPCs, `EnumerateSigners`, ...) simply has nothing to convert into.
    let mut queue: VecDeque<(String, String)> = all
        .iter()
        .filter(|i| {
            let canon = model_typename(&i.raw_name);
            match parse_canonical(model_dir, &canon) {
                None => false,
                // A raw oneOf-union enum (e.g. `GetTxOut { Null, Object(..) }`) maps to a
                // struct/newtype model through its variant alias, not as itself; seed a raw enum
                // root only when the canonical is itself an enum (a real data union like
                // `ActivityEntry`). Such union enums are bridged via their aliased variant type.
                Some(c) =>
                    !matches!(i.shape, RawShape::Enum(_)) || matches!(c, CanonShape::Enum(_)),
            }
        })
        .map(|i| (i.raw_name.clone(), model_typename(&i.raw_name)))
        .collect();
    let has_roots = !queue.is_empty();

    let mut seen: BTreeSet<String> = BTreeSet::new();
    let mut emitted: Vec<(String, String)> = Vec::new(); // (raw_name, source)
    let mut error_names: Vec<String> = Vec::new();
    // Several raw types can map to the same canonical type (e.g. the ancestor/descendant verbose
    // mempool entries both become `MempoolEntry`); their error enum must be emitted only once.
    let mut emitted_errors: BTreeSet<String> = BTreeSet::new();

    while let Some((raw, canon)) = queue.pop_front() {
        if !seen.insert(raw.clone()) {
            continue;
        }
        if !ctx.raw.contains_key(&raw) {
            continue; // a nested type outside this category's response set; skip.
        }
        let (imp, error_enum, err, nested) = emit_type(&ctx, &raw, &canon);
        let mut src = imp;
        if emitted_errors.insert(err.clone()) {
            src.push_str(&error_enum);
            error_names.push(err);
        }
        emitted.push((raw, src));
        for pair in nested {
            queue.push_back(pair);
        }
    }

    emitted.sort_by(|a, b| a.0.cmp(&b.0));
    error_names.sort();

    let mut source = header(version, category);
    for (_, src) in emitted {
        source.push_str(&src);
    }

    Generated { source, error_names, has_roots }
}

/// The `into.rs` file header (module docs + imports).
fn header(version: &str, category: &str) -> String {
    format!(
        "// SPDX-License-Identifier: CC0-1.0\n\n\
         //! Auto-generated `into_model` conversions for Bitcoin Core `{version}` - {category}.\n//!\n\
         //! Generated by `codegen`. Do not edit by hand, re-run `just codegen` to regenerate. Each\n\
         //! conversion turns a raw response type (from the sibling `mod.rs`) into its shared,\n\
         //! version-nonspecific `crate::model` counterpart. Conversions the canonical types get\n\
         //! wrong are routed through `crate::v{version}::generated::compatibility`; see that file and\n\
         //! `corepc_bugs_backlog.md`.\n\n\
         // `unreachable_code` is allowed for the half-done `todo!()` reconstructions (see the\n\
         // per-site TODOs); they diverge, making later struct fields unreachable.\n\
         #![allow(unreachable_code, unused_imports, unused_variables)]\n\n\
         use core::fmt;\n\n\
         use bitcoin::address::{{self, NetworkUnchecked}};\n\
         use bitcoin::bip32::{{self, Xpriv, Xpub}};\n\
         use bitcoin::consensus::encode;\n\
         use bitcoin::error::UnprefixedHexError;\n\
         use bitcoin::hashes::{{hash160, sha256}};\n\
         use bitcoin::hex::FromHex as _;\n\
         use bitcoin::key::{{self, PrivateKey, PublicKey}};\n\
         use bitcoin::sign_message;\n\
         use bitcoin::{{amount, block, hex, network, psbt, witness_program, witness_version, Address, Amount, Block, BlockHash, CompactTarget, FeeRate, Network, OutPoint, Psbt, ScriptBuf, Sequence, SignedAmount, Target, Transaction, TxMerkleNode, TxOut, Txid, Weight, WitnessProgram, WitnessVersion, Work, Wtxid}};\n\n\
         use super::*;\n\
         use crate::error::write_err;\n\
         use crate::model;\n\
         use crate::v{version}::generated::compatibility;\n\n"
    )
}

/// Emit the shared `compatibility.rs`: the hand-shaped wrong-but-compilable shims that isolate
/// canonical-type bugs. Same content for every version (the override table lives in the generator).
pub fn emit_compatibility(version: &str) -> String {
    let mut s = String::new();
    s.push_str(&format!(
        "// SPDX-License-Identifier: CC0-1.0\n\n\
         //! Manual compatibility shims for `into_model` conversions that `codegen` produces\n\
         //! correctly but that do not match the (buggy) canonical `crate::model` types.\n//!\n\
         //! Emitted by `codegen` from a fixed override table, so every version carries the same\n\
         //! shims. Each one is a deliberate WRONG, compilable placeholder that isolates a known bug\n\
         //! in `types/` so the generated code keeps building. Each has a matching entry in\n\
         //! `corepc_bugs_backlog.md`. When a canonical type is fixed, drop the override and let\n\
         //! codegen emit the correct conversion inline (left commented at each call site).\n//!\n\
         //! Bitcoin Core `{version}`.\n\n\
         #![allow(unused_imports)]\n\n\
         use bitcoin::{{Address, Amount, BlockHash, FeeRate}};\n\n"
    ));
    for c in COMPAT {
        s.push_str(&format!(
            "/// WRONG placeholder: {reason}.\n///\n/// Codegen produces `{correct}` ({input}); the canonical field wrongly wants `{returns}`, so this\n/// discards the real value to compile. TODO: fix the canonical type, then delete this shim.\npub fn {shim}(_v: {input}) -> {returns} {{ {placeholder} }}\n\n",
            reason = c.reason,
            correct = c.correct,
            input = c.input,
            returns = c.returns,
            shim = c.shim,
            placeholder = c.placeholder,
        ));
    }
    s
}
