use crate::ops::Value;
use crate::ops::Mul;
use crate::ops::AsProxy;

impl Mul<&Value> for Value {
    type Output = Value;
    fn op_mul(self, rhs: &Value) -> Self::Output {
        return match self {
            Value::Int32(s) => {
                let rhs = rhs.i32();
                return Value::Int32(s * rhs);
            }
            Value::Int64(s) => {
                let rhs = rhs.i64();
                return Value::Int64(s * rhs);
            }
            Value::Double(s) => {
                let rhs = rhs.as_f64().unwrap_or_default();
                return Value::Double(s * rhs);
            }
            _ => {
                return Value::Null;
            }
        };
    }
}

impl Mul<&&Value> for Value {
    type Output = Value;
    fn op_mul(self, rhs: &&Value) -> Self::Output {
        return match self {
            Value::Int32(s) => {
                let rhs = rhs.i32();
                return Value::Int32(s * rhs);
            }
            Value::Int64(s) => {
                let rhs = rhs.i64();
                return Value::Int64(s * rhs);
            }
            Value::Double(s) => {
                let rhs = rhs.as_f64().unwrap_or_default();
                return Value::Double(s * rhs);
            }
            _ => {
                return Value::Null;
            }
        };
    }
}

impl Mul<Value> for Value {
    type Output = Value;
    fn op_mul(self, rhs: Value) -> Self::Output {
        return match self {
            Value::Int32(s) => {
                let rhs = rhs.i32();
                return Value::Int32(s * rhs);
            }
            Value::Int64(s) => {
                let rhs = rhs.i64();
                return Value::Int64(s * rhs);
            }
            Value::Double(s) => {
                let rhs = rhs.as_f64().unwrap_or_default();
                return Value::Double(s * rhs);
            }
            _ => {
                return Value::Null;
            }
        };
    }
}

impl Mul<&Value> for &Value {
    type Output = Value;
    fn op_mul(self, rhs: &Value) -> Self::Output {
        return match self {
            Value::Int32(s) => {
                let rhs = rhs.i32();
                return Value::Int32(s * rhs);
            }
            Value::Int64(s) => {
                let rhs = rhs.i64();
                return Value::Int64(s * rhs);
            }
            Value::Double(s) => {
                let rhs = rhs.as_f64().unwrap_or_default();
                return Value::Double(s * rhs);
            }
            _ => {
                return Value::Null;
            }
        };
    }
}

impl Mul<&&Value> for &Value {
    type Output = Value;
    fn op_mul(self, rhs: &&Value) -> Self::Output {
        return match self {
            Value::Int32(s) => {
                let rhs = rhs.i32();
                return Value::Int32(s * rhs);
            }
            Value::Int64(s) => {
                let rhs = rhs.i64();
                return Value::Int64(s * rhs);
            }
            Value::Double(s) => {
                let rhs = rhs.as_f64().unwrap_or_default();
                return Value::Double(s * rhs);
            }
            _ => {
                return Value::Null;
            }
        };
    }
}

impl Mul<Value> for &Value {
    type Output = Value;
    fn op_mul(self, rhs: Value) -> Self::Output {
        return match self {
            Value::Int32(s) => {
                let rhs = rhs.i32();
                return Value::Int32(s * rhs);
            }
            Value::Int64(s) => {
                let rhs = rhs.i64();
                return Value::Int64(s * rhs);
            }
            Value::Double(s) => {
                let rhs = rhs.as_f64().unwrap_or_default();
                return Value::Double(s * rhs);
            }
            _ => {
                return Value::Null;
            }
        };
    }
}



fn op_mul_i64(value: &Value, other: i64) -> i64 {
    value.i64() * other
}

fn op_mul_f64(value: &Value, other: f64) -> f64 {
    value.as_f64().unwrap_or_default() * other
}

macro_rules! impl_numeric_mul {
    ($($mul:ident [$($ty:ty)*]-> $return_ty:ty)*) => {
        $($(
            impl Mul<$ty> for Value {
                type Output = $return_ty;
                fn op_mul(self, other: $ty) -> Self::Output {
                    $mul(&self, other as _)
                }
            }

            impl Mul<Value> for $ty {
                type Output = $return_ty;
                fn op_mul(self, other: Value) -> Self::Output {
                    $mul(&other, self as _)
                }
            }

            impl Mul<&Value> for $ty {
                type Output = $return_ty;
                fn op_mul(self, other: &Value) -> Self::Output {
                    $mul(other, self as _)
                }
            }

            impl Mul<&&Value> for $ty {
                type Output = $return_ty;
                fn op_mul(self, other: &&Value) -> Self::Output {
                    $mul(*other, self as _)
                }
            }

            impl<'a> Mul<$ty> for &'a Value {
                type Output = $return_ty;
                fn op_mul(self, other: $ty) -> Self::Output {
                    $mul(self, other as _)
                }
            }

            impl<'a> Mul<&$ty> for &'a Value {
                type Output = $return_ty;
                fn op_mul(self, other: &$ty) -> Self::Output {
                    $mul(self, *other as _)
                }
            }
        )*)*
    }
}


impl_numeric_mul! {
    op_mul_i64[i8 i16 i32 i64 isize] -> i64
    op_mul_f64[f32 f64] -> f64
}




macro_rules! mul_self {
    ([$($ty:ty)*]) => {
        $(
impl Mul<$ty> for $ty{
         type Output = $ty;
      fn op_mul(self, rhs: $ty) -> Self::Output {
        self * rhs
      }
    }
impl Mul<&$ty> for $ty{
         type Output = $ty;
      fn op_mul(self, rhs: &$ty) -> Self::Output {
        self * *rhs
      }
    }
impl Mul<$ty> for &$ty{
         type Output = $ty;
      fn op_mul(self, rhs: $ty) -> Self::Output {
        *self * rhs
      }
    }
impl Mul<&$ty> for &$ty{
         type Output = $ty;
      fn op_mul(self, rhs: &$ty) -> Self::Output {
        *self * *rhs
      }
    }
        )*
    };
}

mul_self!([i8 i16 i32 i64 isize]);
mul_self!([f32 f64]);