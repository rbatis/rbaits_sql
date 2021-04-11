use crate::Value;
use std::ops::Rem;

/**
rem
**/
impl Rem<i64> for Value {
    type Output = i64;
    fn rem(self, rhs: i64) -> Self::Output {
        return match self.inner {
            serde_json::Value::Number(s) => {
                s.as_i64().unwrap_or(0) % rhs
            }
            _ => {
                0
            }
        };
    }
}

impl Rem<i32> for Value {
    type Output = i64;
    fn rem(self, rhs: i32) -> Self::Output {
        return match self.inner {
            serde_json::Value::Number(s) => {
                s.as_i64().unwrap_or(0) % rhs as i64
            }
            _ => {
                0
            }
        };
    }
}

impl Rem<f64> for Value {
    type Output = f64;
    fn rem(self, rhs: f64) -> Self::Output {
        return match self.inner {
            serde_json::Value::Number(s) => {
                s.as_f64().unwrap_or(0.0) % rhs
            }
            _ => {
                0.0
            }
        };
    }
}

impl Rem<u64> for Value {
    type Output = u64;
    fn rem(self, rhs: u64) -> Self::Output {
        return match self.inner {
            serde_json::Value::Number(s) => {
                s.as_u64().unwrap_or(0) % rhs
            }
            _ => {
                0
            }
        };
    }
}

impl Rem<&serde_json::Value> for Value {
    type Output = serde_json::Value;
    fn rem(self, rhs: &serde_json::Value) -> Self::Output {
        return match self.inner {
            serde_json::Value::Number(s) => {
                if s.is_i64() {
                    serde_json::json!(s.as_i64().unwrap_or(0) % rhs.as_i64().unwrap_or(0))
                } else if s.is_f64() {
                    serde_json::json!(s.as_f64().unwrap_or(0.0) % rhs.as_f64().unwrap_or(0.0))
                } else {
                    serde_json::json!(s.as_u64().unwrap_or(0) % rhs.as_u64().unwrap_or(0))
                }
            }
            _ => {
                return serde_json::Value::Null;
            }
        };
    }
}


/**
ref
**/

impl Rem<i64> for &Value {
    type Output = i64;
    fn rem(self, rhs: i64) -> Self::Output {
        return match &self.inner {
            serde_json::Value::Number(s) => {
                s.as_i64().unwrap_or(0) % rhs
            }
            _ => {
                0
            }
        };
    }
}

impl Rem<i32> for &Value {
    type Output = i64;
    fn rem(self, rhs: i32) -> Self::Output {
        return match &self.inner {
            serde_json::Value::Number(s) => {
                s.as_i64().unwrap_or(0) % rhs as i64
            }
            _ => {
                0
            }
        };
    }
}

impl Rem<f64> for &Value {
    type Output = f64;
    fn rem(self, rhs: f64) -> Self::Output {
        return match &self.inner {
            serde_json::Value::Number(s) => {
                s.as_f64().unwrap_or(0.0) % rhs
            }
            _ => {
                0.0
            }
        };
    }
}

impl Rem<u64> for &Value {
    type Output = u64;
    fn rem(self, rhs: u64) -> Self::Output {
        return match &self.inner {
            serde_json::Value::Number(s) => {
                s.as_u64().unwrap_or(0) % rhs
            }
            _ => {
                0
            }
        };
    }
}

impl Rem<&serde_json::Value> for &Value {
    type Output = serde_json::Value;
    fn rem(self, rhs: &serde_json::Value) -> Self::Output {
        return match &self.inner {
            serde_json::Value::Number(s) => {
                if s.is_i64() {
                    serde_json::json!(s.as_i64().unwrap_or(0) % rhs.as_i64().unwrap_or(0))
                } else if s.is_f64() {
                    serde_json::json!(s.as_f64().unwrap_or(0.0) % rhs.as_f64().unwrap_or(0.0))
                } else {
                    serde_json::json!(s.as_u64().unwrap_or(0) % rhs.as_u64().unwrap_or(0))
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
impl Rem<Value> for i64 {
    type Output = i64;
    fn rem(self, rhs: Value) -> Self::Output {
        return match rhs.inner {
            serde_json::Value::Number(s) => {
                self as i64 % s.as_i64().unwrap_or(0)
            }
            _ => {
                0
            }
        };
    }
}

impl Rem<Value> for i32 {
    type Output = i64;
    fn rem(self, rhs: Value) -> Self::Output {
        return match rhs.inner {
            serde_json::Value::Number(s) => {
                self as i64 % s.as_i64().unwrap_or(0)
            }
            _ => {
                0
            }
        };
    }
}

impl Rem<Value> for f64 {
    type Output = f64;
    fn rem(self, rhs: Value) -> Self::Output {
        return match rhs.inner {
            serde_json::Value::Number(s) => {
                self as f64 % s.as_f64().unwrap_or(0.0)
            }
            _ => {
                0.0
            }
        };
    }
}

impl Rem<Value> for u64 {
    type Output = u64;
    fn rem(self, rhs: Value) -> Self::Output {
        return match rhs.inner {
            serde_json::Value::Number(s) => {
                self % s.as_u64().unwrap_or(0)
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
impl Rem<&Value> for i64 {
    type Output = i64;
    fn rem(self, rhs: &Value) -> Self::Output {
        return match &rhs.inner {
            serde_json::Value::Number(s) => {
                self as i64 % s.as_i64().unwrap_or(0)
            }
            _ => {
                0
            }
        };
    }
}

impl Rem<&Value> for i32 {
    type Output = i64;
    fn rem(self, rhs: &Value) -> Self::Output {
        return match &rhs.inner {
            serde_json::Value::Number(s) => {
                self as i64 % s.as_i64().unwrap_or(0)
            }
            _ => {
                0
            }
        };
    }
}

impl Rem<&Value> for f64 {
    type Output = f64;
    fn rem(self, rhs: &Value) -> Self::Output {
        return match &rhs.inner {
            serde_json::Value::Number(s) => {
                self as f64 % s.as_f64().unwrap_or(0.0)
            }
            _ => {
                0.0
            }
        };
    }
}

impl Rem<&Value> for u64 {
    type Output = u64;
    fn rem(self, rhs: &Value) -> Self::Output {
        return match &rhs.inner {
            serde_json::Value::Number(s) => {
                self % s.as_u64().unwrap_or(0)
            }
            _ => {
                0
            }
        };
    }
}

/**
value
**/

impl Rem<&Value> for &Value {
    type Output = serde_json::Value;
    fn rem(self, rhs: &Value) -> Self::Output {
        return match &self.inner {
            serde_json::Value::Number(s) => {
                if s.is_i64() {
                    serde_json::json!(s.as_i64().unwrap_or(0) % rhs.as_i64().unwrap_or(0))
                } else if s.is_f64() {
                    serde_json::json!(s.as_f64().unwrap_or(0.0) % rhs.as_f64().unwrap_or(0.0))
                } else {
                    serde_json::json!(s.as_u64().unwrap_or(0) % rhs.as_u64().unwrap_or(0))
                }
            }
            _ => {
                return serde_json::Value::Null;
            }
        };
    }
}
