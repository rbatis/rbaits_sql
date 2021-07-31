use std::cmp::{Ordering, PartialOrd as P};
use crate::ops::{Value, AsProxy};
use crate::ops::PartialOrd;

#[inline]
fn cmp_i64(value: i64, other: i64) -> Option<Ordering> {
    Some(value.cmp(&other))
}
#[inline]
fn cmp_u64(value: u64, other: u64) -> Option<Ordering> {
    Some(value.cmp(&other))
}
#[inline]
fn cmp_f64(value: f64, other: f64) -> Option<Ordering> {
    value.partial_cmp(&other)
}
#[inline]
fn cmp_bool(value: bool, other: bool) -> Option<Ordering> {
    Some(value.cmp(&other))
}

/**
PartialOrd
 **/


fn eq_i64(value: &Value, other: i64) -> Option<Ordering> {
    let value = value.i64();
    if value == other {
        Some(Ordering::Equal)
    } else if value > other {
        Some(Ordering::Greater)
    } else {
        Some(Ordering::Less)
    }
}

fn eq_u64(value: &Value, other: u64) -> Option<Ordering> {
    let value = value.u64();
    if value == other {
        Some(Ordering::Equal)
    } else if value > other {
        Some(Ordering::Greater)
    } else {
        Some(Ordering::Less)
    }
}

fn eq_f64(value: &Value, other: f64) -> Option<Ordering> {
    let value = value.f64();
    if value == other {
        Some(Ordering::Equal)
    } else if value > other {
        Some(Ordering::Greater)
    } else {
        Some(Ordering::Less)
    }
}

fn eq_bool(value: &Value, other: bool) -> Option<Ordering> {
    let value = value.bool();
    if value == other {
        Some(Ordering::Equal)
    } else if value == true && other == false {
        Some(Ordering::Greater)
    } else {
        Some(Ordering::Less)
    }
}


impl PartialOrd<Value> for Value {
    fn op_partial_cmp(&self, other: &Value) -> Option<Ordering> {
        match self {
            Value::Null => { Some(Ordering::Equal) }
            Value::Bool(b) => { cmp_bool(*b, other.bool()) }
            Value::Number(n) => { cmp_f64(n.as_f64().unwrap_or_default(), other.f64()) }
            Value::String(s) => { Some(s.cmp(&other.string())) }
            Value::Array(_) => { None }
            Value::Object(_) => { None }
        }
    }
}

impl PartialOrd<Value> for &Value {
    fn op_partial_cmp(&self, other: &Value) -> Option<Ordering> {
        match self {
            Value::Null => { Some(Ordering::Equal) }
            Value::Bool(b) => { cmp_bool(*b, other.bool()) }
            Value::Number(n) => { cmp_f64(n.as_f64().unwrap_or_default(), other.f64()) }
            Value::String(s) => { Some(s.cmp(&other.string())) }
            Value::Array(_) => { None }
            Value::Object(_) => { None }
        }
    }
}

impl PartialOrd<&Value> for Value {
    fn op_partial_cmp(&self, other: &&Value) -> Option<Ordering> {
        match self {
            Value::Null => { Some(Ordering::Equal) }
            Value::Bool(b) => { cmp_bool(*b, other.bool()) }
            Value::Number(n) => { cmp_f64(n.as_f64().unwrap_or_default(), other.f64()) }
            Value::String(s) => { Some(s.cmp(&other.string())) }
            Value::Array(_) => { None }
            Value::Object(_) => { None }
        }
    }
}


macro_rules! impl_numeric_cmp {
    ($($eq:ident [$($ty:ty)*])*) => {
        $($(
            impl PartialOrd<$ty> for Value {
                fn op_partial_cmp(&self, other: &$ty) -> Option<Ordering> {
                    $eq(self, *other as _)
                }
            }

            impl PartialOrd<Value> for $ty {
                fn op_partial_cmp(&self, other: &Value) -> Option<Ordering> {
                    $eq(other, *self as _)
                }
            }

            impl PartialOrd<&Value> for $ty {
                fn op_partial_cmp(&self, other: &&Value)  -> Option<Ordering> {
                    $eq(*other, *self as _)
                }
            }

            impl PartialOrd<&&Value> for $ty {
                fn op_partial_cmp(&self, other: &&&Value)  -> Option<Ordering> {
                    $eq(**other, *self as _)
                }
            }

            impl<'a> PartialOrd<$ty> for &'a Value {
                fn op_partial_cmp(&self, other: &$ty) -> Option<Ordering> {
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


macro_rules! cmp_self {
    ($eq:ident[$($ty:ty)*]) => {
        $(
impl PartialOrd<$ty> for $ty{
      fn op_partial_cmp(&self, rhs: &$ty) ->  Option<Ordering> {
        $eq(*self as _, *rhs as _)
      }
    }
impl PartialOrd<&$ty> for $ty{
      fn op_partial_cmp(&self, rhs: &&$ty) ->  Option<Ordering> {
        $eq(*self as _, **rhs as _)
      }
    }
impl PartialOrd<$ty> for &$ty{
      fn op_partial_cmp(&self, rhs: &$ty) ->  Option<Ordering> {
        $eq(**self as _, *rhs as _)
      }
    }
impl PartialOrd<&$ty> for &$ty{
      fn op_partial_cmp(&self, rhs: &&$ty) ->  Option<Ordering> {
        $eq(**self as _, **rhs as _)
      }
    }
        )*
    };
}

cmp_self!(cmp_i64[i8 i16 i32 i64 isize]);
cmp_self!(cmp_u64[u8 u16 u32 u64 usize]);
cmp_self!(cmp_f64[f32 f64]);