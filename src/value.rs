use serde_json::{Number, Value};
use crate::vec_map::VecMap;
use serde::{Serialize, Serializer, Deserialize, Deserializer, de};
use std::fmt::Debug;
use serde::de::{Visitor, SeqAccess, MapAccess, DeserializeSeed};
use std::{fmt, io};
use core::ops;
use std::borrow::Borrow;
use std::hash::Hash;
use std::ops::Index;

#[derive(Clone, Eq, PartialEq)]
pub enum JsonValue {
    /// Represents a JSON null value.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!(null);
    /// ```
    Null,

    /// Represents a JSON boolean.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!(true);
    /// ```
    Bool(bool),

    /// Represents a JSON number, whether integer or floating point.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!(12.5);
    /// ```
    Number(Number),

    /// Represents a JSON string.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!("a string");
    /// ```
    String(String),

    /// Represents a JSON array.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!(["an", "array"]);
    /// ```
    Array(Vec<JsonValue>),

    /// Represents a JSON object.
    ///
    /// By default the map is backed by a BTreeMap. Enable the `preserve_order`
    /// feature of serde_json to use IndexMap instead, which preserves
    /// entries in the order they are inserted into the map. In particular, this
    /// allows JSON data to be deserialized into a Value and serialized to a
    /// string while retaining the order of map keys in the input.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!({ "an": "object" });
    /// ```
    Object(VecMap<String, JsonValue>),
}

impl Debug for JsonValue {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            JsonValue::Null => formatter.debug_tuple("Null").finish(),
            JsonValue::Bool(v) => formatter.debug_tuple("Bool").field(&v).finish(),
            JsonValue::Number(ref v) => Debug::fmt(v, formatter),
            JsonValue::String(ref v) => formatter.debug_tuple("String").field(v).finish(),
            JsonValue::Array(ref v) => {
                formatter.write_str("Array(")?;
                Debug::fmt(v, formatter)?;
                formatter.write_str(")")
            }
            JsonValue::Object(ref v) => {
                formatter.write_str("Object(")?;
                v.fmt(formatter)?;
                formatter.write_str(")")
            }
        }
    }
}

impl fmt::Display for JsonValue {
    /// Display a JSON value as a string.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let json = json!({ "city": "London", "street": "10 Downing Street" });
    ///
    /// // Compact format:
    /// //
    /// // {"city":"London","street":"10 Downing Street"}
    /// let compact = format!("{}", json);
    /// assert_eq!(compact,
    ///     "{\"city\":\"London\",\"street\":\"10 Downing Street\"}");
    ///
    /// // Pretty format:
    /// //
    /// // {
    /// //   "city": "London",
    /// //   "street": "10 Downing Street"
    /// // }
    /// let pretty = format!("{:#}", json);
    /// assert_eq!(pretty,
    ///     "{\n  \"city\": \"London\",\n  \"street\": \"10 Downing Street\"\n}");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        struct WriterFormatter<'a, 'b: 'a> {
            inner: &'a mut fmt::Formatter<'b>,
        }

        impl<'a, 'b> io::Write for WriterFormatter<'a, 'b> {
            fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
                // Safety: the serializer below only emits valid utf8 when using
                // the default formatter.
                let s = unsafe { std::str::from_utf8_unchecked(buf) };
                self.inner.write_str(s).map_err(io_error)?;
                Ok(buf.len())
            }

            fn flush(&mut self) -> io::Result<()> {
                Ok(())
            }
        }

        fn io_error(_: fmt::Error) -> io::Error {
            // Error value does not matter because Display impl just maps it
            // back to fmt::Error.
            io::Error::new(io::ErrorKind::Other, "fmt error")
        }

        let alternate = f.alternate();
        let mut wr = WriterFormatter { inner: f };
        if alternate {
            // {:#}
            serde_json::ser::to_writer_pretty(&mut wr, self).map_err(|_| fmt::Error)
        } else {
            // {}
            serde_json::ser::to_writer(&mut wr, self).map_err(|_| fmt::Error)
        }
    }
}

impl From<serde_json::Value> for JsonValue {
    fn from(arg: serde_json::Value) -> Self {
        match arg {
            Value::Null => { Self::Null }
            Value::Bool(b) => { Self::Bool(b) }
            Value::Number(n) => { Self::Number(n) }
            Value::String(s) => { Self::String(s) }
            Value::Array(arr) => {
                let mut array = vec![];
                for x in arr {
                    array.push(JsonValue::from(x));
                }
                JsonValue::Array(array)
            }
            Value::Object(obj) => {
                let mut map = VecMap::new();
                for (k, v) in obj {
                    map.insert(k, JsonValue::from(v));
                }
                JsonValue::Object(map)
            }
        }
    }
}

