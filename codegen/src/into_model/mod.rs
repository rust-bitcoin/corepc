// SPDX-License-Identifier: CC0-1.0

//! Generates the `into_model` conversions from the raw response types into the shared,
//! version-nonspecific `crate::model` types in `corepc-types`.
//!
//! This is the codegen counterpart of the hand-written `vN/<category>/into.rs` files. For every
//! response type (and every nested type it reaches) it emits
//! `impl RawType { fn into_model(self) -> Result<model::T, E> }` plus the error enum.
//!
//! # How it decides each conversion
//!
//! The generator does NOT hand-encode conversions per RPC. It reads two shapes and bridges them:
//!
//! 1. the RAW type's fields (name + Rust type) from the spec-derived IR, and
//! 2. the CANONICAL `crate::model` type's fields, parsed from the `corepc-types` model source.
//!
//! Fields are matched by a normalized name key (so `chainwork` <-> `chain_work`). The model field
//! name and the target type come from the canonical struct, so per-RPC quirks (a field made
//! `Option`, a field dropped, a field strongly typed) need no special casing: they fall out of
//! whatever the canonical type declares. The conversion expression itself is chosen by generic
//! type-pair rules in [`leaf`] (`String -> BlockHash`, `i64 -> u32`, `f64 -> Amount`, ...) with
//! [`convert`] handling the `Option`/`Vec`/`BTreeMap` wrappers around them.
//!
//! # Nested types
//!
//! When a canonical field is itself a generated type (e.g. `Vec<MempoolAcceptance>`), the matching
//! raw field is a generated type too (`Vec<TestMempoolAcceptItem>`). The pairing comes from the
//! PARENT field, not from a name match: the element is converted with `.into_model()` and the
//! `(raw, canonical)` pair is queued so its own `into_model` is emitted. This walks the whole type
//! graph from each root without any nested-type name table.
//!
//! # Escape hatches
//!
//! - A raw field with no canonical counterpart is dropped; an unmatched canonical `Option`/`Vec`
//!   defaults to `None`/`vec![]`, any other unmatched canonical field becomes a `todo!()`.
//! - A handful of top-level entry points whose raw name diverges from the canonical name (verbose
//!   levels, RPC aliases) are listed in [`TYPE_ALIAS`]; a handful of semantic field renames in
//!   [`FIELD_ALIAS`]. These are RPC-level naming choices, not per-field conversion tables.
//! - A `(raw, canonical)` pair the canonical type gets *wrong* is listed in [`COMPAT`] and routed to
//!   a hand-shaped placeholder in `compatibility.rs`, isolating the bug while the crate still builds
//!   (see `corepc_bugs_backlog.md`).
//!
//! Every response type with a `crate::model` counterpart is generated; a raw type whose canonical
//! name has no model simply has nothing to convert into and is skipped. The compile gate against the
//! real `crate::model` catches any wrong field name or type.

use std::collections::BTreeMap;
use std::path::Path;

mod convert;
mod emit;
mod tables;

pub use self::emit::{emit_compatibility, generate_category};
use self::tables::TYPE_ALIAS;

/// One field of a raw response struct, handed to the generator by `codegen`.
pub struct RawField {
    pub name: String,
    /// The full Rust type including any `Option<...>`/`Vec<...>` wrapper.
    pub ty: String,
}

/// The shape of a raw response type.
pub enum RawShape {
    /// `pub struct X { .. }`.
    Struct(Vec<RawField>),
    /// `pub struct X(pub Inner);`, the inner Rust type.
    Newtype(String),
    /// `#[serde(untagged)] pub enum X { Variant(Inner), .. }`, `(variant ident, inner type)` pairs.
    Enum(Vec<(String, String)>),
}

/// A raw response type and its shape.
pub struct RawTypeInfo {
    pub raw_name: String,
    pub shape: RawShape,
}

