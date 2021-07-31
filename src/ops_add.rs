use crate::ops::{Add, AsProxy, Value};

use std::borrow::Cow;


fn op_add_i64(value: &Value, other: i64) -> i64 {
    value.as_i64().unwrap_or_default() + other
}

fn op_add_u64(value: &Value, other: u64) -> u64 {
    value.as_u64().unwrap_or_default() + other
}

fn op_add_f64(value: &Value, other: f64) -> f64 {
    value.as_f64().unwrap_or_default() + other
}

macro_rules! impl_numeric_add {
    ($($eq:ident [$($ty:ty)*]-> $return_ty:ty)*) => {
        $($(
            impl Add<$ty> for Value {
                type Output = $return_ty;
                fn op_add(self, other: $ty) -> Self::Output {
                    $eq(&self, other as _)
                }
            }

            impl Add<Value> for $ty {
                type Output = $return_ty;
                fn op_add(self, other: Value) -> Self::Output {
                    $eq(&other, self as _)
                }
            }

            impl Add<&Value> for $ty {
                type Output = $return_ty;
                fn op_add(self, other: &Value) -> Self::Output {
                    $eq(other, self as _)
                }
            }

            impl Add<&&Value> for $ty {
                type Output = $return_ty;
                fn op_add(self, other: &&Value) -> Self::Output {
                    $eq(*other, self as _)
                }
            }

            impl<'a> Add<$ty> for &'a Value {
                type Output = $return_ty;
                fn op_add(self, other: $ty) -> Self::Output {
                    $eq(self, other as _)
                }
            }

            impl<'a> Add<&$ty> for &'a Value {
                type Output = $return_ty;
                fn op_add(self, other: &$ty) -> Self::Output {
                    $eq(self, *other as _)
                }
            }
        )*)*
    }
}

impl_numeric_add! {
    op_add_i64[i8 i16 i32 i64 isize] -> i64
    op_add_u64[u8 u16 u32 u64 usize] -> u64
    op_add_f64[f32 f64] -> f64
}

//value
impl Add<&Value> for Value {
    type Output = Value;
    fn op_add(self, rhs: &Value) -> Self::Output {
        return match self {
            serde_json::Value::String(s) => {
                serde_json::Value::String(s + rhs.as_str().unwrap_or(""))
            }
            serde_json::Value::Number(s) => {
                if s.is_i64() {
                    serde_json::json!(s.as_i64().unwrap_or_default() + rhs.as_i64().unwrap_or_default())
                } else if s.is_f64() {
                   serde_json::json!(s.as_f64().unwrap_or_default() + rhs.as_f64().unwrap_or_default())
                } else {
                   serde_json::json!(s.as_u64().unwrap_or_default() + rhs.as_u64().unwrap_or_default())
                }
            }
            _ => {
                return serde_json::Value::Null;
            }
        };
    }
}

impl Add<Value> for Value {
    type Output = Value;
    fn op_add(self, rhs: Value) -> Self::Output {
        return match self {
            serde_json::Value::String(s) => {
                serde_json::Value::String(s + rhs.as_str().unwrap_or(""))
            }
            serde_json::Value::Number(s) => {
                if s.is_i64() {
                    serde_json::json!(s.as_i64().unwrap_or_default() + rhs.as_i64().unwrap_or_default())
                } else if s.is_f64() {
                    serde_json::json!(s.as_f64().unwrap_or_default() + rhs.as_f64().unwrap_or_default())
                } else {
                    serde_json::json!(s.as_u64().unwrap_or_default() + rhs.as_u64().unwrap_or_default())
                }
            }
            _ => {
                return serde_json::Value::Null;
            }
        };
    }
}

