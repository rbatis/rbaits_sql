use crate::Value;
use std::ops::Sub;
use crate::ops::AsProxy;

impl Sub<&serde_json::Value> for Value {
    type Output = Value;
    fn sub(self, rhs: &serde_json::Value) -> Self::Output {
        return match self.inner {
            serde_json::Value::Number(s) => {
                if s.is_i64() {
                    serde_json::json!(s.as_i64().unwrap_or_default() - rhs.as_i64().unwrap_or_default()).into_proxy()
                } else if s.is_f64() {
                    serde_json::json!(s.as_f64().unwrap_or_default() - rhs.as_f64().unwrap_or_default()).into_proxy()
                } else {
                    serde_json::json!(s.as_u64().unwrap_or_default() - rhs.as_u64().unwrap_or_default()).into_proxy()
                }
            }
            _ => {
                return serde_json::Value::Null.into_proxy();
            }
        };
    }
}

impl Sub<&serde_json::Value> for &Value {
    type Output = Value;
    fn sub(self, rhs: &serde_json::Value) -> Self::Output {
        return match &self.inner {
            serde_json::Value::Number(s) => {
                if s.is_i64() {
                    serde_json::json!(s.as_i64().unwrap_or_default() - rhs.as_i64().unwrap_or_default()).into_proxy()
                } else if s.is_f64() {
                    serde_json::json!(s.as_f64().unwrap_or_default() - rhs.as_f64().unwrap_or_default()).into_proxy()
                } else {
                    serde_json::json!(s.as_u64().unwrap_or_default() - rhs.as_u64().unwrap_or_default()).into_proxy()
                }
            }
            _ => {
                return serde_json::Value::Null.into_proxy();
            }
        };
    }
}

impl Sub<&Value> for &Value {
    type Output = Value;
    fn sub(self, rhs: &Value) -> Self::Output {
        return match &self.inner {
            serde_json::Value::Number(s) => {
                if s.is_i64() {
                    serde_json::json!(s.as_i64().unwrap_or_default() - rhs.as_i64().unwrap_or_default()).into_proxy()
                } else if s.is_f64() {
                    serde_json::json!(s.as_f64().unwrap_or_default() - rhs.as_f64().unwrap_or_default()).into_proxy()
                } else {
                    serde_json::json!(s.as_u64().unwrap_or_default() - rhs.as_u64().unwrap_or_default()).into_proxy()
                }
            }
            _ => {
                return serde_json::Value::Null.into_proxy();
            }
        };
    }
}




fn sub_i64(value: &Value, other: i64) -> i64 {
    value.as_i64().unwrap_or_default() - other
}

fn sub_u64(value: &Value, other: u64) -> u64 {
    value.as_u64().unwrap_or_default() - other
}

fn sub_f64(value: &Value, other: f64) -> f64 {
    value.as_f64().unwrap_or_default() - other
}


fn sub_i64_value(value: &Value, other: i64) -> i64 {
    other - value.as_i64().unwrap_or_default()
}

fn sub_u64_value(value: &Value, other: u64) -> u64 {
    other - value.as_u64().unwrap_or_default()
}

fn sub_f64_value(value: &Value, other: f64) -> f64 {
    other - value.as_f64().unwrap_or_default()
}


macro_rules! impl_numeric_sub {
    ($($sub:ident,$sub_value:ident [$($ty:ty)*]-> $return_ty:ty)*) => {
        $($(
            impl Sub<$ty> for Value {
                type Output = $return_ty;
                fn sub(self, other: $ty) -> Self::Output {
                    $sub(&self, other as _)
                }
            }

            impl Sub<Value> for $ty {
                type Output = $return_ty;
                fn sub(self, other: Value) -> Self::Output {
                    $sub_value(&other, self as _)
                }
            }

            impl Sub<&Value> for $ty {
                type Output = $return_ty;
                fn sub(self, other: &Value) -> Self::Output {
                    $sub_value(other, self as _)
                }
            }

            impl Sub<&mut Value> for $ty {
                type Output = $return_ty;
                fn sub(self, other: &mut Value) -> Self::Output {
                    $sub_value(other, self as _)
                }
            }

            impl<'a> Sub<$ty> for &'a Value {
                type Output = $return_ty;
                fn sub(self, other: $ty) -> Self::Output {
                    $sub(self, other as _)
                }
            }

            impl<'a> Sub<$ty> for &'a mut Value {
                type Output = $return_ty;
                fn sub(self, other: $ty) -> Self::Output {
                    $sub(self, other as _)
                }
            }
        )*)*
    }
}


impl_numeric_sub! {
    sub_i64,sub_i64_value[i8 i16 i32 i64 isize] -> i64
    sub_u64,sub_u64_value[u8 u16 u32 u64 usize] -> u64
    sub_f64,sub_f64_value[f32 f64] -> f64
}



