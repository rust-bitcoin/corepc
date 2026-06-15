// SPDX-License-Identifier: CC0-1.0

//! Typed representation of the OpenRPC document fields used by codegen.

use serde::Deserialize;
use serde_json::Value;

/// Top-level OpenRPC document.
#[derive(Debug, Deserialize)]
pub struct Spec {
    pub methods: Vec<Method>,
}

/// One JSON-RPC method.
#[derive(Debug, Deserialize)]
pub struct Method {
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub params: Vec<Param>,
    pub result: ResultObj,
    #[serde(rename = "x-bitcoin-category", default = "default_category")]
    pub category: String,
}

fn default_category() -> String { "misc".to_owned() }

/// A method parameter.
#[derive(Debug, Deserialize)]
pub struct Param {
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub required: bool,
    pub schema: Schema,
}

/// A method's result, wrapping its schema.
#[derive(Debug, Deserialize)]
pub struct ResultObj {
    pub schema: Schema,
}

/// A JSON Schema node. Only the fields needed by codegen are named; unknown keys are dropped.
#[derive(Debug, Default, Deserialize)]
pub struct Schema {
    #[serde(rename = "type", default)]
    pub kind: Option<SchemaType>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub default: Option<Value>,
    #[serde(default)]
    pub properties: Option<serde_json::Map<String, Value>>,
    #[serde(default)]
    pub required: Option<Vec<String>>,
    #[serde(default)]
    pub items: Option<Items>,
    #[serde(rename = "prefixItems", default)]
    pub prefix_items: Option<Vec<Schema>>,
    #[serde(rename = "additionalProperties", default)]
    pub additional_properties: Option<AdditionalProperties>,
    #[serde(rename = "oneOf", default)]
    pub one_of: Option<Vec<Schema>>,
    #[serde(rename = "anyOf", default)]
    pub any_of: Option<Vec<Schema>>,
    #[serde(rename = "x-bitcoin-type", default)]
    pub bitcoin_type: Option<String>,
    #[serde(rename = "x-bitcoin-object-dynamic", default)]
    pub dynamic: bool,
    #[serde(rename = "x-bitcoin-optional", default)]
    pub bitcoin_optional: bool,
    #[serde(rename = "x-bitcoin-condition", default)]
    pub condition: Option<String>,
}

/// JSON Schema `type`: either a single string or an array of strings.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum SchemaType {
    One(String),
    Many(Vec<String>),
}

impl SchemaType {
    /// The first declared type.
    pub fn primary(&self) -> &str {
        match self {
            SchemaType::One(s) => s.as_str(),
            SchemaType::Many(v) => v.first().map(String::as_str).unwrap_or(""),
        }
    }
}

/// JSON Schema `additionalProperties`: either a schema object or a bool.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum AdditionalProperties {
    Schema(Box<Schema>),
    Bool(#[allow(dead_code)] bool),
}

/// JSON Schema `items`: a single schema or a tuple array.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Items {
    Single(Box<Schema>),
    Tuple(Vec<Schema>),
}

impl Items {
    /// The canonical item schema; tuple form collapses to the first entry.
    pub fn primary(&self) -> Option<&Schema> {
        match self {
            Items::Single(s) => Some(s),
            Items::Tuple(v) => v.first(),
        }
    }
}

impl Schema {
    /// True for bare scalars (string/bool/number/integer) with no real properties.
    pub fn is_simple(&self) -> bool {
        matches!(
            self.kind.as_ref().map(SchemaType::primary),
            Some("string" | "boolean" | "number" | "integer")
        ) && !self.has_props()
    }

    /// True if the schema's primary type is `null`.
    pub fn returns_null(&self) -> bool {
        matches!(self.kind.as_ref().map(SchemaType::primary), Some("null"))
    }

    /// True if `properties` contains at least one object-valued entry. String-valued entries are
    /// Core's prose commentary and do not count.
    pub fn has_props(&self) -> bool {
        self.properties.as_ref().map(|m| m.values().any(Value::is_object)).unwrap_or(false)
    }

    /// The primary type string, or `None` if untyped.
    pub fn primary_kind(&self) -> Option<&str> { self.kind.as_ref().map(SchemaType::primary) }

