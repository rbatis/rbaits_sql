use serde::{Serializer, Deserializer};
use std::fmt::{Debug, Formatter};
use std::cmp::Ordering;
use crate::value::JsonValue;
use crate::vec_map::VecMap;
use serde_json::Number;


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



impl JsonValue{
    pub fn as_i64(&self) -> Option<i64> {
        return match &self {
            JsonValue::Number(n) => {
                n.as_i64()
            }
            _ => {
                None
            }
        }
    }
    pub fn as_f64(&self) -> Option<f64> {
        return match &self {
            JsonValue::Number(n) => {
                n.as_f64()
            }
            _ => {
                None
            }
        }
    }
    pub fn as_u64(&self) -> Option<u64> {
        return match &self {
            JsonValue::Number(n) => {
                n.as_u64()
            }
            _ => {
                None
            }
        }
    }
    pub fn as_str(&self) -> Option<&str> {
        return match &self {
            JsonValue::String(n) => {
                Some(n.as_str())
            }
            _ => {
                None
            }
        }
    }
    pub fn as_bool(&self) -> Option<bool> {
        return match &self {
            JsonValue::Bool(n) => {
                Some(*n)
            }
            _ => {
                None
            }
        }
    }
    pub fn as_null(&self) -> Option<()> {
        return match &self {
            JsonValue::Null => {
               return Some(());
            }
            _ => {
                None
            }
        }
    }
    pub fn as_object(&self) -> Option<&VecMap<String, JsonValue>> {
        return match &self {
            JsonValue::Object(n) => {
               Some(n)
            }
            _ => {
                None
            }
        }
    }
    pub fn as_array(&self) -> Option<&Vec<JsonValue>> {
        return match &self {
            JsonValue::Array(n) => {
                Some(n)
            }
            _ => {
                None
            }
        }
    }


    pub fn is_null(&self) -> bool {
        return match &self {
            JsonValue::Null=> {
                true
            }
            _ => {
               false
            }
        }
    }
    pub fn is_string(&self) -> bool {
        return match &self {
            JsonValue::String(_)=> {
                true
            }
            _ => {
                false
            }
        }
    }
    pub fn is_f64(&self) -> bool {
        return match &self {
            JsonValue::Number(n)=> {
                n.is_f64()
            }
            _ => {
                false
            }
        }
    }
    pub fn is_i64(&self) -> bool {
        return match &self {
            JsonValue::Number(n)=> {
                n.is_i64()
            }
            _ => {
                false
            }
        }
    }
    pub fn is_u64(&self) -> bool {
        return match &self {
            JsonValue::Number(n)=> {
                n.is_u64()
            }
            _ => {
                false
            }
        }
    }
    pub fn is_bool(&self) -> bool {
        return match &self {
            JsonValue::Bool(_)=> {
               true
            }
            _ => {
                false
            }
        }
    }
    pub fn is_object(&self) -> bool {
        return match &self {
            JsonValue::Object(_)=> {
                true
            }
            _ => {
                false
            }
        }
    }
    pub fn is_array(&self) -> bool {
        return match &self {
            JsonValue::Array(_)=> {
                true
            }
            _ => {
                false
            }
        }
    }
    pub fn is_empty(&self) -> bool {
        return match &self {
            JsonValue::Null => {
                true
            }
            JsonValue::Bool(_) => {
                false
            }
            JsonValue::Number(_) => {
                false
            }
            JsonValue::String(s) => {
                s.is_empty()
            }
            JsonValue::Array(arr) => {
                arr.is_empty()
            }
            JsonValue::Object(m) => {
                m.is_empty()
            }
        };
    }
}

impl From<serde_json::Number> for JsonValue{
    fn from(arg: serde_json::Number) -> Self {
        JsonValue::Number(arg)
    }
}

impl From<&serde_json::Number> for JsonValue{
    fn from(arg: &serde_json::Number) -> Self {
        JsonValue::Number(arg.clone())
    }
}

impl From<String> for JsonValue{
    fn from(arg: String) -> Self {
        JsonValue::String(arg)
    }
}

impl From<&String> for JsonValue{
    fn from(arg: &String) -> Self {
        JsonValue::String(arg.to_owned())
    }
}

impl From<&str> for JsonValue{
    fn from(arg: &str) -> Self {
        JsonValue::String(arg.to_string())
    }
}
