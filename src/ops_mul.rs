use crate::Value;
use std::ops::Mul;

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