impl Add<&Value> for &Value {
    type Output = Value;
    fn op_add(self, rhs: &Value) -> Self::Output {
        return match self {
            serde_json::Value::String(s) => {
                serde_json::Value::String(s.to_owned() + rhs.as_str().unwrap_or(""))
            }
            serde_json::Value::Number(s) => {
                if s.is_i64() {
                    serde_json::json!(s.as_i64().unwrap_or_default() + rhs.as_i64().unwrap_or_default())
                } else if s.is_f64() {
                    serde_json::json!(s.as_f64().unwrap_or_default() + rhs.as_f64().unwrap_or_default())
                } else {
                    serde_json::json!(s.as_u64().unwrap_or_default() + rhs.as_u64().unwrap_or_default())
                }
            }
            _ => {
                return serde_json::Value::Null;
            }
        };
    }
}

impl Add<&&Value> for &Value {
    type Output = Value;
    fn op_add(self, rhs: &&Value) -> Self::Output {
        return match self {
            serde_json::Value::String(s) => {
                serde_json::Value::String(s.to_owned() + rhs.as_str().unwrap_or(""))
            }
            serde_json::Value::Number(s) => {
                if s.is_i64() {
                    serde_json::json!(s.as_i64().unwrap_or_default() + rhs.as_i64().unwrap_or_default())
                } else if s.is_f64() {
                    serde_json::json!(s.as_f64().unwrap_or_default() + rhs.as_f64().unwrap_or_default())
                } else {
                    serde_json::json!(s.as_u64().unwrap_or_default() + rhs.as_u64().unwrap_or_default())
                }
            }
            _ => {
                return serde_json::Value::Null;
            }
        };
    }
}

impl Add<Value> for &Value {
    type Output = Value;
    fn op_add(self, rhs: Value) -> Self::Output {
        return match self {
            serde_json::Value::String(s) => {
                serde_json::Value::String(s.to_owned() + rhs.str())
            }
            serde_json::Value::Number(s) => {
                if s.is_i64() {
                    serde_json::json!(s.as_i64().unwrap_or_default() + rhs.as_i64().unwrap_or_default())
                } else if s.is_f64() {
                    serde_json::json!(s.as_f64().unwrap_or_default() + rhs.as_f64().unwrap_or_default())
                } else {
                    serde_json::json!(s.as_u64().unwrap_or_default() + rhs.as_u64().unwrap_or_default())
                }
            }
            _ => {
                return serde_json::Value::Null;
            }
        };
    }
}

//str
impl Add<Value> for &str {
    type Output = String;
    fn op_add(self, rhs: Value) -> Self::Output {
        return match rhs {
            serde_json::Value::String(s) => {
                self.to_string() + s.as_str()
            }
            _ => {
                String::new()
            }
        };
    }
}

impl Add<&Value> for &str {
    type Output = String;
    fn op_add(self, rhs: &Value) -> Self::Output {
        return match rhs {
            serde_json::Value::String(s) => {
                self.to_string() + s.as_str()
            }
            _ => {
                String::new()
            }
        };
    }
}

impl Add<&str> for Value {
    type Output = String;
    fn op_add(self, rhs: &str) -> Self::Output {
        return match self {
            serde_json::Value::String(s) => {
                s + rhs
            }
            _ => {
                String::new()
            }
        };
    }
}

impl Add<&str> for &Value {
    type Output = String;
    fn op_add(self, rhs: &str) -> Self::Output {
        return match self {
            serde_json::Value::String(s) => {
                s.to_string() + rhs
            }
            _ => {
                String::new()
            }
        };
    }
}

impl Add<&&str> for Value {
    type Output = String;
    fn op_add(self, rhs: &&str) -> Self::Output {
        return match self {
            serde_json::Value::String(s) => {
                s + *rhs
            }
            _ => {
                String::new()
            }
        };
    }
}

impl Add<&&str> for &Value {
    type Output = String;
    fn op_add(self, rhs: &&str) -> Self::Output {
        return match self {
            serde_json::Value::String(s) => {
                s.to_string() + *rhs
            }
            _ => {
                String::new()
            }
        };
    }
}

