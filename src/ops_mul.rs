use crate::Value;
use std::ops::Mul;

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

impl Mul<&serde_json::Value> for &Value {
    type Output = serde_json::Value;
    fn mul(self, rhs: &serde_json::Value) -> Self::Output {
        return match &self.inner {
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

impl Mul<&Value> for &Value {
    type Output = serde_json::Value;
    fn mul(self, rhs: &Value) -> Self::Output {
        return match &self.inner {
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



fn mul_i64(value: &Value, other: i64) -> i64 {
    value.as_i64().unwrap_or(0) * other
}

fn mul_u64(value: &Value, other: u64) -> u64 {
    value.as_u64().unwrap_or(0) * other
}

fn mul_f64(value: &Value, other: f64) -> f64 {
    value.as_f64().unwrap_or(0.0) * other
}

macro_rules! impl_numeric_mul {
    ($($div:ident [$($ty:ty)*]-> $return_ty:ty)*) => {
        $($(
            impl Mul<$ty> for Value {
                type Output = $return_ty;
                fn mul(self, other: $ty) -> Self::Output {
                    $div(&self, other as _)
                }
            }

            impl Mul<Value> for $ty {
                type Output = $return_ty;
                fn mul(self, other: Value) -> Self::Output {
                    $div(&other, self as _)
                }
            }

            impl<'a> Mul<$ty> for &'a Value {
                type Output = $return_ty;
                fn mul(self, other: $ty) -> Self::Output {
                    $div(self, other as _)
                }
            }

            impl<'a> Mul<$ty> for &'a mut Value {
                type Output = $return_ty;
                fn mul(self, other: $ty) -> Self::Output {
                    $div(self, other as _)
                }
            }
        )*)*
    }
}


impl_numeric_mul! {
    mul_i64[i8 i16 i32 i64 isize] -> i64
    mul_u64[u8 u16 u32 u64 usize] -> u64
    mul_f64[f32 f64] -> f64
}