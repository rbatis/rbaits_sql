use std::ops::{Add, Sub, Mul, Div, Rem};
use serde::{Serializer, Deserializer};
use std::fmt::Debug;


pub trait AsProxy {
    fn as_proxy(self) -> Value;
    fn as_proxy_clone(&self) -> Value;
}


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
}


impl AsProxy for serde_json::Value {
    fn as_proxy(self) -> Value {
        Value {
            inner: self
        }
    }

    fn as_proxy_clone(&self) -> Value {
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




