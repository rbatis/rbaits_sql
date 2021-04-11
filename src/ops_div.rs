use crate::Value;
use std::ops::Div;

/**
div
**/


impl Div<i64> for Value {
    type Output = i64;
    fn div(self, rhs: i64) -> Self::Output {
        return match self.inner {
            serde_json::Value::Number(s) => {
                if rhs == 0 {
                    return 0;
                }
                s.as_i64().unwrap_or(0) / rhs
            }
            _ => {
                0
            }
        };
    }
}

impl Div<i32> for Value {
    type Output = i64;
    fn div(self, rhs: i32) -> Self::Output {
        return match self.inner {
            serde_json::Value::Number(s) => {
                if rhs == 0 {
                    return 0;
                }
                s.as_i64().unwrap_or(0) / rhs as i64
            }
            _ => {
                0
            }
        };
    }
}

impl Div<f64> for Value {
    type Output = f64;
    fn div(self, rhs: f64) -> Self::Output {
        return match self.inner {
            serde_json::Value::Number(s) => {
                if rhs == 0.0 {
                    return 0.0;
                }
                s.as_f64().unwrap_or(0.0) / rhs
            }
            _ => {
                0.0
            }
        };
    }
}

impl Div<u64> for Value {
    type Output = u64;
    fn div(self, rhs: u64) -> Self::Output {
        return match self.inner {
            serde_json::Value::Number(s) => {
                if rhs == 0 {
                    return 0;
                }
                s.as_u64().unwrap_or(0) / rhs
            }
            _ => {
                0
            }
        };
    }
}

