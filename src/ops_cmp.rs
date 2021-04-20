use std::cmp::Ordering;

use crate::Value;

/**
PartialOrd
**/

fn eq_i64(value: &Value, other: i64) -> Option<Ordering> {
    value.as_i64().unwrap_or_default().partial_cmp(&other)
}

fn eq_u64(value: &Value, other: u64) -> Option<Ordering> {
    value.as_u64().unwrap_or_default().partial_cmp(&other)
}

fn eq_f64(value: &Value, other: f64) -> Option<Ordering> {
    value.as_f64().unwrap_or_default().partial_cmp(&other)
}

fn eq_bool(value: &Value, other: bool) -> Option<Ordering> {
    value.as_bool().unwrap_or(false).partial_cmp(&other)
}

// fn eq_str(value: &Value, other: &str) -> Option<Ordering> {
//     value.as_str().unwrap_or("").partial_cmp(& other)
// }

macro_rules! impl_numeric_cmp {
    ($($eq:ident [$($ty:ty)*])*) => {
        $($(
            impl PartialOrd<$ty> for Value {
                fn partial_cmp(&self, other: &$ty) -> Option<Ordering> {
                    $eq(self, *other as _)
                }
            }

            impl PartialOrd<Value> for $ty {
                fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
                    $eq(other, *self as _)
                }
            }

            impl PartialOrd<&Value> for $ty {
                fn partial_cmp(&self, other: &&Value)  -> Option<Ordering> {
                    $eq(*other, *self as _)
                }
            }

            impl<'a> PartialOrd<$ty> for &'a Value {
                fn partial_cmp(&self, other: &$ty) -> Option<Ordering> {
                    $eq(*self, *other as _)
                }
            }

            impl<'a> PartialOrd<$ty> for &'a mut Value {
                fn partial_cmp(&self, other: &$ty) -> Option<Ordering> {
                    $eq(*self, *other as _)
                }
            }
        )*)*
    }
}

impl_numeric_cmp! {
    eq_i64[i8 i16 i32 i64 isize]
    eq_u64[u8 u16 u32 u64 usize]
    eq_f64[f32 f64]
    eq_bool[bool]
}


impl PartialOrd<Value> for Value {
    fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
        self.inner.as_f64().unwrap_or_default().partial_cmp(&other.inner.as_f64().unwrap_or_default())
    }
}

impl PartialOrd<Value> for &Value {
    fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
        self.inner.as_f64().unwrap_or_default().partial_cmp(&other.inner.as_f64().unwrap_or_default())
    }
}

impl PartialOrd<&Value> for Value{
    fn partial_cmp(&self, other: &&Value) -> Option<Ordering> {
        self.inner.as_f64().unwrap_or_default().partial_cmp(&other.inner.as_f64().unwrap_or_default())
    }
}