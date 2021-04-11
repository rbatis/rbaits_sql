use std::ops::{Add, Sub, Mul, Div, Rem};
use serde::{Serializer, Deserializer};
use std::fmt::{Debug, Formatter};


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

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.inner, f)
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


/**
eq base
**/
impl PartialEq<Value> for str {
    fn eq(&self, other: &Value) -> bool {
        other.as_str().unwrap_or("").eq(self)
    }
}

impl PartialEq<str> for Value {
    fn eq(&self, other: &str) -> bool {
        self.as_str().unwrap_or("").eq(other)
    }
}

impl PartialEq<str> for &Value {
    fn eq(&self, other: &str) -> bool {
        self.as_str().unwrap_or("").eq(other)
    }
}

impl PartialEq<Value> for String {
    fn eq(&self, other: &Value) -> bool {
        other.as_str().unwrap_or("").eq(self)
    }
}

impl PartialEq<String> for Value {
    fn eq(&self, other: &String) -> bool {
        other.eq(self.as_str().unwrap_or(""))
    }
}

impl PartialEq<String> for &Value {
    fn eq(&self, other: &String) -> bool {
        other.eq(self.as_str().unwrap_or(""))
    }
}



impl PartialEq<Value> for i32 {
    fn eq(&self, other: &Value) -> bool {
        (*self as i64).eq(&other.as_i64().unwrap_or(0))
    }
}

impl PartialEq<i32> for Value {
    fn eq(&self, other: &i32) -> bool {
        (*other as i64).eq(&self.as_i64().unwrap_or(0))
    }
}

impl PartialEq<i32> for &Value {
    fn eq(&self, other: &i32) -> bool {
        (*other as i64).eq(&self.as_i64().unwrap_or(0))
    }
}

impl PartialEq<Value> for i64 {
    fn eq(&self, other: &Value) -> bool {
        (*self as i64).eq(&other.as_i64().unwrap_or(0))
    }
}

impl PartialEq<i64> for Value {
    fn eq(&self, other: &i64) -> bool {
        (*other as i64).eq(&self.as_i64().unwrap_or(0))
    }
}

impl PartialEq<i64> for &Value {
    fn eq(&self, other: &i64) -> bool {
        (*other as i64).eq(&self.as_i64().unwrap_or(0))
    }
}

impl PartialEq<Value> for f32 {
    fn eq(&self, other: &Value) -> bool {
        (*self as i64).eq(&other.as_i64().unwrap_or(0))
    }
}

impl PartialEq<f32> for Value {
    fn eq(&self, other: &f32) -> bool {
        (*other as i64).eq(&self.as_i64().unwrap_or(0))
    }
}

impl PartialEq<f32> for &Value {
    fn eq(&self, other: &f32) -> bool {
        (*other as i64).eq(&self.as_i64().unwrap_or(0))
    }
}


impl PartialEq<Value> for f64 {
    fn eq(&self, other: &Value) -> bool {
        (*self as i64).eq(&other.as_i64().unwrap_or(0))
    }
}

impl PartialEq<f64> for Value {
    fn eq(&self, other: &f64) -> bool {
        (*other as i64).eq(&self.as_i64().unwrap_or(0))
    }
}

impl PartialEq<f64> for &Value {
    fn eq(&self, other: &f64) -> bool {
        (*other as i64).eq(&self.as_i64().unwrap_or(0))
    }
}


impl PartialEq<Value> for serde_json::Value {
    fn eq(&self, other: &Value) -> bool {
        (*self).eq(&other.inner)
    }
}

impl PartialEq<serde_json::Value> for Value {
    fn eq(&self, other: &serde_json::Value) -> bool {
        (*other).eq(&self.inner)
    }
}

impl PartialEq<serde_json::Value> for &Value {
    fn eq(&self, other: &serde_json::Value) -> bool {
        (*other).eq(&self.inner)
    }
}