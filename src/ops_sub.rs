use crate::ops::Value;
use crate::ops::Sub;
use crate::ops::AsProxy;


//value
impl Sub<&Value> for Value {
    type Output = Value;
    fn op_sub(self, rhs: &Value) -> Self::Output {
        return match self {
            serde_json::Value::Number(s) => {
                if s.is_i64() {
                    serde_json::json!(s.as_i64().unwrap_or_default() - rhs.as_i64().unwrap_or_default())
                } else if s.is_f64() {
                    serde_json::json!(s.as_f64().unwrap_or_default() - rhs.as_f64().unwrap_or_default())
                } else {
                    serde_json::json!(s.as_u64().unwrap_or_default() - rhs.as_u64().unwrap_or_default())
                }
            }
            _ => {
                return serde_json::Value::Null;
            }
        };
    }
}

impl Sub<Value> for Value {
    type Output = Value;
    fn op_sub(self, rhs: Value) -> Self::Output {
        return match self {
            serde_json::Value::Number(s) => {
                if s.is_i64() {
                    serde_json::json!(s.as_i64().unwrap_or_default() - rhs.as_i64().unwrap_or_default())
                } else if s.is_f64() {
                    serde_json::json!(s.as_f64().unwrap_or_default() - rhs.as_f64().unwrap_or_default())
                } else {
                    serde_json::json!(s.as_u64().unwrap_or_default() - rhs.as_u64().unwrap_or_default())
                }
            }
            _ => {
                return serde_json::Value::Null;
            }
        };
    }
}

impl Sub<&Value> for &Value {
    type Output = Value;
    fn op_sub(self, rhs: &Value) -> Self::Output {
        return match self {
            serde_json::Value::Number(s) => {
                if s.is_i64() {
                    serde_json::json!(s.as_i64().unwrap_or_default() - rhs.as_i64().unwrap_or_default())
                } else if s.is_f64() {
                    serde_json::json!(s.as_f64().unwrap_or_default() - rhs.as_f64().unwrap_or_default())
                } else {
                    serde_json::json!(s.as_u64().unwrap_or_default() - rhs.as_u64().unwrap_or_default())
                }
            }
            _ => {
                return serde_json::Value::Null;
            }
        };
    }
}

impl Sub<&&Value> for &Value {
    type Output = Value;
    fn op_sub(self, rhs: &&Value) -> Self::Output {
        return match self {
            serde_json::Value::Number(s) => {
                if s.is_i64() {
                    serde_json::json!(s.as_i64().unwrap_or_default() - rhs.as_i64().unwrap_or_default())
                } else if s.is_f64() {
                    serde_json::json!(s.as_f64().unwrap_or_default() - rhs.as_f64().unwrap_or_default())
                } else {
                    serde_json::json!(s.as_u64().unwrap_or_default() - rhs.as_u64().unwrap_or_default())
                }
            }
            _ => {
                return serde_json::Value::Null;
            }
        };
    }
}

impl Sub<Value> for &Value {
    type Output = Value;
    fn op_sub(self, rhs: Value) -> Self::Output {
        return match self {
            serde_json::Value::Number(s) => {
                if s.is_i64() {
                    serde_json::json!(s.as_i64().unwrap_or_default() - rhs.as_i64().unwrap_or_default())
                } else if s.is_f64() {
                    serde_json::json!(s.as_f64().unwrap_or_default() - rhs.as_f64().unwrap_or_default())
                } else {
                    serde_json::json!(s.as_u64().unwrap_or_default() - rhs.as_u64().unwrap_or_default())
                }
            }
            _ => {
                return serde_json::Value::Null;
            }
        };
    }
}




fn op_sub_i64(value: &Value, other: i64) -> i64 {
    value.as_i64().unwrap_or_default() - other
}

fn op_sub_u64(value: &Value, other: u64) -> u64 {
    value.as_u64().unwrap_or_default() - other
}

fn op_sub_f64(value: &Value, other: f64) -> f64 {
    value.as_f64().unwrap_or_default() - other
}


fn op_sub_i64_value(value: &Value, other: i64) -> i64 {
    other - value.as_i64().unwrap_or_default()
}

fn op_sub_u64_value(value: &Value, other: u64) -> u64 {
    other - value.as_u64().unwrap_or_default()
}

fn op_sub_f64_value(value: &Value, other: f64) -> f64 {
    other - value.as_f64().unwrap_or_default()
}


macro_rules! impl_numeric_sub {
    ($($sub:ident,$sub_value:ident [$($ty:ty)*]-> $return_ty:ty)*) => {
        $($(
            impl Sub<$ty> for Value {
                type Output = $return_ty;
                fn op_sub(self, other: $ty) -> Self::Output {
                    $sub(&self, other as _)
                }
            }

            impl Sub<Value> for $ty {
                type Output = $return_ty;
                fn op_sub(self, other: Value) -> Self::Output {
                    $sub_value(&other, self as _)
                }
            }

            impl Sub<&Value> for $ty {
                type Output = $return_ty;
                fn op_sub(self, other: &Value) -> Self::Output {
                    $sub_value(other, self as _)
                }
            }
            impl Sub<&&Value> for $ty {
                type Output = $return_ty;
                fn op_sub(self, other: &&Value) -> Self::Output {
                    $sub_value(*other, self as _)
                }
            }

            impl<'a> Sub<$ty> for &'a Value {
                type Output = $return_ty;
                fn op_sub(self, other: $ty) -> Self::Output {
                    $sub(self, other as _)
                }
            }
        )*)*
    }
}


impl_numeric_sub! {
    op_sub_i64,op_sub_i64_value[i8 i16 i32 i64 isize] -> i64
    op_sub_u64,op_sub_u64_value[u8 u16 u32 u64 usize] -> u64
    op_sub_f64,op_sub_f64_value[f32 f64] -> f64
}



macro_rules! sub_self {
    ([$($ty:ty)*]) => {
        $(
impl Sub<$ty> for $ty{
         type Output = $ty;
      fn op_sub(self, rhs: $ty) -> Self::Output {
        self-rhs
      }
    }
impl Sub<&$ty> for $ty{
         type Output = $ty;
      fn op_sub(self, rhs: &$ty) -> Self::Output {
        self-*rhs
      }
    }
impl Sub<$ty> for &$ty{
         type Output = $ty;
      fn op_sub(self, rhs: $ty) -> Self::Output {
        *self-rhs
      }
    }
impl Sub<&$ty> for &$ty{
         type Output = $ty;
      fn op_sub(self, rhs: &$ty) -> Self::Output {
        *self-*rhs
      }
    }
        )*
    };
}

sub_self!([i8 i16 i32 i64 isize]);
sub_self!([u8 u16 u32 u64 usize]);
sub_self!([f32 f64]);