    /// The element schema for an array (`items` or `prefixItems`), or `None` if absent.
    pub fn array_items(&self) -> Option<&Schema> {
        self.items
            .as_ref()
            .and_then(Items::primary)
            .or_else(|| self.prefix_items.as_ref().and_then(|v| v.first()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn schema(json: serde_json::Value) -> Schema {
        serde_json::from_value(json).expect("schema should deserialise")
    }

    #[test]
    fn has_props_ignores_string_commentary_in_properties() {
        let only_commentary = schema(serde_json::json!({
            "type": "object",
            "properties": { "note": "this is prose, not a field" }
        }));
        assert!(
            !only_commentary.has_props(),
            "string-valued properties are commentary, not fields"
        );

        let real = schema(serde_json::json!({
            "type": "object",
            "properties": { "height": { "type": "number" } }
        }));
        assert!(real.has_props(), "an object-valued property is a real field");
    }

    #[test]
    fn is_simple_is_false_for_scalar_carrying_real_properties() {
        let s = schema(serde_json::json!({
            "type": "string",
            "properties": { "extra": { "type": "number" } }
        }));
        assert!(!s.is_simple());
    }

    #[test]
    fn is_simple_true_for_bare_scalars_only() {
        for ty in ["string", "boolean", "number", "integer"] {
            assert!(schema(serde_json::json!({ "type": ty })).is_simple(), "{ty} should be simple");
        }
        assert!(!schema(serde_json::json!({ "type": "object" })).is_simple());
        assert!(!schema(serde_json::json!({ "type": "array" })).is_simple());
        let with_commentary = schema(serde_json::json!({
            "type": "number",
            "properties": { "note": "the height" }
        }));
        assert!(with_commentary.is_simple());
    }

    #[test]
    fn schema_type_array_collapses_to_first_entry() {
        let nullable = schema(serde_json::json!({ "type": ["string", "null"] }));
        assert_eq!(nullable.primary_kind(), Some("string"));

        let single = schema(serde_json::json!({ "type": "boolean" }));
        assert_eq!(single.primary_kind(), Some("boolean"));

        let untyped = schema(serde_json::json!({}));
        assert_eq!(untyped.primary_kind(), None);
    }

    #[test]
    fn returns_null_only_for_null_type() {
        assert!(schema(serde_json::json!({ "type": "null" })).returns_null());
        assert!(!schema(serde_json::json!({ "type": "string" })).returns_null());
        assert!(!schema(serde_json::json!({})).returns_null());
    }

    #[test]
    fn additional_properties_accepts_both_bool_and_object() {
        let as_bool = schema(serde_json::json!({
            "type": "object",
            "additionalProperties": false
        }));
        assert!(matches!(as_bool.additional_properties, Some(AdditionalProperties::Bool(false))));

        let as_schema = schema(serde_json::json!({
            "type": "object",
            "additionalProperties": { "type": "string" }
        }));
        match as_schema.additional_properties {
            Some(AdditionalProperties::Schema(inner)) => {
                assert_eq!(inner.primary_kind(), Some("string"));
            }
            other => panic!("expected Schema arm, got {other:?}"),
        }
    }

    #[test]
    fn items_tuple_form_uses_first_schema() {
        let s = schema(serde_json::json!({
            "type": "array",
            "items": [ { "type": "string" }, { "type": "number" } ]
        }));
        let item =
            s.items.as_ref().and_then(Items::primary).expect("tuple items has a first entry");
        assert_eq!(item.primary_kind(), Some("string"));

        let single = schema(serde_json::json!({
            "type": "array",
            "items": { "type": "boolean" }
        }));
        let item = single.items.as_ref().and_then(Items::primary).unwrap();
        assert_eq!(item.primary_kind(), Some("boolean"));
    }

    #[test]
    fn bitcoin_extension_keys_deserialise() {
        let s = schema(serde_json::json!({
            "type": "object",
            "x-bitcoin-type": "amount",
            "x-bitcoin-object-dynamic": true,
            "x-bitcoin-optional": true,
            "x-bitcoin-condition": "verbose=1"
        }));
        assert_eq!(s.bitcoin_type.as_deref(), Some("amount"));
        assert!(s.dynamic);
        assert!(s.bitcoin_optional);
        assert_eq!(s.condition.as_deref(), Some("verbose=1"));
    }

    #[test]
    fn unknown_top_level_keys_are_dropped_not_fatal() {
        let s = schema(serde_json::json!({
            "type": "string",
            "title": "ignored",
            "examples": [1, 2, 3],
            "$comment": "whatever"
        }));
        assert_eq!(s.primary_kind(), Some("string"));
    }
}
