use core::fmt;

use serde::ser::{
    Error as SerError, Impossible, SerializeMap, SerializeSeq, SerializeStruct, SerializeTuple,
    Serializer,
};
use serde::Serialize;

/// Error type for URL encoding serialization.
#[derive(Debug)]
pub struct Error(String);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { fmt::Display::fmt(&self.0, f) }
}

impl std::error::Error for Error {}

impl serde::ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self { Error(msg.to_string()) }
}

/// Serialize to a URL query string.
pub fn to_string<T: Serialize>(value: &T) -> Result<String, Error> {
    let mut serializer = UrlSerializer { output: String::new() };
    value.serialize(&mut serializer)?;
    Ok(serializer.output)
}

/// Percent-encode a string for use in URL form data.
fn percent_encode(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => result.push(c),
            ' ' => result.push('+'),
            _ =>
                for byte in c.to_string().as_bytes() {
                    result.push_str(&format!("%{:02X}", byte));
                },
        }
    }
    result
}

struct UrlSerializer {
    output: String,
}

impl UrlSerializer {
    fn push_pair(&mut self, key: &str, value: &str) {
        if !self.output.is_empty() {
            self.output.push('&');
        }
        self.output.push_str(key);
        self.output.push('=');
        self.output.push_str(value);
    }
}

impl<'a> Serializer for &'a mut UrlSerializer {
    type Ok = ();
    type Error = Error;

    type SerializeStruct = Self;
    type SerializeMap = UrlMapSerializer<'a>;
    type SerializeSeq = UrlSeqSerializer<'a>;

    type SerializeTuple = Impossible<(), Self::Error>;
    type SerializeTupleStruct = Impossible<(), Self::Error>;
    type SerializeTupleVariant = Impossible<(), Self::Error>;
    type SerializeStructVariant = Impossible<(), Self::Error>;

    // --- Struct (flat) ---
    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(self)
    }

    // --- Map (top-level HashMap, BTreeMap, etc.) ---
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(UrlMapSerializer { ser: self })
    }

    // --- Seq (top-level Vec<(K,V)>) ---
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(UrlSeqSerializer { ser: self })
    }

    // We intentionally do NOT support arbitrary scalars as top-level.
    fn serialize_str(self, _v: &str) -> Result<Self::Ok, Self::Error> {
        Err(SerError::custom("top-level string not supported; use struct/map/vec of pairs"))
    }
    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
        Err(SerError::custom("top-level scalar not supported; use struct/map/vec of pairs"))
    }
    fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
        Err(SerError::custom("top-level scalar not supported; use struct/map/vec of pairs"))
    }
    fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
        Err(SerError::custom("top-level scalar not supported; use struct/map/vec of pairs"))
    }

    // Everything else: keep it minimal.
    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(SerError::custom("top-level none not supported"))
    }
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(SerError::custom("top-level unit not supported"))
    }

    // The rest can be added if you need them later.
    fn serialize_i128(self, _v: i128) -> Result<Self::Ok, Self::Error> {
        Err(SerError::custom("top-level scalar not supported; use struct/map/vec of pairs"))
    }
    fn serialize_u128(self, _v: u128) -> Result<Self::Ok, Self::Error> {
        Err(SerError::custom("top-level scalar not supported; use struct/map/vec of pairs"))
    }

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> { self.serialize_i32(0) }
    fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> { self.serialize_i32(0) }
    fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> { self.serialize_i32(0) }
    fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> { self.serialize_u32(0) }
    fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> { self.serialize_u32(0) }
    fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> { self.serialize_u32(0) }
    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
        Err(SerError::custom("top-level scalar not supported; use struct/map/vec of pairs"))
    }
    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        Err(SerError::custom("top-level scalar not supported; use struct/map/vec of pairs"))
    }
    fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
        Err(SerError::custom("top-level scalar not supported; use struct/map/vec of pairs"))
    }
    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(SerError::custom("bytes not supported"))
    }
    fn serialize_some<T: ?Sized + Serialize>(self, _value: &T) -> Result<Self::Ok, Self::Error> {
        Err(SerError::custom("top-level some not supported"))
    }
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(SerError::custom("unit struct not supported"))
    }
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(SerError::custom("unit variant not supported"))
    }
    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        Err(SerError::custom("newtype struct not supported"))
    }
    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        Err(SerError::custom("newtype variant not supported"))
    }
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(SerError::custom("top-level tuple not supported"))
    }
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(SerError::custom("tuple struct not supported"))
    }
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(SerError::custom("tuple variant not supported"))
    }
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(SerError::custom("struct variant not supported"))
    }

    fn collect_str<T: ?Sized + fmt::Display>(self, _value: &T) -> Result<Self::Ok, Self::Error> {
        Err(SerError::custom("top-level string not supported; use struct/map/vec of pairs"))
    }
}

