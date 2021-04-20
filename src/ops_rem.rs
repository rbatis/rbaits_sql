use crate::Value;
use std::ops::Rem;

impl Rem<&serde_json::Value> for Value {
    type Output = serde_json::Value;
    fn rem(self, rhs: &serde_json::Value) -> Self::Output {
        return match self.inner {
            serde_json::Value::Number(s) => {
                if s.is_i64() {
                    serde_json::json!(s.as_i64().unwrap_or_default() % rhs.as_i64().unwrap_or_default())
                } else if s.is_f64() {
                    serde_json::json!(s.as_f64().unwrap_or_default() % rhs.as_f64().unwrap_or_default())
                } else {
                    serde_json::json!(s.as_u64().unwrap_or_default() % rhs.as_u64().unwrap_or_default())
                }
            }
            _ => {
                return serde_json::Value::Null;
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
                    serde_json::json!(s.as_i64().unwrap_or_default() % rhs.as_i64().unwrap_or_default())
                } else if s.is_f64() {
                    serde_json::json!(s.as_f64().unwrap_or_default() % rhs.as_f64().unwrap_or_default())
                } else {
                    serde_json::json!(s.as_u64().unwrap_or_default() % rhs.as_u64().unwrap_or_default())
                }
            }
            _ => {
                return serde_json::Value::Null;
            }
        };
    }
}

impl Rem<&Value> for &Value {
    type Output = serde_json::Value;
    fn rem(self, rhs: &Value) -> Self::Output {
        return match &self.inner {
            serde_json::Value::Number(s) => {
                if s.is_i64() {
                    serde_json::json!(s.as_i64().unwrap_or_default() % rhs.as_i64().unwrap_or_default())
                } else if s.is_f64() {
                    serde_json::json!(s.as_f64().unwrap_or_default() % rhs.as_f64().unwrap_or_default())
                } else {
                    serde_json::json!(s.as_u64().unwrap_or_default() % rhs.as_u64().unwrap_or_default())
                }
            }
            _ => {
                return serde_json::Value::Null;
            }
        };
    }
}


fn rem_i64(value: &Value, other: i64) -> i64 {
    value.as_i64().unwrap_or_default() * other
}

fn rem_u64(value: &Value, other: u64) -> u64 {
    value.as_u64().unwrap_or_default() * other
}

fn rem_f64(value: &Value, other: f64) -> f64 {
    value.as_f64().unwrap_or_default() * other
}

macro_rules! impl_numeric_rem {
    ($($rem:ident [$($ty:ty)*]-> $return_ty:ty)*) => {
        $($(
            impl Rem<$ty> for Value {
                type Output = $return_ty;
                fn rem(self, other: $ty) -> Self::Output {
                    $rem(&self, other as _)
                }
            }

            impl Rem<Value> for $ty {
                type Output = $return_ty;
                fn rem(self, other: Value) -> Self::Output {
                    $rem(&other, self as _)
                }
            }

            impl<'a> Rem<$ty> for &'a Value {
                type Output = $return_ty;
                fn rem(self, other: $ty) -> Self::Output {
                    $rem(self, other as _)
                }
            }

            impl<'a> Rem<$ty> for &'a mut Value {
                type Output = $return_ty;
                fn rem(self, other: $ty) -> Self::Output {
                    $rem(self, other as _)
                }
            }
        )*)*
    }
}


impl_numeric_rem! {
    rem_i64[i8 i16 i32 i64 isize] -> i64
    rem_u64[u8 u16 u32 u64 usize] -> u64
    rem_f64[f32 f64] -> f64
}