impl Add<String> for Value {
    type Output = String;
    #[must_use]
    fn op_add(self, rhs: String) -> Self::Output {
        return match self {
            serde_json::Value::String(s) => {
                s + rhs.as_str()
            }
            _ => {
                String::new()
            }
        };
    }
}

impl Add<String> for &Value {
    type Output = String;
    fn op_add(self, rhs: String) -> Self::Output {
        return match self {
            serde_json::Value::String(s) => {
                s.to_string() + rhs.as_str()
            }
            _ => {
                String::new()
            }
        };
    }
}

//string ref
impl Add<Value> for &String {
    type Output = String;
    fn op_add(self, rhs: Value) -> Self::Output {
        return match rhs {
            serde_json::Value::String(s) => {
                self.to_string() + s.as_str()
            }
            _ => {
                String::new()
            }
        };
    }
}

impl Add<&Value> for &String {
    type Output = String;
    fn op_add(self, rhs: &Value) -> Self::Output {
        return match rhs {
            serde_json::Value::String(s) => {
                self.to_string() + s.as_str()
            }
            _ => {
                String::new()
            }
        };
    }
}

impl Add<&String> for Value {
    type Output = String;
    fn op_add(self, rhs: &String) -> Self::Output {
        return match self {
            serde_json::Value::String(s) => {
                s + rhs.as_str()
            }
            _ => {
                String::new()
            }
        };
    }
}

impl Add<&String> for &Value {
    type Output = String;
    fn op_add(self, rhs: &String) -> Self::Output {
        return match self {
            serde_json::Value::String(s) => {
                s.to_string() + rhs.as_str()
            }
            _ => {
                String::new()
            }
        };
    }
}

impl Add<Value> for String {
    type Output = String;
    fn op_add(self, rhs: Value) -> Self::Output {
        return match rhs {
            serde_json::Value::String(s) => {
                self + s.as_str()
            }
            _ => {
                String::new()
            }
        };
    }
}

impl Add<&Value> for String {
    type Output = String;
    fn op_add(self, rhs: &Value) -> Self::Output {
        return match rhs {
            serde_json::Value::String(s) => {
                self + s.as_str()
            }
            _ => {
                String::new()
            }
        };
    }
}


macro_rules! add_self {
    ([$($ty:ty)*]) => {
        $(
impl Add<$ty> for $ty{
         type Output = $ty;
      fn op_add(self, rhs: $ty) -> Self::Output {
        self+rhs
      }
    }
impl Add<&$ty> for $ty{
         type Output = $ty;
      fn op_add(self, rhs: &$ty) -> Self::Output {
        self+*rhs
      }
    }
impl Add<$ty> for &$ty{
         type Output = $ty;
      fn op_add(self, rhs: $ty) -> Self::Output {
        *self+rhs
      }
    }
impl Add<&$ty> for &$ty{
         type Output = $ty;
      fn op_add(self, rhs: &$ty) -> Self::Output {
        *self+*rhs
      }
    }
        )*
    };
}

add_self!([i8 i16 i32 i64 isize]);
add_self!([u8 u16 u32 u64 usize]);
add_self!([f32 f64]);

impl Add<String> for String{
    type Output = String;

    fn op_add(self, rhs: String) -> Self::Output {
        self+&rhs
    }
}

impl Add<&str> for String{
    type Output = String;

    fn op_add(self, rhs: &str) -> Self::Output {
        self+rhs
    }
}

impl Add<&&str> for String{
    type Output = String;

    fn op_add(self, rhs: &&str) -> Self::Output {
        self+*rhs
    }
}

impl Add<String> for &str{
    type Output = String;

    fn op_add(self, rhs: String) -> Self::Output {
        self.to_string()+&rhs
    }
}

impl Add<&String> for &str{
    type Output = String;

    fn op_add(self, rhs: &String) -> Self::Output {
        self.to_string() + rhs.as_str()
    }
}

impl Add<&&String> for &str{
    type Output = String;

    fn op_add(self, rhs: &&String) -> Self::Output {
        self.to_string() + rhs.as_str()
    }
}