// -------------------- struct support --------------------

impl<'a> SerializeStruct for &'a mut UrlSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        let mut vs = ValueSerializer::default();
        value.serialize(&mut vs)?;
        self.push_pair(key, &vs.value);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> { Ok(()) }
}

// -------------------- map support --------------------

struct UrlMapSerializer<'a> {
    ser: &'a mut UrlSerializer,
}

impl<'a> SerializeMap for UrlMapSerializer<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_entry<K: ?Sized + Serialize, V: ?Sized + Serialize>(
        &mut self,
        key: &K,
        value: &V,
    ) -> Result<(), Self::Error> {
        let mut ks = KeySerializer::default();
        key.serialize(&mut ks)?;

        let mut vs = ValueSerializer::default();
        value.serialize(&mut vs)?;

        self.ser.push_pair(&ks.key, &vs.value);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> { Ok(()) }

    fn serialize_key<K: ?Sized + Serialize>(&mut self, _key: &K) -> Result<(), Self::Error> {
        Err(SerError::custom("serialize_key not supported; use serialize_entry"))
    }
    fn serialize_value<V: ?Sized + Serialize>(&mut self, _value: &V) -> Result<(), Self::Error> {
        Err(SerError::custom("serialize_value not supported; use serialize_entry"))
    }
}

// -------------------- seq of pairs support --------------------

struct UrlSeqSerializer<'a> {
    ser: &'a mut UrlSerializer,
}

impl<'a> SerializeSeq for UrlSeqSerializer<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, element: &T) -> Result<(), Self::Error> {
        // Each element must be a (K, V) tuple.
        let mut pair = PairSerializer::default();
        element.serialize(&mut pair)?;
        let (k, v) = pair.finish()?;
        self.ser.push_pair(&k, &v);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> { Ok(()) }
}

/// Collects exactly one (key,value) from a serialized 2-tuple.
#[derive(Default)]
struct PairSerializer {
    key: Option<String>,
    value: Option<String>,
    expecting_tuple_len_2: bool,
}

impl PairSerializer {
    fn finish(self) -> Result<(String, String), Error> {
        if !self.expecting_tuple_len_2 {
            return Err(SerError::custom("expected each element to be a 2-tuple (key, value)"));
        }
        match (self.key, self.value) {
            (Some(k), Some(v)) => Ok((k, v)),
            _ => Err(SerError::custom("missing key or value in (key, value) tuple")),
        }
    }
}

impl<'a> Serializer for &'a mut PairSerializer {
    type Ok = ();
    type Error = Error;

    type SerializeTuple = PairTupleSerializer<'a>;