impl Serialize for JsonValue {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        match *self {
            JsonValue::Null => serializer.serialize_unit(),
            JsonValue::Bool(b) => serializer.serialize_bool(b),
            JsonValue::Number(ref n) => n.serialize(serializer),
            JsonValue::String(ref s) => serializer.serialize_str(s),
            JsonValue::Array(ref v) => v.serialize(serializer),
            JsonValue::Object(ref m) => {
                use serde::ser::SerializeMap;
                let mut map = serializer.serialize_map(Some(m.len()))?;
                for (k, v) in m.iter() {
                    map.serialize_entry(k, v)?;
                }
                map.end()
            }
        }
    }
}


struct KeyClassifier;

enum KeyClass {
    Map(String),
}

impl<'de> DeserializeSeed<'de> for KeyClassifier {
    type Value = KeyClass;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(self)
    }
}

impl<'de> Visitor<'de> for KeyClassifier {
    type Value = KeyClass;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string key")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
    {
        match s {
            _ => Ok(KeyClass::Map(s.to_owned())),
        }
    }

    fn visit_string<E>(self, s: String) -> Result<Self::Value, E>
        where
            E: de::Error,
    {
        match s.as_str() {
            _ => Ok(KeyClass::Map(s)),
        }
    }
}

impl<'de> Deserialize<'de> for JsonValue {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<JsonValue, D::Error>
        where
            D: serde::Deserializer<'de>,
    {
        struct ValueVisitor;

        impl<'de> Visitor<'de> for ValueVisitor {
            type Value = JsonValue;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("any valid JSON JsonValue")
            }

            #[inline]
            fn visit_bool<E>(self, value: bool) -> Result<JsonValue, E> {
                Ok(JsonValue::Bool(value))
            }

            #[inline]
            fn visit_i64<E>(self, value: i64) -> Result<JsonValue, E> {
                Ok(JsonValue::Number(value.into()))
            }

            #[inline]
            fn visit_u64<E>(self, value: u64) -> Result<JsonValue, E> {
                Ok(JsonValue::Number(value.into()))
            }

            #[inline]
            fn visit_f64<E>(self, value: f64) -> Result<JsonValue, E> {
                Ok(Number::from_f64(value).map_or(JsonValue::Null, JsonValue::Number))
            }

            #[inline]
            fn visit_str<E>(self, value: &str) -> Result<JsonValue, E>
                where
                    E: serde::de::Error,
            {
                self.visit_string(String::from(value))
            }

            #[inline]
            fn visit_string<E>(self, value: String) -> Result<JsonValue, E> {
                Ok(JsonValue::String(value))
            }

            #[inline]
            fn visit_none<E>(self) -> Result<JsonValue, E> {
                Ok(JsonValue::Null)
            }

            #[inline]
            fn visit_some<D>(self, deserializer: D) -> Result<JsonValue, D::Error>
                where
                    D: serde::Deserializer<'de>,
            {
                Deserialize::deserialize(deserializer)
            }

            #[inline]
            fn visit_unit<E>(self) -> Result<JsonValue, E> {
                Ok(JsonValue::Null)
            }

            #[inline]
            fn visit_seq<V>(self, mut visitor: V) -> Result<JsonValue, V::Error>
                where
                    V: SeqAccess<'de>,
            {
                let mut vec = Vec::new();

                while let Some(elem) = visitor.next_element()? {
                    vec.push(elem);
                }

                Ok(JsonValue::Array(vec))
            }

            fn visit_map<V>(self, mut visitor: V) -> Result<JsonValue, V::Error>
                where
                    V: MapAccess<'de>,
            {
                match visitor.next_key_seed(KeyClassifier)? {
                    // #[cfg(feature = "arbitrary_precision")]
                    // Some(KeyClass::Number) => {
                    //     let number: NumberFromString = visitor.next_value()?;
                    //     Ok(Value::Number(number.value))
                    // }
                    // #[cfg(feature = "raw_value")]
                    // Some(KeyClass::RawValue) => {
                    //     let value = visitor.next_value_seed(crate::raw::BoxedFromString)?;
                    //     crate::from_str(value.get()).map_err(de::Error::custom)
                    // }
                    Some(KeyClass::Map(first_key)) => {
                        let mut values = VecMap::<String, JsonValue>::new();
                        values.insert(first_key, visitor.next_value()?);
                        while let Some((key, value)) = visitor.next_entry()? {
                            values.insert(key, value);
                        }
                        Ok(JsonValue::Object(values))
                    }
                    None => Ok(JsonValue::Object(VecMap::new())),
                }
            }
        }

        deserializer.deserialize_any(ValueVisitor)
    }
}