use std::ops::{Add, Sub, Mul};
use serde::{Serializer, Deserializer};
use std::fmt::Debug;


pub trait AsProxy {
    fn as_proxy(self) -> Value;
    fn as_proxy_clone(&self) -> Value;
}


#[derive(Eq, PartialEq)]
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


impl Add<&str> for Value {
    type Output = String;
    fn add(self, rhs: &str) -> Self::Output {
        return match self.inner {
            serde_json::Value::String(s) => {
                s + rhs
            }
            _ => {
                String::new()
            }
        };
    }
}

impl Add<i64> for Value {
    type Output = i64;
    fn add(self, rhs: i64) -> Self::Output {
        return match self.inner {
            serde_json::Value::Number(s) => {
                s.as_i64().unwrap_or(0) + rhs
            }
            _ => {
                0
            }
        };
    }
}

impl Add<i32> for Value {
    type Output = i64;
    fn add(self, rhs: i32) -> Self::Output {
        return match self.inner {
            serde_json::Value::Number(s) => {
                s.as_i64().unwrap_or(0) + rhs as i64
            }
            _ => {
               0
            }
        };
    }
}

impl Add<i32> for &Value {
    type Output = i64;
    fn add(self, rhs: i32) -> Self::Output {
        return match &self.inner {
            serde_json::Value::Number(s) => {
                s.as_i64().unwrap_or(0) + rhs as i64
            }
            _ => {
                0
            }
        };
    }
}

impl Add<f64> for Value {
    type Output = f64;
    fn add(self, rhs: f64) -> Self::Output {
        return match self.inner {
            serde_json::Value::Number(s) => {
                s.as_f64().unwrap_or(0.0) + rhs
            }
            _ => {
               0.0
            }
        };
    }
}

impl Add<u64> for Value {
    type Output = u64;
    fn add(self, rhs: u64) -> Self::Output {
        return match self.inner {
            serde_json::Value::Number(s) => {
                s.as_u64().unwrap_or(0) + rhs
            }
            _ => {
                0
            }
        };
    }
}

impl Add<&serde_json::Value> for Value {
    type Output = serde_json::Value;
    fn add(self, rhs: &serde_json::Value) -> Self::Output {
        return match self.inner {
            serde_json::Value::String(s) => {
                serde_json::Value::String(s + rhs.as_str().unwrap_or(""))
            }
            serde_json::Value::Number(s) => {
                if s.is_i64() {
                    serde_json::json!(s.as_i64().unwrap_or(0) + rhs.as_i64().unwrap_or(0))
                } else if s.is_f64() {
                    serde_json::json!(s.as_f64().unwrap_or(0.0) + rhs.as_f64().unwrap_or(0.0))
                } else {
                    serde_json::json!(s.as_u64().unwrap_or(0) + rhs.as_u64().unwrap_or(0))
                }
            }
            _ => {
                return serde_json::Value::Null;
            }
        };
    }
}

/**
sub
**/


impl Sub<i64> for Value {
    type Output = i64;
    fn sub(self, rhs: i64) -> Self::Output {
        return match self.inner {
            serde_json::Value::Number(s) => {
                s.as_i64().unwrap_or(0) - rhs
            }
            _ => {
                0
            }
        };
    }
}

impl Sub<i32> for Value {
    type Output = i64;
    fn sub(self, rhs: i32) -> Self::Output {
        return match self.inner {
            serde_json::Value::Number(s) => {
                s.as_i64().unwrap_or(0) - rhs as i64
            }
            _ => {
               0
            }
        };
    }
}

impl Sub<i32> for &Value {
    type Output = i64;
    fn sub(self, rhs: i32) -> Self::Output {
        return match &self.inner {
            serde_json::Value::Number(s) => {
                s.as_i64().unwrap_or(0) - rhs as i64
            }
            _ => {
               0
            }
        };
    }
}

impl Sub<f64> for Value {
    type Output = f64;
    fn sub(self, rhs: f64) -> Self::Output {
        return match self.inner {
            serde_json::Value::Number(s) => {
                s.as_f64().unwrap_or(0.0) - rhs
            }
            _ => {
                0.0
            }
        };
    }
}

impl Sub<u64> for Value {
    type Output = u64;
    fn sub(self, rhs: u64) -> Self::Output {
        return match self.inner {
            serde_json::Value::Number(s) => {
                s.as_u64().unwrap_or(0) - rhs
            }
            _ => {
                0
            }
        };
    }
}

impl Sub<&serde_json::Value> for Value {
    type Output = serde_json::Value;
    fn sub(self, rhs: &serde_json::Value) -> Self::Output {
        return match self.inner {
            serde_json::Value::Number(s) => {
                if s.is_i64() {
                    serde_json::json!(s.as_i64().unwrap_or(0) - rhs.as_i64().unwrap_or(0))
                } else if s.is_f64() {
                    serde_json::json!(s.as_f64().unwrap_or(0.0) - rhs.as_f64().unwrap_or(0.0))
                } else {
                    serde_json::json!(s.as_u64().unwrap_or(0) - rhs.as_u64().unwrap_or(0))
                }
            }
            _ => {
                return serde_json::Value::Null;
            }
        };
    }
}



/**
mul
**/


impl Mul<i64> for Value {
    type Output = i64;
    fn mul(self, rhs: i64) -> Self::Output {
        return match self.inner {
            serde_json::Value::Number(s) => {
                s.as_i64().unwrap_or(0) * rhs
            }
            _ => {
                0
            }
        };
    }
}

impl Mul<i32> for Value {
    type Output = i64;
    fn mul(self, rhs: i32) -> Self::Output {
        return match self.inner {
            serde_json::Value::Number(s) => {
                s.as_i64().unwrap_or(0) * rhs as i64
            }
            _ => {
                0
            }
        };
    }
}

impl Mul<i32> for &Value {
    type Output = i64;
    fn mul(self, rhs: i32) -> Self::Output {
        return match &self.inner {
            serde_json::Value::Number(s) => {
                s.as_i64().unwrap_or(0) * rhs as i64
            }
            _ => {
                0
            }
        };
    }
}

impl Mul<f64> for Value {
    type Output = f64;
    fn mul(self, rhs: f64) -> Self::Output {
        return match self.inner {
            serde_json::Value::Number(s) => {
                s.as_f64().unwrap_or(0.0) * rhs
            }
            _ => {
                0.0
            }
        };
    }
}

impl Mul<u64> for Value {
    type Output = u64;
    fn mul(self, rhs: u64) -> Self::Output {
        return match self.inner {
            serde_json::Value::Number(s) => {
                s.as_u64().unwrap_or(0) * rhs
            }
            _ => {
                0
            }
        };
    }
}

impl Mul<&serde_json::Value> for Value {
    type Output = serde_json::Value;
    fn mul(self, rhs: &serde_json::Value) -> Self::Output {
        return match self.inner {
            serde_json::Value::Number(s) => {
                if s.is_i64() {
                    serde_json::json!(s.as_i64().unwrap_or(0) * rhs.as_i64().unwrap_or(0))
                } else if s.is_f64() {
                    serde_json::json!(s.as_f64().unwrap_or(0.0) * rhs.as_f64().unwrap_or(0.0))
                } else {
                    serde_json::json!(s.as_u64().unwrap_or(0) * rhs.as_u64().unwrap_or(0))
                }
            }
            _ => {
                return serde_json::Value::Null;
            }
        };
    }
}

