use crate::Value;

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
        (*self as f64).eq(&other.as_f64().unwrap_or(0.0))
    }
}

impl PartialEq<f64> for Value {
    fn eq(&self, other: &f64) -> bool {
        (*other as f64).eq(&self.as_f64().unwrap_or(0.0))
    }
}

impl PartialEq<f64> for &Value {
    fn eq(&self, other: &f64) -> bool {
        (*other as f64).eq(&self.as_f64().unwrap_or(0.0))
    }
}


impl PartialEq<Value> for u64 {
    fn eq(&self, other: &Value) -> bool {
        (*self as u64).eq(&other.as_u64().unwrap_or(0))
    }
}

impl PartialEq<u64> for Value {
    fn eq(&self, other: &u64) -> bool {
        (*other as u64).eq(&self.as_u64().unwrap_or(0))
    }
}

impl PartialEq<u64> for &Value {
    fn eq(&self, other: &u64) -> bool {
        (*other as u64).eq(&self.as_u64().unwrap_or(0))
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