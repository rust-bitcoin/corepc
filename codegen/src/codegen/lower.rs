// SPDX-License-Identifier: CC0-1.0

//! Front-to-middle: lower a parsed OpenRPC [`Spec`] into the generator's IR ([`Modules`]) and the
//! raw response-type source it carries. Schema-shape dispatch, type naming, number classification,
//! and the small override tables the spec can't express.

use std::collections::BTreeSet;

use super::emit::{esc_doc, fmt_doc, method_doc, param_description};
use super::*;
use crate::names::{method_to_pascal, method_to_snake, to_pascal, to_rust_field};
use crate::spec::{AdditionalProperties, Method, Param, Schema, SchemaType, Spec};

const DERIVES: &str = "#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]\n\
     #[cfg_attr(feature = \"serde-deny-unknown-fields\", serde(deny_unknown_fields))]";

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

/// Map a PascalCase method name to its generated type name, avoiding std/prelude collisions.
pub(crate) fn safe_type_name(pascal: &str) -> String {
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
    Modules { types, methods }
}

/// Whether a schema is a JSON object carrying real (non-commentary) properties.
fn is_object_with_props(schema: &Schema) -> bool {
    schema.primary_kind() == Some("object") && schema.properties.is_some() && schema.has_props()
}

/// Lower a method's parameters into [`ParamOut`]s and any helper types they spawn.
pub(crate) fn lower_params(
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

/// Whether a parameter name is the verbosity selector.
fn is_selector(name: &str) -> bool {
    let n = name.to_ascii_lowercase();
    n == "verbose" || n == "verbosity"
}

/// Return per-variant methods if the result is cleanly selected by a `verbose`/`verbosity` param, else `None`.
pub(crate) fn verbose_variants(method: &Method) -> Option<Vec<VerboseVariant>> {
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
pub(crate) fn return_type_ident(method: &Method) -> String {
    let s = &method.result.schema;
    if s.returns_null() {
        return "()".to_owned();
    }
    safe_type_name(&method_to_pascal(&method.name))
}

/// Generate the top-level return type for a method, or `None` if the method returns null.
pub(crate) fn generate_return_type(
    method: &Method,
    seen: &mut BTreeSet<String>,
) -> Option<GenType> {
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
pub(crate) fn enum_type(
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

/// A hand-specified field for a [`StructOverride`].
struct OverrideField {
    /// Rust field name.
    rust_name: &'static str,
    /// Wire name when it differs from `rust_name` (emitted as `#[serde(rename = "..")]`).
    rename: Option<&'static str>,
    /// Inner Rust type, before any `Option<..>` wrap.
    ty: &'static str,
    /// Whether the field is optional (`Option`-wrapped, skipped on serialize when none).
    optional: bool,
    /// Field rustdoc.
    doc: &'static str,
}

/// A raw struct whose shape the OpenRPC spec leaves underspecified - an empty object with
/// `additionalProperties: true` - so codegen has nothing to derive fields from.
///
/// `getaddressinfo`'s recursive `embedded` object is the case in point: Core ships it as a typeless
/// `{}` in the spec, so the schema-driven path emits a `flatten` catch-all that cannot feed the
/// strongly typed `model::GetAddressInfoEmbedded`. We hand-specify the field set Core actually
/// returns; the `into_model` generator still produces the conversion from this. Lenient on unknown
/// fields (no `deny_unknown_fields`) because we intentionally model a subset.
struct StructOverride {
    name: &'static str,
    doc: &'static str,
    fields: &'static [OverrideField],
}

const STRUCT_OVERRIDES: &[StructOverride] = &[StructOverride {
    name: "GetAddressInfoEmbedded",
    doc: "Information about the address embedded in P2SH or P2WSH, if relevant and known.",
    fields: &[
        OverrideField { rust_name: "address", rename: None, ty: "String", optional: false, doc: "The bitcoin address validated." },
        OverrideField { rust_name: "script_pub_key", rename: Some("scriptPubKey"), ty: "String", optional: false, doc: "The hex-encoded output script generated by the address." },
        OverrideField { rust_name: "solvable", rename: None, ty: "bool", optional: true, doc: "Whether we know how to spend coins sent to this address, ignoring the possible lack of private keys." },
        OverrideField { rust_name: "desc", rename: None, ty: "String", optional: true, doc: "A descriptor for spending coins sent to this address (only when solvable)." },
        OverrideField { rust_name: "parent_desc", rename: None, ty: "String", optional: true, doc: "The descriptor used to derive this address if this is a descriptor wallet." },
        OverrideField { rust_name: "is_script", rename: Some("isscript"), ty: "bool", optional: true, doc: "If the key is a script." },
        OverrideField { rust_name: "is_change", rename: Some("ischange"), ty: "bool", optional: true, doc: "If the address was used for change output." },
        OverrideField { rust_name: "is_witness", rename: Some("iswitness"), ty: "bool", optional: false, doc: "If the address is a witness address." },
        OverrideField { rust_name: "witness_version", rename: None, ty: "i64", optional: true, doc: "The version number of the witness program." },
        OverrideField { rust_name: "witness_program", rename: None, ty: "String", optional: true, doc: "The hex value of the witness program." },
        OverrideField { rust_name: "script", rename: None, ty: "String", optional: true, doc: "The output script type." },
        OverrideField { rust_name: "hex", rename: None, ty: "String", optional: true, doc: "The redeemscript for the p2sh address." },
        OverrideField { rust_name: "pubkeys", rename: None, ty: "Vec<String>", optional: true, doc: "Array of pubkeys associated with the known redeemscript (only if script is multisig)." },
        OverrideField { rust_name: "sigs_required", rename: Some("sigsrequired"), ty: "i64", optional: true, doc: "The number of signatures required to spend multisig output (only if script is multisig)." },
        OverrideField { rust_name: "pubkey", rename: None, ty: "String", optional: true, doc: "The hex value of the raw public key for single-key addresses." },
        OverrideField { rust_name: "is_compressed", rename: Some("iscompressed"), ty: "bool", optional: true, doc: "If the pubkey is compressed." },
        OverrideField { rust_name: "labels", rename: None, ty: "Vec<String>", optional: true, doc: "Array of labels associated with the address." },
    ],
}];

/// Emit a raw struct from a [`StructOverride`] (used when the spec underspecifies the shape).
fn emit_struct_override(ov: &StructOverride) -> GenType {
    let mut field_lines: Vec<String> = Vec::new();
    let mut field_irs: Vec<FieldIr> = Vec::new();
    for f in ov.fields {
        let mut serde_args: Vec<String> = Vec::new();
        if let Some(r) = f.rename {
            serde_args.push(format!("rename = \"{r}\""));
        }
        if f.optional {
            serde_args.push("skip_serializing_if = \"Option::is_none\"".to_owned());
        }
        let attr = if serde_args.is_empty() {
            String::new()
        } else {
            format!("    #[serde({})]\n", serde_args.join(", "))
        };
        let final_ty = if f.optional { format!("Option<{}>", f.ty) } else { f.ty.to_owned() };
        field_lines.push(format!("    /// {}\n{attr}    pub {}: {final_ty},", f.doc, f.rust_name));
        field_irs.push(FieldIr {
            rust_name: f.rust_name.to_owned(),
            rust_type: f.ty.to_owned(),
            optional: f.optional,
        });
    }
    // Lenient: we model a subset of Core's embedded fields, so do not `deny_unknown_fields`.
    let derives = "#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]";
    let body = format!(
        "{}{derives}\npub struct {} {{\n{}\n}}\n",
        fmt_doc(Some(ov.doc)),
        ov.name,
        field_lines.join("\n")
    );
    GenType { name: ov.name.to_owned(), body, nested: Vec::new(), ir: TypeIr::Struct(field_irs) }
}

/// Raw struct fields whose generated Rust name should match the curated type's ergonomic name
/// rather than the wire-derived default, so integration tests that read the field directly stay
/// unchanged when the type migrates to the generated path. `(struct, wire name, rust name)`; a
/// `#[serde(rename = "<wire>")]` is emitted automatically because the rust name then differs.
const RAW_FIELD_RENAME: &[(&str, &str, &str)] =
    &[("GetAddressInfo", "parent_desc", "parent_descriptor")];

/// The rust field-name override for a `(struct, wire field)`, if any.
fn raw_field_rename(struct_name: &str, wire: &str) -> Option<&'static str> {
    RAW_FIELD_RENAME.iter().find(|(s, w, _)| *s == struct_name && *w == wire).map(|(_, _, r)| *r)
}

/// Emit a `pub struct` from an object schema.
fn struct_type(
    name: &str,
    schema: &Schema,
    doc: Option<&str>,
    seen: &mut BTreeSet<String>,
) -> GenType {
    seen.insert(name.to_owned());
    if let Some(ov) = STRUCT_OVERRIDES.iter().find(|o| o.name == name) {
        return emit_struct_override(ov);
    }
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
        let rust_name = match raw_field_rename(name, k) {
            Some(r) => {
                used_fields.insert(r.to_owned());
                r.to_owned()
            }
            None => uniquify(&mut used_fields, &to_rust_field(k)),
        };
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
        field_irs.push(FieldIr { rust_name, rust_type: ty, optional });
    }

    let header = fmt_doc(doc);

    if field_lines.is_empty()
        && commentary_only.iter().any(|s| s.to_lowercase().contains("decoderawtransaction"))
    {
        let body = format!("{header}pub type {name} = DecodeRawTransaction;\n");
        return GenType { name: name.to_owned(), body, nested, ir: TypeIr::Alias };
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
pub(crate) enum NumKind {
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
pub(crate) fn classify_number(parent: &str, field: &str, desc: Option<&str>) -> NumKind {
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
pub(crate) fn verbose_suffix(condition: &str, index: usize) -> String {
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

pub(crate) fn param_type(schema: &Schema, name: Option<&str>) -> String {
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
