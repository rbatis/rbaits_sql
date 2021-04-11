use crate::Value;
use std::ops::Add;

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

//ref

impl Add<&str> for &Value {
    type Output = String;
    fn add(self, rhs: &str) -> Self::Output {
        return match &self.inner {
            serde_json::Value::String(s) => {
                s.to_string() + rhs
            }
            _ => {
                String::new()
            }
        };
    }
}

impl Add<i64> for &Value {
    type Output = i64;
    fn add(self, rhs: i64) -> Self::Output {
        return match &self.inner {
            serde_json::Value::Number(s) => {
                s.as_i64().unwrap_or(0) + rhs
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


impl Add<f64> for &Value {
    type Output = f64;
    fn add(self, rhs: f64) -> Self::Output {
        return match &self.inner {
            serde_json::Value::Number(s) => {
                s.as_f64().unwrap_or(0.0) + rhs
            }
            _ => {
                0.0
            }
        };
    }
}

impl Add<u64> for &Value {
    type Output = u64;
    fn add(self, rhs: u64) -> Self::Output {
        return match &self.inner {
            serde_json::Value::Number(s) => {
                s.as_u64().unwrap_or(0) + rhs
            }
            _ => {
                0
            }
        };
    }
}

impl Add<&serde_json::Value> for &Value {
    type Output = serde_json::Value;
    fn add(self, rhs: &serde_json::Value) -> Self::Output {
        return match &self.inner {
            serde_json::Value::String(s) => {
                serde_json::Value::String(s.to_string() + rhs.as_str().unwrap_or(""))
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
base
**/
impl Add<Value> for &str {
    type Output = String;
    fn add(self, rhs: Value) -> Self::Output {
        return match rhs.inner {
            serde_json::Value::String(s) => {
               self.to_string()+&s
            }
            _ => {
                String::new()
            }
        };
    }
}

impl Add<Value> for i64 {
    type Output = i64;
    fn add(self, rhs: Value) -> Self::Output {
        return match rhs.inner {
            serde_json::Value::Number(s) => {
                self +s.as_i64().unwrap_or(0)
            }
            _ => {
                0
            }
        };
    }
}

impl Add<Value> for i32 {
    type Output = i64;
    fn add(self, rhs: Value) -> Self::Output {
        return match rhs.inner {
            serde_json::Value::Number(s) => {
                s.as_i64().unwrap_or(0) + self as i64
            }
            _ => {
                0
            }
        };
    }
}


impl Add<Value> for f64 {
    type Output = f64;
    fn add(self, rhs: Value) -> Self::Output {
        return match rhs.inner {
            serde_json::Value::Number(s) => {
                s.as_f64().unwrap_or(0.0) + self
            }
            _ => {
                0.0
            }
        };
    }
}

impl Add<Value> for u64 {
    type Output = u64;
    fn add(self, rhs: Value) -> Self::Output {
        return match rhs.inner {
            serde_json::Value::Number(s) => {
                s.as_u64().unwrap_or(0) + self
            }
            _ => {
                0
            }
        };
    }
}


/**
base ref
**/
impl Add<&Value> for &str {
    type Output = String;
    fn add(self, rhs: &Value) -> Self::Output {
        return match &rhs.inner {
            serde_json::Value::String(s) => {
                self.to_string()+&s
            }
            _ => {
                String::new()
            }
        };
    }
}

impl Add<&Value> for i64 {
    type Output = i64;
    fn add(self, rhs: &Value) -> Self::Output {
        return match &rhs.inner {
            serde_json::Value::Number(s) => {
                self +s.as_i64().unwrap_or(0)
            }
            _ => {
                0
            }
        };
    }
}

impl Add<&Value> for i32 {
    type Output = i64;
    fn add(self, rhs: &Value) -> Self::Output {
        return match &rhs.inner {
            serde_json::Value::Number(s) => {
                s.as_i64().unwrap_or(0) + self as i64
            }
            _ => {
                0
            }
        };
    }
}


impl Add<&Value> for f64 {
    type Output = f64;
    fn add(self, rhs: &Value) -> Self::Output {
        return match &rhs.inner {
            serde_json::Value::Number(s) => {
                s.as_f64().unwrap_or(0.0) + self
            }
            _ => {
                0.0
            }
        };
    }
}

impl Add<&Value> for u64 {
    type Output = u64;
    fn add(self, rhs: &Value) -> Self::Output {
        return match &rhs.inner {
            serde_json::Value::Number(s) => {
                s.as_u64().unwrap_or(0) + self
            }
            _ => {
                0
            }
        };
    }
}