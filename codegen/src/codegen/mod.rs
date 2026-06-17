// SPDX-License-Identifier: CC0-1.0

//! Translate a parsed OpenRPC [`Spec`] into Rust source for the output files. The generator IR
//! ([`Modules`] and friends) lives here; [`lower`] builds it from the spec, [`emit`] writes it out.

use serde_json::Value;

mod blocking;
mod emit;
mod lower;

pub use self::lower::lower;

/// One emitted Rust definition; `nested` holds helper types it depends on.
#[derive(Debug)]
pub(crate) struct GenType {
    pub(crate) name: String,
    pub(crate) body: String,
    pub(crate) nested: Vec<GenType>,
    pub(crate) ir: TypeIr,
}

/// Response types live in `corepc-types` with model counterparts; Param types live in `corepc-client`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Origin {
    Response,
    Param,
}

/// Shape of a generated raw type, handed to `crate::into_model` (as `RawShape`) so it can emit
/// conversions without re-walking the spec.
#[derive(Debug, Clone)]
pub(crate) enum TypeIr {
    /// `pub struct Name { fields }`.
    Struct(Vec<FieldIr>),
    /// `pub struct Name(pub inner);`, the inner Rust type string.
    Newtype(String),
    /// `pub enum Name { Variant(inner), ... }`, `(variant ident, inner Rust type)` pairs.
    Enum(Vec<(String, String)>),
    /// `pub type Name = Target;`: a response that reuses another type's `into_model`, so it produces
    /// no `RawTypeInfo` of its own.
    Alias,
}

/// One field of a generated struct, handed to `crate::into_model`; `rust_type` is pre-`Option` wrap.
#[derive(Debug, Clone)]
pub(crate) struct FieldIr {
    pub(crate) rust_name: String,
    pub(crate) rust_type: String,
    pub(crate) optional: bool,
}

/// Output bundle: every type, method, and options struct, ready to be written to disk.
pub struct Modules {
    pub(crate) types: Vec<(String, Origin, GenType)>,
    pub(crate) methods: Vec<MethodOut>,
}

#[derive(Debug)]
pub(crate) struct MethodOut {
    pub(crate) method_name: String, // wire name (`getblockheader`)
    pub(crate) snake: String,       // Rust fn name (`get_block_header`)
    pub(crate) pascal: String,      // Rust return-type ident (`GetBlockHeader`)
    pub(crate) description: String,
    pub(crate) return_type: String,
    pub(crate) params: Vec<ParamOut>,
    pub(crate) category: String,
    /// True when the method's sole optional parameter is a JSON object whose properties have been
    /// flattened into the `*Options` struct. The struct is then sent as one positional argument
    /// rather than its fields being spread.
    pub(crate) object_options: bool,
    /// Non-empty when the result is a verbose-selected union: instead of one method, emit one
    /// method per variant, each returning a concrete type and hardcoding the selector argument.
    pub(crate) verbose_variants: Vec<VerboseVariant>,
    /// Positional index of the selector within the original spec params, set only for verbose
    /// methods. The selector is dropped from `params`, so the emitter reinserts its hardcoded
    /// literal at this index.
    pub(crate) selector_idx: usize,
}

#[derive(Debug)]
pub(crate) struct ParamOut {
    pub(crate) rust_name: String,
    pub(crate) wire_name: String, // JSON key, for serde rename when an options object is serialised whole
    pub(crate) rust_type: String,
    pub(crate) description: String,
    pub(crate) required: bool,
    pub(crate) default: Option<Value>,
}

/// One variant of a verbose-selected union return.
#[derive(Debug, Clone)]
pub(crate) struct VerboseVariant {
    pub(crate) word: String,      // verbosity level digit: "0", "1", ...
    pub(crate) type_name: String, // concrete variant return type, e.g. "GetBlockVerbose1"
    pub(crate) selector: String, // JSON literal for the selector argument: "0", "1", "true", "false"
}

impl Modules {
    /// Number of generated types across all categories.
    pub fn types_count(&self) -> usize { self.types.len() }
    /// Number of generated RPC methods.
    pub fn methods_count(&self) -> usize { self.methods.len() }
    /// Number of methods that emit an `*Options` struct.
    pub fn option_count(&self) -> usize { self.methods.iter().filter(|m| m.has_optional()).count() }
}

impl MethodOut {
    /// Whether this method has at least one optional parameter.
    pub(crate) fn has_optional(&self) -> bool { self.params.iter().any(|p| !p.required) }
    /// Name of the `*Options` struct for the `_with` variant of this method.
    pub(crate) fn options_struct_name(&self) -> String { format!("{}Options", self.pascal) }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::emit::*;
    use super::lower::*;
    use super::*;
    use crate::spec::{Method, Schema, Spec};

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
