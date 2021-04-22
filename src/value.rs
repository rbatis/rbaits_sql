use serde_json::{Number, Value};
use crate::vec_map::VecMap;
use serde::{Serialize, Serializer};
use std::fmt::Debug;

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
use serde::ser::Error;

// macro_rules! tri {
//     ($e:expr) => {
//         match $e {
//             crate::error::Result::Ok(val) => val,
//             crate::error::Result::Err(err) => return crate::error::Result::Err(err),
//         }
//     };
//     ($e:expr,) => {
//         tri!($e)
//     };
// }

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
                for (k,v) in m.iter() {
                    map.serialize_entry(k, v)?;
                }
                map.end()
            }
        }
    }
}