    type SerializeStruct = Impossible<(), Self::Error>;
    type SerializeSeq = Impossible<(), Self::Error>;
    type SerializeMap = Impossible<(), Self::Error>;
    type SerializeTupleStruct = Impossible<(), Self::Error>;
    type SerializeTupleVariant = Impossible<(), Self::Error>;
    type SerializeStructVariant = Impossible<(), Self::Error>;

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        if len != 2 {
            return Err(SerError::custom("expected tuple length 2 for (key, value)"));
        }
        self.expecting_tuple_len_2 = true;
        Ok(PairTupleSerializer { pair: self, idx: 0 })
    }

    // Anything else is not acceptable for an element.
    fn serialize_str(self, _v: &str) -> Result<Self::Ok, Self::Error> {
        Err(SerError::custom("expected (key, value) tuple, got scalar"))
    }
    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
        Err(SerError::custom("expected (key, value) tuple, got scalar"))
    }
    fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
        Err(SerError::custom("expected (key, value) tuple, got scalar"))
    }
    fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
        Err(SerError::custom("expected (key, value) tuple, got scalar"))
    }

    // Keep minimal:
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(SerError::custom("expected (key, value) tuple, got map"))
    }
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(SerError::custom("expected (key, value) tuple, got seq"))
    }
    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(SerError::custom("expected (key, value) tuple, got struct"))
    }

    // Boilerplate for other methods:
    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(SerError::custom("unsupported"))
    }
    fn serialize_some<T: ?Sized + Serialize>(self, _value: &T) -> Result<Self::Ok, Self::Error> {
        Err(SerError::custom("unsupported"))
    }
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(SerError::custom("unsupported"))
    }
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(SerError::custom("unsupported"))
    }
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(SerError::custom("unsupported"))
    }
    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        Err(SerError::custom("unsupported"))
    }
    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        Err(SerError::custom("unsupported"))
    }
    fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
        Err(SerError::custom("unsupported"))
    }
    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(SerError::custom("unsupported"))
    }
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(SerError::custom("unsupported"))
    }
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(SerError::custom("unsupported"))
    }
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(SerError::custom("unsupported"))
    }

    fn serialize_i128(self, _v: i128) -> Result<Self::Ok, Self::Error> {
        Err(SerError::custom("unsupported"))
    }
    fn serialize_u128(self, _v: u128) -> Result<Self::Ok, Self::Error> {
        Err(SerError::custom("unsupported"))
    }

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
        Err(SerError::custom("unsupported"))
    }
    fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
        Err(SerError::custom("unsupported"))
    }
    fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
        Err(SerError::custom("unsupported"))
    }
    fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
        Err(SerError::custom("unsupported"))
    }
    fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
        Err(SerError::custom("unsupported"))
    }
    fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
        Err(SerError::custom("unsupported"))
    }
    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
        Err(SerError::custom("unsupported"))
    }
    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        Err(SerError::custom("unsupported"))
    }

    fn collect_str<T: ?Sized + fmt::Display>(self, _value: &T) -> Result<Self::Ok, Self::Error> {
        Err(SerError::custom("expected (key, value) tuple, got string"))
    }
}

struct PairTupleSerializer<'a> {
    pair: &'a mut PairSerializer,
    idx: usize,
}

impl<'a> SerializeTuple for PairTupleSerializer<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        match self.idx {
            0 => {
                let mut ks = KeySerializer::default();
                value.serialize(&mut ks)?;
                self.pair.key = Some(ks.key);
            }
            1 => {
                let mut vs = ValueSerializer::default();
                value.serialize(&mut vs)?;
                self.pair.value = Some(vs.value);
            }
            _ => return Err(SerError::custom("too many elements in tuple")),
        }
        self.idx += 1;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> { Ok(()) }
}

// -------------------- key/value serializers --------------------

#[derive(Default)]
struct KeySerializer {
    key: String,
}

impl<'a> Serializer for &'a mut KeySerializer {
    type Ok = ();
    type Error = Error;

    type SerializeStruct = Impossible<(), Self::Error>;
    type SerializeSeq = Impossible<(), Self::Error>;
    type SerializeMap = Impossible<(), Self::Error>;
    type SerializeTuple = Impossible<(), Self::Error>;
    type SerializeTupleStruct = Impossible<(), Self::Error>;
    type SerializeTupleVariant = Impossible<(), Self::Error>;
    type SerializeStructVariant = Impossible<(), Self::Error>;