/// Per-category lookup the generator threads through the type-graph walk.
pub(crate) struct Ctx<'a> {
    /// Every response type in the category by raw name; membership marks a "generated" type, which
    /// is how a nested field is told apart from a primitive.
    pub(crate) raw: BTreeMap<String, &'a RawTypeInfo>,
    pub(crate) model_dir: &'a Path,
}

/// The canonical `crate::model` type name for a raw type name. [`TYPE_ALIAS`] first, then the
/// verbose digit->word rewrite (`GetBlockVerbose1` -> `GetBlockVerboseOne`), else identity.
pub(crate) fn model_typename(raw: &str) -> String {
    if let Some((_, canon)) = TYPE_ALIAS.iter().find(|(r, _)| *r == raw) {
        return (*canon).to_owned();
    }
    let mut s = raw.to_owned();
    for (digit, word) in [
        ("Verbose0", "VerboseZero"),
        ("Verbose1", "VerboseOne"),
        ("Verbose2", "VerboseTwo"),
        ("Verbose3", "VerboseThree"),
    ] {
        s = s.replace(digit, word);
    }
    s
}

/// Normalized name key for matching a raw field to a canonical field (`chain_work` -> `chainwork`).
pub(crate) fn norm(name: &str) -> String {
    name.chars().filter(|c| *c != '_').flat_map(char::to_lowercase).collect()
}

