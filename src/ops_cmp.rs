use crate::Value;
use std::cmp::Ordering;

/**
PartialOrd
**/

impl PartialOrd<Value> for Value {
    fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
        self.inner.as_f64().unwrap_or(0.0).partial_cmp(&other.inner.as_f64().unwrap_or(0.0))
    }
}

impl PartialOrd<i32> for &Value {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.inner.as_i64().unwrap_or(0).partial_cmp(&(*other as i64))
    }
}

impl PartialOrd<i64> for &Value {
    fn partial_cmp(&self, other: &i64) -> Option<Ordering> {
        self.inner.as_i64().unwrap_or(0).partial_cmp(&(*other))
    }
}

impl PartialOrd<f32> for &Value {
    fn partial_cmp(&self, other: &f32) -> Option<Ordering> {
        self.inner.as_f64().unwrap_or(0.0).partial_cmp(&(*other as f64))
    }
}

impl PartialOrd<f64> for &Value {
    fn partial_cmp(&self, other: &f64) -> Option<Ordering> {
        self.inner.as_f64().unwrap_or(0.0).partial_cmp(&(*other))
    }
}

impl PartialOrd<u64> for &Value {
    fn partial_cmp(&self, other: &u64) -> Option<Ordering> {
        self.inner.as_u64().unwrap_or(0).partial_cmp(&(*other))
    }
}

//base
impl PartialOrd<Value> for i32 {
    fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
        (*self as i64).partial_cmp(&other.inner.as_i64().unwrap_or(0))
    }
}

impl PartialOrd<Value> for i64 {
    fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
        self.partial_cmp(&other.inner.as_i64().unwrap_or(0))
    }
}

impl PartialOrd<Value> for f32 {
    fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
        (*self as f64).partial_cmp(&other.inner.as_f64().unwrap_or(0.0))
    }
}

impl PartialOrd<Value> for f64 {
    fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
        self.partial_cmp(&other.inner.as_f64().unwrap_or(0.0))
    }
}

impl PartialOrd<Value> for u64 {
    fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
        self.partial_cmp(&other.inner.as_u64().unwrap_or(0))
    }
}