    fn serialize_str(self, v: &str) -> Result<(), Self::Error> {
        self.key = percent_encode(v);
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<(), Self::Error> {
        self.key = v.to_string();
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<(), Self::Error> {
        self.key = v.to_string();
        Ok(())
    }

    fn serialize_bool(self, v: bool) -> Result<(), Self::Error> {
        self.key = v.to_string();
        Ok(())
    }

    // Everything else: reject to keep keys predictable.
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(SerError::custom("map keys must be scalar (string/number/bool)"))
    }
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(SerError::custom("map keys must be scalar (string/number/bool)"))
    }
    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(SerError::custom("map keys must be scalar (string/number/bool)"))
    }

    // Boilerplate rejections:
    fn serialize_none(self) -> Result<(), Self::Error> {
        Err(SerError::custom("key cannot be none"))
    }
    fn serialize_some<T: ?Sized + Serialize>(self, _value: &T) -> Result<(), Self::Error> {
        Err(SerError::custom("key cannot be option"))
    }
    fn serialize_unit(self) -> Result<(), Self::Error> {
        Err(SerError::custom("key cannot be unit"))
    }
    fn serialize_char(self, _v: char) -> Result<(), Self::Error> {
        Err(SerError::custom("unsupported key type"))
    }
    fn serialize_bytes(self, _v: &[u8]) -> Result<(), Self::Error> {
        Err(SerError::custom("unsupported key type"))
    }
    fn serialize_unit_struct(self, _name: &'static str) -> Result<(), Self::Error> {
        Err(SerError::custom("unsupported key type"))
    }
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<(), Self::Error> {
        Err(SerError::custom("unsupported key type"))
    }
    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<(), Self::Error> {
        Err(SerError::custom("unsupported key type"))
    }
    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<(), Self::Error> {
        Err(SerError::custom("unsupported key type"))
    }
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(SerError::custom("unsupported key type"))
    }
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(SerError::custom("unsupported key type"))
    }
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(SerError::custom("unsupported key type"))
    }
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(SerError::custom("unsupported key type"))
    }

    fn serialize_i128(self, _v: i128) -> Result<(), Self::Error> {
        Err(SerError::custom("unsupported key type"))
    }
    fn serialize_u128(self, _v: u128) -> Result<(), Self::Error> {
        Err(SerError::custom("unsupported key type"))
    }
    fn serialize_i8(self, _v: i8) -> Result<(), Self::Error> {
        Err(SerError::custom("unsupported key type"))
    }
    fn serialize_i16(self, _v: i16) -> Result<(), Self::Error> {
        Err(SerError::custom("unsupported key type"))
    }
    fn serialize_i64(self, _v: i64) -> Result<(), Self::Error> {
        Err(SerError::custom("unsupported key type"))
    }
    fn serialize_u8(self, _v: u8) -> Result<(), Self::Error> {
        Err(SerError::custom("unsupported key type"))
    }
    fn serialize_u16(self, _v: u16) -> Result<(), Self::Error> {
        Err(SerError::custom("unsupported key type"))
    }
    fn serialize_u64(self, _v: u64) -> Result<(), Self::Error> {
        Err(SerError::custom("unsupported key type"))
    }
    fn serialize_f32(self, _v: f32) -> Result<(), Self::Error> {
        Err(SerError::custom("unsupported key type"))
    }
    fn serialize_f64(self, _v: f64) -> Result<(), Self::Error> {
        Err(SerError::custom("unsupported key type"))
    }

    fn collect_str<T: ?Sized + fmt::Display>(self, value: &T) -> Result<Self::Ok, Self::Error> {
        self.key = percent_encode(&value.to_string());
        Ok(())
    }
}

