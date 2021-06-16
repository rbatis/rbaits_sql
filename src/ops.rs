use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};

use serde::{Deserializer, Serializer};
use std::borrow::Cow;

/// convert serde_json::Value to Value
pub trait AsProxy {
    fn into_proxy(self) -> Value<'static>;
    fn as_proxy(&self) -> Value<'_>;
}


/// proxy serde_json::Value struct,support Deserializer, Serializer
/// use Cow Optimize unnecessary clones
/// This structure has a certain amount of computing power
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Value<'a> {
    pub inner: Cow<'a, serde_json::Value>,
}

impl Default for Value<'_>{
    fn default() -> Self {
        serde_json::Value::Null.into_proxy()
    }
}

impl<'a> Value<'a> {
    pub fn i64(&self) -> i64 {
        self.inner.as_i64().unwrap_or_default()
    }
    pub fn f64(&self) -> f64 {
        self.inner.as_f64().unwrap_or_default()
    }
    pub fn u64(&self) -> u64 {
        self.inner.as_u64().unwrap_or_default()
    }
    pub fn str(&self) -> &str {
        self.inner.as_str().unwrap_or_default()
    }
    pub fn string(&self) -> String {
        self.inner.as_str().unwrap_or_default().to_string()
    }
    pub fn bool(&self) -> bool {
        self.inner.as_bool().unwrap_or_default()
    }
    pub fn null(&self) -> () {
        self.inner.as_null().unwrap_or_default()
    }


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
        return match self.inner.as_ref() {
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

impl<'a> Value<'a> {
    fn into_proxy(self) -> Value<'a> {
        self
    }
    pub fn as_proxy(self) -> Value<'a> {
        self
    }
}

impl<'a> std::fmt::Display for Value<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.inner, f)
    }
}


impl AsProxy for serde_json::Value {
    fn into_proxy(self) -> Value<'static> {
        Value {
            inner: Cow::Owned(self)
        }
    }

    fn as_proxy(&self) -> Value<'_> {
        Value {
            inner: Cow::Borrowed(self)
        }
    }
}

impl AsProxy for &serde_json::Value {
    fn into_proxy(self) -> Value<'static> {
        Value {
            inner: Cow::Owned(self.clone())
        }
    }

    fn as_proxy(&self) -> Value<'_> {
        Value {
            inner: Cow::Borrowed(self)
        }
    }
}


impl<'a> From<serde_json::Value> for Value<'a> {
    fn from(arg: serde_json::Value) -> Self {
        Value {
            inner: Cow::Owned(arg)
        }
    }
}

impl<'a> From<&'a serde_json::Value> for Value<'a> {
    fn from(arg: &'a serde_json::Value) -> Self {
        Value {
            inner: Cow::Borrowed(arg)
        }
    }
}

impl<'a> serde::Serialize for Value<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        self.inner.serialize(serializer)
    }
}

impl<'a, 'de> serde::Deserialize<'de> for Value<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error> where
        D: Deserializer<'de> {
        let r = serde_json::Value::deserialize(deserializer);
        match r {
            Ok(o) => {
                return Ok(Value {
                    inner: Cow::Owned(o)
                });
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
}


impl AsProxy for &str {
    fn into_proxy(self) -> Value<'static> {
        Value {
            inner: Cow::Owned(serde_json::Value::String(self.to_string()))
        }
    }

    fn as_proxy(&self) -> Value<'static> {
        Value {
            inner: Cow::Owned(serde_json::Value::String(self.to_string()))
        }
    }
}

macro_rules! impl_into_proxy {
    ($($ty:ty)*) => {
        $(
 impl AsProxy for $ty {
    fn into_proxy(self) -> Value<'static> {
        Value {
            inner: Cow::Owned(serde_json::json!(self))
        }
    }
    fn as_proxy(&self) -> Value<'static> {
        Value {
            inner: Cow::Owned(serde_json::json!(self))
        }
    }}
      )*
    };
}

impl_into_proxy!(i8 i16 i32 i64 isize);
impl_into_proxy!(u8 u16 u32 u64 usize);
impl_into_proxy!(f32 f64);
impl_into_proxy!(String);