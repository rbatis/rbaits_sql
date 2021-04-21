use std::ops::Div;

use crate::Value;
use crate::ops::AsProxy;

fn div_i64(value: &Value, other: i64) -> i64 {
    if other == 0 {
        return 0;
    }
    (value.as_i64().unwrap_or_default() / other)
}

fn div_u64(value: &Value, other: u64) -> u64 {
    if other == 0 {
        return 0;
    }
    (value.as_u64().unwrap_or_default() / other)
}

fn div_f64(value: &Value, other: f64) -> f64 {
    if other == 0.0 {
        return 0.0;
    }
    value.as_f64().unwrap_or_default() / other
}


fn div_i64_value(value: &Value, other: i64) -> i64 {
    let v = value.as_i64().unwrap_or_default();
    if v == 0 {
        return 0;
    }
    (other / v)
}

fn div_u64_value(value: &Value, other: u64) -> u64 {
    let v = value.as_u64().unwrap_or_default();
    if v == 0 {
        return 0;
    }
    (other / v)
}

fn div_f64_value(value: &Value, other: f64) -> f64 {
    let v = value.as_f64().unwrap_or_default();
    if v == 0.0 {
        return 0.0;
    }
    (other / v)
}

macro_rules! impl_numeric_div {
    ($($div:ident,$div_value:ident [$($ty:ty)*]-> $return_ty:ty)*) => {
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
                    $div_value(&other, self as _)
                }
            }

            impl Div<&Value> for $ty {
                type Output = $return_ty;
                fn div(self, other: &Value) -> Self::Output {
                    $div_value(other, self as _)
                }
            }

            impl Div<&mut Value> for $ty {
                type Output = $return_ty;
                fn div(self, other: &mut Value) -> Self::Output {
                    $div_value(other, self as _)
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
    div_i64,div_i64_value[i8 i16 i32 i64 isize] -> i64
    div_u64,div_u64_value[u8 u16 u32 u64 usize] -> u64
    div_f64,div_f64_value[f32 f64] -> f64
}

//serde json value

impl Div<&serde_json::Value> for Value {
    type Output = Value;
    fn div(self, rhs: &serde_json::Value) -> Self::Output {
        return match self.inner {
            serde_json::Value::Number(s) => {
                if s.is_i64() {
                    let rhs = rhs.as_i64().unwrap_or_default();
                    if rhs == 0 {
                        return serde_json::json!(rhs).into_proxy();
                    }
                    serde_json::json!(s.as_i64().unwrap_or_default() / rhs).into_proxy()
                } else if s.is_f64() {
                    let rhs = rhs.as_f64().unwrap_or_default();
                    if rhs == 0.0 {
                        return serde_json::json!(rhs).into_proxy();
                    }
                    serde_json::json!(s.as_f64().unwrap_or_default() / rhs).into_proxy()
                } else {
                    let rhs = rhs.as_u64().unwrap_or_default();
                    if rhs == 0 {
                        return serde_json::json!(rhs).into_proxy();
                    }
                    serde_json::json!(s.as_u64().unwrap_or_default() / rhs).into_proxy()
                }
            }
            _ => {
                return serde_json::Value::Null.into_proxy();
            }
        };
    }
}

impl Div<serde_json::Value> for Value {
    type Output = Value;
    fn div(self, rhs: serde_json::Value) -> Self::Output {
        return match self.inner {
            serde_json::Value::Number(s) => {
                if s.is_i64() {
                    let rhs = rhs.as_i64().unwrap_or_default();
                    if rhs == 0 {
                        return serde_json::json!(rhs).into_proxy();
                    }
                    serde_json::json!(s.as_i64().unwrap_or_default() / rhs).into_proxy()
                } else if s.is_f64() {
                    let rhs = rhs.as_f64().unwrap_or_default();
                    if rhs == 0.0 {
                        return serde_json::json!(rhs).into_proxy();
                    }
                    serde_json::json!(s.as_f64().unwrap_or_default() / rhs).into_proxy()
                } else {
                    let rhs = rhs.as_u64().unwrap_or_default();
                    if rhs == 0 {
                        return serde_json::json!(rhs).into_proxy();
                    }
                    serde_json::json!(s.as_u64().unwrap_or_default() / rhs).into_proxy()
                }
            }
            _ => {
                return serde_json::Value::Null.into_proxy();
            }
        };
    }
}

impl Div<serde_json::Value> for &Value {
    type Output = Value;
    fn div(self, rhs: serde_json::Value) -> Self::Output {
        return match &self.inner {
            serde_json::Value::Number(s) => {
                if s.is_i64() {
                    let rhs = rhs.as_i64().unwrap_or_default();
                    if rhs == 0 {
                        return serde_json::json!(rhs).into_proxy();
                    }
                    serde_json::json!(s.as_i64().unwrap_or_default() / rhs).into_proxy()
                } else if s.is_f64() {
                    let rhs = rhs.as_f64().unwrap_or_default();
                    if rhs == 0.0 {
                        return serde_json::json!(rhs).into_proxy();
                    }
                    serde_json::json!(s.as_f64().unwrap_or_default() / rhs).into_proxy()
                } else {
                    let rhs = rhs.as_u64().unwrap_or_default();
                    if rhs == 0 {
                        return serde_json::json!(rhs).into_proxy();
                    }
                    serde_json::json!(s.as_u64().unwrap_or_default() / rhs).into_proxy()
                }
            }
            _ => {
                return serde_json::Value::Null.into_proxy();
            }
        };
    }
}

impl Div<&serde_json::Value> for &Value {
    type Output = Value;
    fn div(self, rhs: &serde_json::Value) -> Self::Output {
        return match &self.inner {
            serde_json::Value::Number(s) => {
                if s.is_i64() {
                    let rhs = rhs.as_i64().unwrap_or_default();
                    if rhs == 0 {
                        return serde_json::json!(rhs).into_proxy();
                    }
                    serde_json::json!(s.as_i64().unwrap_or_default() / rhs).into_proxy()
                } else if s.is_f64() {
                    let rhs = rhs.as_f64().unwrap_or_default();
                    if rhs == 0.0 {
                        return serde_json::json!(rhs).into_proxy();
                    }
                    serde_json::json!(s.as_f64().unwrap_or_default() / rhs).into_proxy()
                } else {
                    let rhs = rhs.as_u64().unwrap_or_default();
                    if rhs == 0 {
                        return serde_json::json!(rhs).into_proxy();
                    }
                    serde_json::json!(s.as_u64().unwrap_or_default() / rhs).into_proxy()
                }
            }
            _ => {
                return serde_json::Value::Null.into_proxy();
            }
        };
    }
}


//value

impl Div<&Value> for Value {
    type Output = Value;
    fn div(self, rhs: &Value) -> Self::Output {
        return match self.inner {
            serde_json::Value::Number(s) => {
                if s.is_i64() {
                    let rhs = rhs.as_i64().unwrap_or_default();
                    if rhs == 0 {
                        return serde_json::json!(rhs).into_proxy();
                    }
                    serde_json::json!(s.as_i64().unwrap_or_default() / rhs).into_proxy()
                } else if s.is_f64() {
                    let rhs = rhs.as_f64().unwrap_or_default();
                    if rhs == 0.0 {
                        return serde_json::json!(rhs).into_proxy();
                    }
                    serde_json::json!(s.as_f64().unwrap_or_default() / rhs).into_proxy()
                } else {
                    let rhs = rhs.as_u64().unwrap_or_default();
                    if rhs == 0 {
                        return serde_json::json!(rhs).into_proxy();
                    }
                    serde_json::json!(s.as_u64().unwrap_or_default() / rhs).into_proxy()
                }
            }
            _ => {
                return serde_json::Value::Null.into_proxy();
            }
        };
    }
}

impl Div<Value> for Value {
    type Output = Value;
    fn div(self, rhs: Value) -> Self::Output {
        return match self.inner {
            serde_json::Value::Number(s) => {
                if s.is_i64() {
                    let rhs = rhs.as_i64().unwrap_or_default();
                    if rhs == 0 {
                        return serde_json::json!(rhs).into_proxy();
                    }
                    serde_json::json!(s.as_i64().unwrap_or_default() / rhs).into_proxy()
                } else if s.is_f64() {
                    let rhs = rhs.as_f64().unwrap_or_default();
                    if rhs == 0.0 {
                        return serde_json::json!(rhs).into_proxy();
                    }
                    serde_json::json!(s.as_f64().unwrap_or_default() / rhs).into_proxy()
                } else {
                    let rhs = rhs.as_u64().unwrap_or_default();
                    if rhs == 0 {
                        return serde_json::json!(rhs).into_proxy();
                    }
                    serde_json::json!(s.as_u64().unwrap_or_default() / rhs).into_proxy()
                }
            }
            _ => {
                return serde_json::Value::Null.into_proxy();
            }
        };
    }
}

impl Div<Value> for &Value {
    type Output = Value;
    fn div(self, rhs: Value) -> Self::Output {
        return match &self.inner {
            serde_json::Value::Number(s) => {
                if s.is_i64() {
                    let rhs = rhs.as_i64().unwrap_or_default();
                    if rhs == 0 {
                        return serde_json::json!(rhs).into_proxy();
                    }
                    serde_json::json!(s.as_i64().unwrap_or_default() / rhs).into_proxy()
                } else if s.is_f64() {
                    let rhs = rhs.as_f64().unwrap_or_default();
                    if rhs == 0.0 {
                        return serde_json::json!(rhs).into_proxy();
                    }
                    serde_json::json!(s.as_f64().unwrap_or_default() / rhs).into_proxy()
                } else {
                    let rhs = rhs.as_u64().unwrap_or_default();
                    if rhs == 0 {
                        return serde_json::json!(rhs).into_proxy();
                    }
                    serde_json::json!(s.as_u64().unwrap_or_default() / rhs).into_proxy()
                }
            }
            _ => {
                return serde_json::Value::Null.into_proxy();
            }
        };
    }
}

impl Div<&Value> for &Value {
    type Output = Value;
    fn div(self, rhs: &Value) -> Self::Output {
        return match &self.inner {
            serde_json::Value::Number(s) => {
                if s.is_i64() {
                    let rhs = rhs.as_i64().unwrap_or_default();
                    if rhs == 0 {
                        return serde_json::json!(rhs).into_proxy();
                    }
                    serde_json::json!(s.as_i64().unwrap_or_default() / rhs).into_proxy()
                } else if s.is_f64() {
                    let rhs = rhs.as_f64().unwrap_or_default();
                    if rhs == 0.0 {
                        return serde_json::json!(rhs).into_proxy();
                    }
                    serde_json::json!(s.as_f64().unwrap_or_default() / rhs).into_proxy()
                } else {
                    let rhs = rhs.as_u64().unwrap_or_default();
                    if rhs == 0 {
                        return serde_json::json!(rhs).into_proxy();
                    }
                    serde_json::json!(s.as_u64().unwrap_or_default() / rhs).into_proxy()
                }
            }
            _ => {
                return serde_json::Value::Null.into_proxy();
            }
        };
    }
}