#[derive(Default)]
struct ValueSerializer {
    value: String,
}

impl<'a> Serializer for &'a mut ValueSerializer {
    type Ok = ();
    type Error = Error;

    type SerializeStruct = Impossible<(), Self::Error>;
    type SerializeSeq = Impossible<(), Self::Error>;
    type SerializeMap = Impossible<(), Self::Error>;
    type SerializeTuple = Impossible<(), Self::Error>;
    type SerializeTupleStruct = Impossible<(), Self::Error>;
    type SerializeTupleVariant = Impossible<(), Self::Error>;
    type SerializeStructVariant = Impossible<(), Self::Error>;

    fn serialize_str(self, v: &str) -> Result<(), Self::Error> {
        self.value = percent_encode(v);
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<(), Self::Error> {
        self.value = v.to_string();
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<(), Self::Error> {
        self.value = v.to_string();
        Ok(())
    }

    fn serialize_bool(self, v: bool) -> Result<(), Self::Error> {
        self.value = v.to_string();
        Ok(())
    }

    fn serialize_none(self) -> Result<(), Self::Error> {
        self.value.clear();
        Ok(())
    }

    // Keep minimal:
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(SerError::custom("nested maps not supported"))
    }
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(SerError::custom("nested sequences not supported"))
    }
    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(SerError::custom("nested structs not supported"))
    }

    // Boilerplate rejections:
    fn serialize_some<T: ?Sized + Serialize>(self, _value: &T) -> Result<(), Self::Error> {
        Err(SerError::custom("option values not supported (except None)"))
    }
    fn serialize_unit(self) -> Result<(), Self::Error> { Err(SerError::custom("unsupported")) }
    fn serialize_char(self, _v: char) -> Result<(), Self::Error> {
        Err(SerError::custom("unsupported"))
    }
    fn serialize_bytes(self, _v: &[u8]) -> Result<(), Self::Error> {
        Err(SerError::custom("unsupported"))
    }
    fn serialize_unit_struct(self, _name: &'static str) -> Result<(), Self::Error> {
        Err(SerError::custom("unsupported"))
    }
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<(), Self::Error> {
        Err(SerError::custom("unsupported"))
    }
    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<(), Self::Error> {
        Err(SerError::custom("unsupported"))
    }
    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<(), Self::Error> {
        Err(SerError::custom("unsupported"))
    }
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(SerError::custom("unsupported"))
    }
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(SerError::custom("unsupported"))
    }
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(SerError::custom("unsupported"))
    }
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(SerError::custom("unsupported"))
    }

    fn serialize_i128(self, _v: i128) -> Result<(), Self::Error> {
        Err(SerError::custom("unsupported"))
    }
    fn serialize_u128(self, _v: u128) -> Result<(), Self::Error> {
        Err(SerError::custom("unsupported"))
    }
    fn serialize_i8(self, _v: i8) -> Result<(), Self::Error> {
        Err(SerError::custom("unsupported"))
    }
    fn serialize_i16(self, _v: i16) -> Result<(), Self::Error> {
        Err(SerError::custom("unsupported"))
    }
    fn serialize_i64(self, _v: i64) -> Result<(), Self::Error> {
        Err(SerError::custom("unsupported"))
    }
    fn serialize_u8(self, _v: u8) -> Result<(), Self::Error> {
        Err(SerError::custom("unsupported"))
    }
    fn serialize_u16(self, _v: u16) -> Result<(), Self::Error> {
        Err(SerError::custom("unsupported"))
    }
    fn serialize_u64(self, _v: u64) -> Result<(), Self::Error> {
        Err(SerError::custom("unsupported"))
    }
    fn serialize_f32(self, _v: f32) -> Result<(), Self::Error> {
        Err(SerError::custom("unsupported"))
    }
    fn serialize_f64(self, _v: f64) -> Result<(), Self::Error> {
        Err(SerError::custom("unsupported"))
    }

    fn collect_str<T: ?Sized + fmt::Display>(self, value: &T) -> Result<Self::Ok, Self::Error> {
        self.value = percent_encode(&value.to_string());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;

    #[test]
    fn test_percent_encode_unreserved() {
        assert_eq!(percent_encode("abc"), "abc");
        assert_eq!(percent_encode("ABC"), "ABC");
        assert_eq!(percent_encode("123"), "123");
        assert_eq!(percent_encode("a-b_c.d~e"), "a-b_c.d~e");
    }

    #[test]
    fn test_percent_encode_space() {
        assert_eq!(percent_encode("hello world"), "hello+world");
        assert_eq!(percent_encode("a b c"), "a+b+c");
    }

    #[test]
    fn test_percent_encode_special_chars() {
        assert_eq!(percent_encode("a&b"), "a%26b");
        assert_eq!(percent_encode("a=b"), "a%3Db");
        assert_eq!(percent_encode("a+b"), "a%2Bb");
        assert_eq!(percent_encode("a?b"), "a%3Fb");
        assert_eq!(percent_encode("a/b"), "a%2Fb");
        assert_eq!(percent_encode("a#b"), "a%23b");
    }

    #[test]
    fn test_percent_encode_unicode() {
        assert_eq!(percent_encode("café"), "caf%C3%A9");
        assert_eq!(percent_encode("日本"), "%E6%97%A5%E6%9C%AC");
    }

    #[test]
    fn test_to_string_btreemap() {
        let mut map = BTreeMap::new();
        map.insert("name", "alice");
        map.insert("age", "30");

        let result = to_string(&map).unwrap();
        // BTreeMap is sorted, so order is deterministic
        assert_eq!(result, "age=30&name=alice");
    }

    #[test]
    fn test_to_string_btreemap_with_spaces() {
        let mut map = BTreeMap::new();
        map.insert("greeting", "hello world");

        let result = to_string(&map).unwrap();
        assert_eq!(result, "greeting=hello+world");
    }

    #[test]
    fn test_to_string_btreemap_with_special_chars() {
        let mut map = BTreeMap::new();
        map.insert("query", "a&b=c");

        let result = to_string(&map).unwrap();
        assert_eq!(result, "query=a%26b%3Dc");
    }

    #[test]
    fn test_to_string_vec_of_tuples() {
        let pairs = vec![("foo", "bar"), ("baz", "qux")];

        let result = to_string(&pairs).unwrap();
        assert_eq!(result, "foo=bar&baz=qux");
    }

    #[test]
    fn test_to_string_vec_of_tuples_with_encoding() {
        let pairs = vec![("key", "value with spaces"), ("special", "a&b")];

        let result = to_string(&pairs).unwrap();
        assert_eq!(result, "key=value+with+spaces&special=a%26b");
    }

    #[test]
    fn test_to_string_empty_map() {
        let map: BTreeMap<&str, &str> = BTreeMap::new();

        let result = to_string(&map).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_to_string_empty_vec() {
        let pairs: Vec<(&str, &str)> = vec![];

        let result = to_string(&pairs).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_to_string_numeric_values() {
        let mut map = BTreeMap::new();
        map.insert("count", 42u32);
        map.insert("negative", 0u32); // Can't use negative with u32

        let result = to_string(&map).unwrap();
        assert_eq!(result, "count=42&negative=0");
    }

    #[test]
    fn test_to_string_bool_values() {
        let mut map = BTreeMap::new();
        map.insert("enabled", true);
        map.insert("disabled", false);

        let result = to_string(&map).unwrap();
        assert_eq!(result, "disabled=false&enabled=true");
    }

    #[test]
    fn test_error_top_level_string() {
        let result = to_string(&"just a string");
        assert!(result.is_err());
    }

    #[test]
    fn test_error_top_level_number() {
        let result = to_string(&42i32);
        assert!(result.is_err());
    }
}
