use std::ops::Div;

use crate::Value;

fn div_i64(value: &Value, other: i64) -> i64 {
    if other == 0 {
        return 0;
    }
    value.as_i64().unwrap_or(0) / other
}

fn div_u64(value: &Value, other: u64) -> u64 {
    if other == 0 {
        return 0;
    }
    value.as_u64().unwrap_or(0) / other
}

fn div_f64(value: &Value, other: f64) -> f64 {
    if other == 0.0 {
        return 0.0;
    }
    value.as_f64().unwrap_or(0.0) / other
}

macro_rules! impl_numeric_div {
    ($($div:ident [$($ty:ty)*]-> $return_ty:ty)*) => {
        $($(
            impl Div<$ty> for Value {
                type Output = $return_ty;
                fn div(self, other: $ty) -> Self::Output {
                    $div(&self, other as _)
                }
            }

            impl Div<Value> for $ty {
                type Output = $return_ty;
                fn div(self, other: Value) -> Self::Output {
                    $div(&other, self as _)
                }
            }

            impl<'a> Div<$ty> for &'a Value {
                type Output = $return_ty;
                fn div(self, other: $ty) -> Self::Output {
                    $div(self, other as _)
                }
            }

            impl<'a> Div<$ty> for &'a mut Value {
                type Output = $return_ty;
                fn div(self, other: $ty) -> Self::Output {
                    $div(self, other as _)
                }
            }
        )*)*
    }
}


impl_numeric_div! {
    div_i64[i8 i16 i32 i64 isize] -> i64
    div_u64[u8 u16 u32 u64 usize] -> u64
    div_f64[f32 f64] -> f64
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

impl Div<&Value> for &Value {
    type Output = serde_json::Value;
    fn div(self, rhs: &Value) -> Self::Output {
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