/// PascalCase an error-variant name from a snake_case field (`chain_work` -> `ChainWork`).
pub(crate) fn variant_of(field: &str) -> String {
    field
        .split('_')
        .map(|seg| {
            let mut c = seg.chars();
            match c.next() {
                Some(first) => first.to_ascii_uppercase().to_string() + c.as_str(),
                None => String::new(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::convert::*;
    use super::emit::*;
    use super::*;

    /// A `Ctx` with no generated types, for `leaf`/`convert` rules that never read the model dir.
    fn empty_ctx() -> Ctx<'static> { Ctx { raw: BTreeMap::new(), model_dir: Path::new(".") } }

    // == name + type-string helpers ==

    #[test]
    fn norm_drops_underscores_and_lowercases() {
        assert_eq!(norm("chain_work"), "chainwork");
        assert_eq!(norm("ChainWork"), "chainwork");
        assert_eq!(norm("a_b_c"), "abc");
        // The whole point: a wire name and a snake name collapse to the same key.
        assert_eq!(norm("chainwork"), norm("chain_work"));
    }

    #[test]
    fn variant_of_pascal_cases_each_segment() {
        assert_eq!(variant_of("chain_work"), "ChainWork");
        assert_eq!(variant_of("vout"), "Vout");
        assert_eq!(variant_of("a_b_c"), "ABC");
    }

    #[test]
    fn model_typename_alias_then_verbose_then_identity() {
        // A `TYPE_ALIAS` entry wins outright.
        assert_eq!(model_typename("GetRawMempool"), "GetRawMempoolResult");
        assert_eq!(model_typename("GetBlockHeaderVerbose0"), "GetBlockHeader");
        // Else the verbose digit -> word rewrite.
        assert_eq!(model_typename("GetBlockVerbose1"), "GetBlockVerboseOne");
        assert_eq!(model_typename("GetBlockVerbose2"), "GetBlockVerboseTwo");
        // Else identity.
        assert_eq!(model_typename("GetBlockchainInfo"), "GetBlockchainInfo");
    }

    #[test]
    fn strip_wrap_unwraps_one_generic_layer() {
        assert_eq!(strip_wrap("Option<Foo>", "Option<"), Some("Foo"));
        assert_eq!(strip_wrap("Option<Vec<u8>>", "Option<"), Some("Vec<u8>"));
        assert_eq!(strip_wrap("Vec<Foo>", "Option<"), None);
    }

    #[test]
    fn map_pair_splits_on_the_top_level_comma() {
        assert_eq!(map_pair("BTreeMap<String, u64>"), Some(("String", "u64")));
        assert_eq!(
            map_pair("std::collections::BTreeMap<String, Vec<Txid>>"),
            Some(("String", "Vec<Txid>"))
        );
        // Depth-aware: the inner map's comma must not split the outer pair.
        assert_eq!(
            map_pair("BTreeMap<String, BTreeMap<String, u8>>"),
            Some(("String", "BTreeMap<String, u8>"))
        );
        assert_eq!(map_pair("Vec<u8>"), None);
    }

    #[test]
    fn fn_path_only_matches_a_bare_function_call() {
        assert_eq!(fn_path("Some(x)", "x"), Some("Some"));
        assert_eq!(fn_path("crate::to_u32(x)", "x"), Some("crate::to_u32"));
        assert_eq!(fn_path("y.method(x)", "x"), None); // method call, not a path
        assert_eq!(fn_path("f(&x)", "x"), None); // takes a reference
        assert_eq!(fn_path("foo(y)", "x"), None); // different variable
    }

    #[test]
    fn has_default_covers_scalars_amounts_and_vecs_only() {
        assert!(has_default("u32"));
        assert!(has_default("Amount"));
        assert!(has_default("SignedAmount"));
        assert!(has_default("Vec<Txid>"));
        assert!(!has_default("BlockHash"));
        assert!(!has_default("Address<NetworkUnchecked>"));
    }

    // == field matching ==

    #[test]
    fn match_raw_alias_then_normalized_then_wildcard() {
        let rfields = vec![
            RawField { name: "chainwork".into(), ty: "String".into() },
            RawField { name: "hex".into(), ty: "String".into() },
            RawField { name: "desc".into(), ty: "String".into() },
        ];
        let name = |o: Option<&RawField>| o.map(|f| f.name.clone());

        // Normalized-name match (`chain_work` <-> `chainwork`).
        assert_eq!(name(match_raw("Whatever", "chain_work", &rfields)), Some("chainwork".into()));
        // Type-specific `FIELD_ALIAS`: `GetTransaction.tx` reads the raw `hex` field.
        assert_eq!(name(match_raw("GetTransaction", "tx", &rfields)), Some("hex".into()));
        // Core-wide `*` wildcard alias: `descriptor` reads the raw `desc` field.
        assert_eq!(name(match_raw("Whatever", "descriptor", &rfields)), Some("desc".into()));
        // No counterpart.
        assert!(match_raw("Whatever", "no_such_field", &rfields).is_none());
    }

    // == leaf type-pair rules ==

    #[test]
    fn leaf_picks_the_right_conversion_per_type_pair() {
        let ctx = empty_ctx();
        let mut nested = vec![];

        let (expr, err, known) = leaf(&ctx, "String", "BlockHash", "v", "\"f\"", &mut nested);
        assert!(expr.contains("v.parse::<BlockHash>()"));
        assert!(matches!(err, Some(LeafErr::Named(_))));
        assert!(known);

        let (expr, err, _) = leaf(&ctx, "f64", "Amount", "v", "\"f\"", &mut nested);
        assert!(expr.contains("Amount::from_btc(v)"));
        assert!(matches!(err, Some(LeafErr::Named(_))));

        // Integer amounts are satoshis and infallible.
        let (expr, err, _) = leaf(&ctx, "i64", "Amount", "v", "\"f\"", &mut nested);
        assert!(expr.contains("Amount::from_sat"));
        assert!(err.is_none());

        // Numeric narrowing reports the shared `Numeric` error.
        let (expr, err, _) = leaf(&ctx, "i64", "u32", "v", "\"f\"", &mut nested);
        assert!(expr.contains("crate::to_u32"));
        assert!(matches!(err, Some(LeafErr::Numeric)));

        // Same type on both sides is a move, no error.
        let (expr, err, known) = leaf(&ctx, "String", "String", "v", "\"f\"", &mut nested);
        assert_eq!(expr, "v");
        assert!(err.is_none() && known);

        // An unhandled pair is left as an honest `todo!()` and flagged unknown.
        let (expr, _err, known) = leaf(&ctx, "Foo", "Bar", "v", "\"f\"", &mut nested);
        assert!(expr.contains("todo!"));
        assert!(!known);

        assert!(nested.is_empty(), "leaf rules above pull in no nested types");
    }

    #[test]
    fn leaf_delegates_a_generated_type_to_its_into_model_and_queues_it() {
        let item = RawTypeInfo { raw_name: "MyItem".into(), shape: RawShape::Struct(vec![]) };
        let mut raw = BTreeMap::new();
        raw.insert("MyItem".to_owned(), &item);
        let ctx = Ctx { raw, model_dir: Path::new(".") };

        let mut nested = vec![];
        let (expr, err, known) = leaf(&ctx, "MyItem", "MyModel", "x", "\"f\"", &mut nested);
        assert_eq!(expr, "x.into_model()");
        assert!(matches!(err, Some(LeafErr::Named(ref s)) if s == "MyModelError"));
        assert!(known);
        // The element pair is queued so its own `into_model` gets emitted by the graph walk.
        assert_eq!(nested, vec![("MyItem".to_owned(), "MyModel".to_owned())]);
    }

    // == convert: Option / Vec / Map / defaulting wrappers ==

    #[test]
    fn convert_threads_wrappers_around_a_leaf() {
        let ctx = empty_ctx();
        let mut errs = vec![];
        let mut numeric = false;
        let mut nested = vec![];
        let mut go = |raw: &str, canon: &str, field: &str| {
            convert(
                &ctx,
                raw,
                canon,
                field,
                &format!("self.{field}"),
                &mut errs,
                &mut numeric,
                &mut nested,
            )
            .0
        };

        // Option<String> -> Option<BlockHash>: map the inner, lift the error with transpose.
        let e = go("Option<String>", "Option<BlockHash>", "hash");
        assert!(e.contains(".map(") && e.contains("transpose"));

        // Vec<String> -> Vec<Txid>: per-element parse then collect.
        let e = go("Vec<String>", "Vec<Txid>", "txids");
        assert!(e.contains("into_iter().map(") && e.contains("collect"));

        // R -> Option<U>: convert then wrap in `Some`.
        let e = go("String", "Option<BlockHash>", "h");
        assert!(e.starts_with("Some("));

        // BTreeMap value conversion (key identity, value narrows).
        let e = go("BTreeMap<String, i64>", "BTreeMap<String, u32>", "m");
        assert!(e.contains("into_iter().map(|(k, v)|"));
    }

    #[test]
    fn convert_absent_value_defaults_or_errors_by_canonical_type() {
        let ctx = empty_ctx();
        let mut numeric = false;
        let mut nested = vec![];

        // Option<R> -> bare U where U has a `Default`: fill it in.
        let mut errs = vec![];
        let (e, _) = convert(
            &ctx,
            "Option<String>",
            "String",
            "s",
            "self.s",
            &mut errs,
            &mut numeric,
            &mut nested,
        );
        assert!(e.contains("unwrap_or_default"));

        // Option<R> -> bare U with no meaningful default (a hash): a dropped field is an error.
        let mut errs = vec![];
        let (e, _) = convert(
            &ctx,
            "Option<String>",
            "BlockHash",
            "h",
            "self.h",
            &mut errs,
            &mut numeric,
            &mut nested,
        );
        assert!(e.contains("MissingField"));
        assert!(errs.iter().any(|(v, _)| v == "HMissing"));
    }

    // == error enum emission ==

    #[test]
    fn emit_error_enum_infallible_is_uninhabited() {
        let src = emit_error_enum("FooError", "Foo", &[], false);
        assert!(src.contains("pub enum FooError {"));
        // No variants and no numeric: the formatter param is unused, named `_f`.
        assert!(src.contains("_f: &mut fmt::Formatter"));
        assert!(!src.contains("crate::NumericError"));
    }

    #[test]
    fn emit_error_enum_named_and_numeric() {
        let named = vec![("Bits".to_owned(), "UnprefixedHexError".to_owned())];
        let src = emit_error_enum("BarError", "Bar", &named, true);
        assert!(src.contains("Bits(UnprefixedHexError)"));
        assert!(src.contains("Numeric(crate::NumericError)"));
        assert!(src.contains("impl From<crate::NumericError> for BarError"));
        // With at least one variant the formatter param is used.
        assert!(src.contains("f: &mut fmt::Formatter"));
    }

    // == end-to-end: generate_category over a synthetic model dir ==

    /// A fresh temp dir holding a single `model.rs` with `contents`, for `generate_category`.
    fn model_dir_with(contents: &str) -> PathBuf {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        let n = COUNTER.fetch_add(1, Ordering::Relaxed);
        let dir =
            std::env::temp_dir().join(format!("corepc_codegen_test_{}_{n}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("model.rs"), contents).unwrap();
        dir
    }

    #[test]
    fn generate_category_emits_struct_conversion_and_walks_nested_types() {
        // `Foo` has a model; `Item` is reached only through `Foo.items`'s element type.
        let dir = model_dir_with(
            "pub struct Foo {\n    pub block_hash: BlockHash,\n    pub count: u32,\n    pub items: Vec<Item>,\n}\npub struct Item {\n    pub id: Txid,\n}\n",
        );
        let foo = RawTypeInfo {
            raw_name: "Foo".into(),
            shape: RawShape::Struct(vec![
                RawField { name: "block_hash".into(), ty: "String".into() },
                RawField { name: "count".into(), ty: "i64".into() },
                RawField { name: "items".into(), ty: "Vec<FooItem>".into() },
            ]),
        };
        let item = RawTypeInfo {
            raw_name: "FooItem".into(),
            shape: RawShape::Struct(vec![RawField { name: "id".into(), ty: "String".into() }]),
        };

        let generated = generate_category("30", "blockchain", &[foo, item], &dir);
        let s = &generated.source;

        assert!(generated.has_roots);
        // Root struct: each field picks its rule (String->BlockHash parse, i64->u32 narrow).
        assert!(s.contains("impl Foo {"));
        assert!(s.contains("pub fn into_model(self) -> Result<model::Foo, FooError>"));
        assert!(s.contains("self.block_hash.parse::<BlockHash>()"));
        assert!(s.contains("crate::to_u32(self.count"));
        // The nested element type is emitted even though it is not a root (no `FooItem` model): the
        // pairing comes from `Foo.items: Vec<Item>` <- raw `Vec<FooItem>`.
        assert!(s.contains("impl FooItem {"));
        assert!(s.contains("pub fn into_model(self) -> Result<model::Item, ItemError>"));
        assert!(s.contains("self.id.parse::<Txid>()"));
        // Both error enums are reported for re-export.
        assert!(generated.error_names.contains(&"FooError".to_owned()));
        assert!(generated.error_names.contains(&"ItemError".to_owned()));

        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn generate_category_handles_a_newtype() {
        let dir = model_dir_with("pub struct GetThing(pub BlockHash);\n");
        let raw =
            RawTypeInfo { raw_name: "GetThing".into(), shape: RawShape::Newtype("String".into()) };

        let generated = generate_category("30", "blockchain", &[raw], &dir);
        let s = &generated.source;

        assert!(generated.has_roots);
        assert!(s.contains("impl GetThing {"));
        assert!(s.contains("Ok(model::GetThing("));
        assert!(s.contains("self.0.parse::<BlockHash>()"));

        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn generate_category_skips_a_raw_type_with_no_model() {
        // The only reason a response type is skipped: its canonical name has no `crate::model`
        // counterpart, so there is nothing to convert into.
        let dir = model_dir_with("pub struct Unrelated(pub u8);\n");
        let raw = RawTypeInfo {
            raw_name: "NoModelHere".into(),
            shape: RawShape::Newtype("String".into()),
        };

        let generated = generate_category("30", "blockchain", &[raw], &dir);

        assert!(!generated.has_roots);
        assert!(!generated.source.contains("impl NoModelHere"));

        fs::remove_dir_all(&dir).unwrap();
    }
}