impl Div<&serde_json::Value> for Value {
    type Output = serde_json::Value;
    fn div(self, rhs: &serde_json::Value) -> Self::Output {
        return match self.inner {
            serde_json::Value::Number(s) => {
                if s.is_i64() {
                    let rhs = rhs.as_i64().unwrap_or(0);
                    if rhs == 0 {
                        return serde_json::json!(rhs);
                    }
                    serde_json::json!(s.as_i64().unwrap_or(0) / rhs)
                } else if s.is_f64() {
                    let rhs = rhs.as_f64().unwrap_or(0.0);
                    if rhs == 0.0 {
                        return serde_json::json!(rhs);
                    }
                    serde_json::json!(s.as_f64().unwrap_or(0.0) / rhs)
                } else {
                    let rhs = rhs.as_u64().unwrap_or(0);
                    if rhs == 0 {
                        return serde_json::json!(rhs);
                    }
                    serde_json::json!(s.as_u64().unwrap_or(0) / rhs)
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

impl Div<i64> for &Value {
    type Output = i64;
    fn div(self, rhs: i64) -> Self::Output {
        return match &self.inner {
            serde_json::Value::Number(s) => {
                if rhs == 0 {
                    return 0;
                }
                s.as_i64().unwrap_or(0) / rhs
            }
            _ => {
                0
            }
        };
    }
}

impl Div<i32> for &Value {
    type Output = i64;
    fn div(self, rhs: i32) -> Self::Output {
        return match &self.inner {
            serde_json::Value::Number(s) => {
                if rhs == 0 {
                    return 0;
                }
                s.as_i64().unwrap_or(0) / rhs as i64
            }
            _ => {
                0
            }
        };
    }
}

impl Div<f64> for &Value {
    type Output = f64;
    fn div(self, rhs: f64) -> Self::Output {
        return match &self.inner {
            serde_json::Value::Number(s) => {
                if rhs == 0.0 {
                    return 0.0;
                }
                s.as_f64().unwrap_or(0.0) / rhs
            }
            _ => {
                0.0
            }
        };
    }
}

impl Div<u64> for &Value {
    type Output = u64;
    fn div(self, rhs: u64) -> Self::Output {
        return match &self.inner {
            serde_json::Value::Number(s) => {
                if rhs == 0 {
                    return 0;
                }
                s.as_u64().unwrap_or(0) / rhs
            }
            _ => {
                0
            }
        };
    }
}

impl Div<&serde_json::Value> for &Value {
    type Output = serde_json::Value;
    fn div(self, rhs: &serde_json::Value) -> Self::Output {
        return match &self.inner {
            serde_json::Value::Number(s) => {
                if s.is_i64() {
                    let rhs = rhs.as_i64().unwrap_or(0);
                    if rhs == 0 {
                        return serde_json::json!(rhs);
                    }
                    serde_json::json!(s.as_i64().unwrap_or(0) / rhs)
                } else if s.is_f64() {
                    let rhs = rhs.as_f64().unwrap_or(0.0);
                    if rhs == 0.0 {
                        return serde_json::json!(rhs);
                    }
                    serde_json::json!(s.as_f64().unwrap_or(0.0) / rhs)
                } else {
                    let rhs = rhs.as_u64().unwrap_or(0);
                    if rhs == 0 {
                        return serde_json::json!(rhs);
                    }
                    serde_json::json!(s.as_u64().unwrap_or(0) / rhs)
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


impl Div<Value> for i64 {
    type Output = i64;
    fn div(self, rhs: Value) -> Self::Output {
        return match rhs.inner {
            serde_json::Value::Number(s) => {
                if self == 0 {
                    return 0;
                }
                self / s.as_i64().unwrap_or(0)
            }
            _ => {
                0
            }
        };
    }
}

impl Div<Value> for i32 {
    type Output = i64;
    fn div(self, rhs: Value) -> Self::Output {
        return match rhs.inner {
            serde_json::Value::Number(s) => {
                if self == 0 {
                    return 0;
                }
                self as i64 / s.as_i64().unwrap_or(0)
            }
            _ => {
                0
            }
        };
    }
}

impl Div<Value> for f64 {
    type Output = f64;
    fn div(self, rhs: Value) -> Self::Output {
        return match rhs.inner {
            serde_json::Value::Number(s) => {
                if self == 0.0 {
                    return 0.0;
                }
               self / s.as_f64().unwrap_or(0.0)
            }
            _ => {
                0.0
            }
        };
    }
}

impl Div<Value> for u64 {
    type Output = u64;
    fn div(self, rhs: Value) -> Self::Output {
        return match rhs.inner {
            serde_json::Value::Number(s) => {
                if self == 0 {
                    return 0;
                }
                self / s.as_u64().unwrap_or(0)
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


impl Div<&Value> for i64 {
    type Output = i64;
    fn div(self, rhs: &Value) -> Self::Output {
        return match &rhs.inner {
            serde_json::Value::Number(s) => {
                if self == 0 {
                    return 0;
                }
                self / s.as_i64().unwrap_or(0)
            }
            _ => {
                0
            }
        };
    }
}

impl Div<&Value> for i32 {
    type Output = i64;
    fn div(self, rhs: &Value) -> Self::Output {
        return match &rhs.inner {
            serde_json::Value::Number(s) => {
                if self == 0 {
                    return 0;
                }
                self as i64 / s.as_i64().unwrap_or(0)
            }
            _ => {
                0
            }
        };
    }
}

impl Div<&Value> for f64 {
    type Output = f64;
    fn div(self, rhs: &Value) -> Self::Output {
        return match &rhs.inner {
            serde_json::Value::Number(s) => {
                if self == 0.0 {
                    return 0.0;
                }
                self / s.as_f64().unwrap_or(0.0)
            }
            _ => {
                0.0
            }
        };
    }
}

impl Div<&Value> for u64 {
    type Output = u64;
    fn div(self, rhs: &Value) -> Self::Output {
        return match &rhs.inner {
            serde_json::Value::Number(s) => {
                if self == 0 {
                    return 0;
                }
                self / s.as_u64().unwrap_or(0)
            }
            _ => {
                0
            }
        };
    }
}