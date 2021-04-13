use std::ops::{Add, Sub, Mul, Div, Rem};
use serde::{Serializer, Deserializer};
use std::fmt::{Debug, Formatter};
use std::cmp::Ordering;


/// convert serde_json::Value to Value
pub trait AsProxy {
    fn into_proxy(self) -> Value;
    fn as_proxy(&self) -> Value;
}


/// proxy serde_json::Value struct
/// This structure has a certain amount of computing power
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Value {
    pub inner: serde_json::Value
}

impl Value {
    pub fn as_i64(&self) -> Option<i64> {
        self.inner.as_i64()
    }
    pub fn as_f64(&self) -> Option<f64> {
        self.inner.as_f64()
    }
    pub fn as_u64(&self) -> Option<u64> {
        self.inner.as_u64()
    }
    pub fn as_str(&self) -> Option<&str> {
        self.inner.as_str()
    }
    pub fn as_bool(&self) -> Option<bool> {
        self.inner.as_bool()
    }
    pub fn as_null(&self) -> Option<()> {
        self.inner.as_null()
    }
    pub fn as_object(&self) -> Option<&serde_json::Map<String, serde_json::Value>> {
        self.inner.as_object()
    }
    pub fn as_array(&self) -> Option<&Vec<serde_json::Value>> {
        self.inner.as_array()
    }


    pub fn is_null(&self) -> bool {
        self.inner.is_null()
    }
    pub fn is_string(&self) -> bool {
        self.inner.is_string()
    }
    pub fn is_f64(&self) -> bool {
        self.inner.is_f64()
    }
    pub fn is_i64(&self) -> bool {
        self.inner.is_i64()
    }
    pub fn is_u64(&self) -> bool {
        self.inner.is_u64()
    }
    pub fn is_bool(&self) -> bool {
        self.inner.is_boolean()
    }
    pub fn is_object(&self) -> bool {
        self.inner.is_object()
    }
    pub fn is_array(&self) -> bool {
        self.inner.is_array()
    }
    pub fn is_empty(&self) -> bool {
        return match &self.inner {
            serde_json::Value::Null => {
                true
            }
            serde_json::Value::Bool(_) => {
                false
            }
            serde_json::Value::Number(_) => {
                false
            }
            serde_json::Value::String(s) => {
                s.is_empty()
            }
            serde_json::Value::Array(arr) => {
                arr.is_empty()
            }
            serde_json::Value::Object(m) => {
                m.is_empty()
            }
        };
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.inner, f)
    }
}


impl AsProxy for serde_json::Value {
    fn into_proxy(self) -> Value {
        Value {
            inner: self
        }
    }

    fn as_proxy(&self) -> Value {
        Value {
            inner: self.clone()
        }
    }
}


impl From<serde_json::Value> for Value {
    fn from(arg: serde_json::Value) -> Self {
        Value {
            inner: arg
        }
    }
}

impl From<&serde_json::Value> for Value {
    fn from(arg: &serde_json::Value) -> Self {
        Value {
            inner: arg.clone()
        }
    }
}

impl serde::Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        self.inner.serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error> where
        D: Deserializer<'de> {
        let r = serde_json::Value::deserialize(deserializer);
        match r {
            Ok(o) => {
                return Ok(Value {
                    inner: o
                });
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
}