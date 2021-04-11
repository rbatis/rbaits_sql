use std::ops::Add;
use serde::{Serializer, Deserializer};
use std::fmt::Debug;

#[derive(Eq, PartialEq)]
pub struct Value {
    pub inner: serde_json::Value
}




impl From<serde_json::Value> for Value{
    fn from(arg: serde_json::Value) -> Self {
        Value{
            inner:arg
        }
    }
}
impl From<&serde_json::Value> for Value{
    fn from(arg: &serde_json::Value) -> Self {
        Value{
            inner:arg.clone()
        }
    }
}

impl serde::Serialize for Value{
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        self.inner.serialize(serializer)
    }
}
impl <'de>serde::Deserialize<'de> for Value{
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error> where
        D: Deserializer<'de> {
        let r=serde_json::Value::deserialize(deserializer);
        match r{
            Ok(o)=>{
                return Ok(Value{
                    inner:o
                });
            }
            Err(e)=>{
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
                rhs.to_string()
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
                rhs
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
                rhs as i64
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
                rhs as i64
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
                rhs
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
                rhs
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