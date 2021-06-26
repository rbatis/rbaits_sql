use std::ops::Add;

use crate::Value;
use std::borrow::Cow;

fn add_i64(value: &Value, other: i64) -> i64 {
    value.as_i64().unwrap_or_default() + other
}

fn add_u64(value: &Value, other: u64) -> u64 {
    value.as_u64().unwrap_or_default() + other
}

fn add_f64(value: &Value, other: f64) -> f64 {
    value.as_f64().unwrap_or_default() + other
}

macro_rules! impl_numeric_add {
    ($($eq:ident [$($ty:ty)*]-> $return_ty:ty)*) => {
        $($(
            impl Add<$ty> for Value<'_> {
                type Output = $return_ty;
                fn add(self, other: $ty) -> Self::Output {
                    $eq(&self, other as _)
                }
            }

            impl Add<Value<'_>> for $ty {
                type Output = $return_ty;
                fn add(self, other: Value) -> Self::Output {
                    $eq(&other, self as _)
                }
            }

            impl Add<&Value<'_>> for $ty {
                type Output = $return_ty;
                fn add(self, other: &Value) -> Self::Output {
                    $eq(other, self as _)
                }
            }

            impl Add<&mut Value<'_>> for $ty {
                type Output = $return_ty;
                fn add(self, other: &mut Value) -> Self::Output {
                    $eq(other, self as _)
                }
            }

            impl<'a> Add<$ty> for &'a Value<'_> {
                type Output = $return_ty;
                fn add(self, other: $ty) -> Self::Output {
                    $eq(self, other as _)
                }
            }

            impl<'a> Add<$ty> for &'a mut Value<'_> {
                type Output = $return_ty;
                fn add(self, other: $ty) -> Self::Output {
                    $eq(self, other as _)
                }
            }
        )*)*
    }
}

impl_numeric_add! {
    add_i64[i8 i16 i32 i64 isize] -> i64
    add_u64[u8 u16 u32 u64 usize] -> u64
    add_f64[f32 f64] -> f64
}

//value
impl Add<&Value<'_>> for Value<'_> {
    type Output = Value<'static>;
    fn add(self, rhs: &Value) -> Self::Output {
        let inner = match self.inner {
            Cow::Borrowed(b) => {
                b.to_owned()
            }
            Cow::Owned(o) => {
                o.to_owned()
            }
        };
        return match inner {
            serde_json::Value::String(s) => {
                Value {
                    inner: Cow::Owned(serde_json::Value::String(s + rhs.as_str().unwrap_or("")))
                }
            }
            serde_json::Value::Number(s) => {
                if s.is_i64() {
                    Value { inner: Cow::Owned(serde_json::json!(s.as_i64().unwrap_or_default() + rhs.as_i64().unwrap_or_default())) }
                } else if s.is_f64() {
                    Value { inner: Cow::Owned(serde_json::json!(s.as_f64().unwrap_or_default() + rhs.as_f64().unwrap_or_default())) }
                } else {
                    Value { inner: Cow::Owned(serde_json::json!(s.as_u64().unwrap_or_default() + rhs.as_u64().unwrap_or_default())) }
                }
            }
            _ => {
                return Value { inner: Cow::Owned(serde_json::Value::Null) };
            }
        };
    }
}

impl Add<Value<'_>> for Value<'_> {
    type Output = Value<'static>;
    fn add(self, rhs: Value) -> Self::Output {
        let inner = match self.inner {
            Cow::Borrowed(b) => {
                b.to_owned()
            }
            Cow::Owned(o) => {
                o.to_owned()
            }
        };
        return match inner {
            serde_json::Value::String(s) => {
                Value { inner: Cow::Owned(serde_json::Value::String(s + rhs.as_str().unwrap_or(""))) }
            }
            serde_json::Value::Number(s) => {
                if s.is_i64() {
                    Value { inner: Cow::Owned(serde_json::json!(s.as_i64().unwrap_or_default() + rhs.as_i64().unwrap_or_default())) }
                } else if s.is_f64() {
                    Value { inner: Cow::Owned(serde_json::json!(s.as_f64().unwrap_or_default() + rhs.as_f64().unwrap_or_default())) }
                } else {
                    Value { inner: Cow::Owned(serde_json::json!(s.as_u64().unwrap_or_default() + rhs.as_u64().unwrap_or_default())) }
                }
            }
            _ => {
                return Value { inner: Cow::Owned(serde_json::Value::Null) };
            }
        };
    }
}

impl Add<&Value<'_>> for &Value<'_> {
    type Output = Value<'static>;
    fn add(self, rhs: &Value) -> Self::Output {
        return match self.inner.as_ref() {
            serde_json::Value::String(s) => {
                Value { inner: Cow::Owned(serde_json::Value::String(s.to_string() + rhs.as_str().unwrap_or(""))) }
            }
            serde_json::Value::Number(s) => {
                if s.is_i64() {
                    Value { inner: Cow::Owned(serde_json::json!(s.as_i64().unwrap_or_default() + rhs.as_i64().unwrap_or_default())) }
                } else if s.is_f64() {
                    Value { inner: Cow::Owned(serde_json::json!(s.as_f64().unwrap_or_default() + rhs.as_f64().unwrap_or_default())) }
                } else {
                    Value { inner: Cow::Owned(serde_json::json!(s.as_u64().unwrap_or_default() + rhs.as_u64().unwrap_or_default())) }
                }
            }
            _ => {
                return Value { inner: Cow::Owned(serde_json::Value::Null) };
            }
        };
    }
}

impl Add<Value<'_>> for &Value<'_> {
    type Output = Value<'static>;
    fn add(self, rhs: Value) -> Self::Output {
        return match self.inner.as_ref() {
            serde_json::Value::String(s) => {
                Value { inner: Cow::Owned(serde_json::Value::String(s.to_string() + rhs.as_str().unwrap_or(""))) }
            }
            serde_json::Value::Number(s) => {
                if s.is_i64() {
                    Value { inner: Cow::Owned(serde_json::json!(s.as_i64().unwrap_or_default() + rhs.as_i64().unwrap_or_default())) }
                } else if s.is_f64() {
                    Value { inner: Cow::Owned(serde_json::json!(s.as_f64().unwrap_or_default() + rhs.as_f64().unwrap_or_default())) }
                } else {
                    Value { inner: Cow::Owned(serde_json::json!(s.as_u64().unwrap_or_default() + rhs.as_u64().unwrap_or_default())) }
                }
            }
            _ => {
                return Value { inner: Cow::Owned(serde_json::Value::Null) };
            }
        };
    }
}


//serde_json value
impl Add<&serde_json::Value> for Value<'_> {
    type Output = Value<'static>;
    fn add(self, rhs: &serde_json::Value) -> Self::Output {
        let inner = match self.inner {
            Cow::Borrowed(b) => {
                b.to_owned()
            }
            Cow::Owned(o) => {
                o.to_owned()
            }
        };
        return match inner {
            serde_json::Value::String(s) => {
                Value { inner: Cow::Owned(serde_json::Value::String(s + rhs.as_str().unwrap_or(""))) }
            }
            serde_json::Value::Number(s) => {
                if s.is_i64() {
                    Value { inner: Cow::Owned(serde_json::json!(s.as_i64().unwrap_or_default() + rhs.as_i64().unwrap_or_default())) }
                } else if s.is_f64() {
                    Value { inner: Cow::Owned(serde_json::json!(s.as_f64().unwrap_or_default() + rhs.as_f64().unwrap_or_default())) }
                } else {
                    Value { inner: Cow::Owned(serde_json::json!(s.as_u64().unwrap_or_default() + rhs.as_u64().unwrap_or_default())) }
                }
            }
            _ => {
                return Value { inner: Cow::Owned(serde_json::Value::Null) };
            }
        };
    }
}

impl Add<serde_json::Value> for Value<'_> {
    type Output = Value<'static>;
    fn add(self, rhs: serde_json::Value) -> Self::Output {
        let inner = match self.inner {
            Cow::Borrowed(b) => {
                b.to_owned()
            }
            Cow::Owned(o) => {
                o.to_owned()
            }
        };
        return match inner {
            serde_json::Value::String(s) => {
                Value { inner: Cow::Owned(serde_json::Value::String(s + rhs.as_str().unwrap_or(""))) }
            }
            serde_json::Value::Number(s) => {
                if s.is_i64() {
                    Value { inner: Cow::Owned(serde_json::json!(s.as_i64().unwrap_or_default() + rhs.as_i64().unwrap_or_default())) }
                } else if s.is_f64() {
                    Value { inner: Cow::Owned(serde_json::json!(s.as_f64().unwrap_or_default() + rhs.as_f64().unwrap_or_default())) }
                } else {
                    Value { inner: Cow::Owned(serde_json::json!(s.as_u64().unwrap_or_default() + rhs.as_u64().unwrap_or_default())) }
                }
            }
            _ => {
                return Value { inner: Cow::Owned(serde_json::Value::Null) };
            }
        };
    }
}

impl Add<&serde_json::Value> for &Value<'_> {
    type Output = Value<'static>;
    fn add(self, rhs: &serde_json::Value) -> Self::Output {
        return match self.inner.as_ref() {
            serde_json::Value::String(s) => {
                Value { inner: Cow::Owned(serde_json::Value::String(s.to_string() + rhs.as_str().unwrap_or(""))) }
            }
            serde_json::Value::Number(s) => {
                if s.is_i64() {
                    Value { inner: Cow::Owned(serde_json::json!(s.as_i64().unwrap_or_default() + rhs.as_i64().unwrap_or_default())) }
                } else if s.is_f64() {
                    Value { inner: Cow::Owned(serde_json::json!(s.as_f64().unwrap_or_default() + rhs.as_f64().unwrap_or_default())) }
                } else {
                    Value { inner: Cow::Owned(serde_json::json!(s.as_u64().unwrap_or_default() + rhs.as_u64().unwrap_or_default())) }
                }
            }
            _ => {
                return Value { inner: Cow::Owned(serde_json::Value::Null) };
            }
        };
    }
}

impl Add<serde_json::Value> for &Value<'_> {
    type Output = Value<'static>;
    fn add(self, rhs: serde_json::Value) -> Self::Output {
        return match self.inner.as_ref() {
            serde_json::Value::String(s) => {
                Value { inner: Cow::Owned(serde_json::Value::String(s.to_string() + rhs.as_str().unwrap_or(""))) }
            }
            serde_json::Value::Number(s) => {
                if s.is_i64() {
                    Value { inner: Cow::Owned(serde_json::json!(s.as_i64().unwrap_or_default() + rhs.as_i64().unwrap_or_default())) }
                } else if s.is_f64() {
                    Value { inner: Cow::Owned(serde_json::json!(s.as_f64().unwrap_or_default() + rhs.as_f64().unwrap_or_default())) }
                } else {
                    Value { inner: Cow::Owned(serde_json::json!(s.as_u64().unwrap_or_default() + rhs.as_u64().unwrap_or_default())) }
                }
            }
            _ => {
                return Value { inner: Cow::Owned(serde_json::Value::Null) };
            }
        };
    }
}


//str
impl Add<Value<'_>> for &str {
    type Output = String;
    fn add(self, rhs: Value) -> Self::Output {
        return match rhs.inner.as_ref() {
            serde_json::Value::String(s) => {
                self.to_string() + s.as_str()
            }
            _ => {
                String::new()
            }
        };
    }
}

impl Add<&Value<'_>> for &str {
    type Output = String;
    fn add(self, rhs: &Value) -> Self::Output {
        return match rhs.inner.as_ref() {
            serde_json::Value::String(s) => {
                self.to_string() + s.as_str()
            }
            _ => {
                String::new()
            }
        };
    }
}

impl Add<&str> for Value<'_> {
    type Output = String;
    fn add(self, rhs: &str) -> Self::Output {
        let inner = match self.inner {
            Cow::Borrowed(b) => {
                b.to_owned()
            }
            Cow::Owned(o) => {
                o.to_owned()
            }
        };
        return match inner {
            serde_json::Value::String(s) => {
                s + rhs
            }
            _ => {
                String::new()
            }
        };
    }
}

impl Add<&str> for &Value<'_> {
    type Output = String;
    fn add(self, rhs: &str) -> Self::Output {
        return match self.inner.as_ref() {
            serde_json::Value::String(s) => {
                s.to_string() + rhs
            }
            _ => {
                String::new()
            }
        };
    }
}

//string
impl Add<Value<'_>> for String {
    type Output = String;
    fn add(self, rhs: Value) -> Self::Output {
        return match rhs.inner.as_ref() {
            serde_json::Value::String(s) => {
                self + s.as_str()
            }
            _ => {
                String::new()
            }
        };
    }
}

impl Add<&Value<'_>> for String {
    type Output = String;
    fn add(self, rhs: &Value) -> Self::Output {
        return match rhs.inner.as_ref() {
            serde_json::Value::String(s) => {
                self + s.as_str()
            }
            _ => {
                String::new()
            }
        };
    }
}

impl Add<String> for Value<'_> {
    type Output = String;
    fn add(self, rhs: String) -> Self::Output {
        let inner = match self.inner {
            Cow::Borrowed(b) => {
                b.to_owned()
            }
            Cow::Owned(o) => {
                o.to_owned()
            }
        };
        return match inner {
            serde_json::Value::String(s) => {
                s + rhs.as_str()
            }
            _ => {
                String::new()
            }
        };
    }
}

impl Add<String> for &Value<'_> {
    type Output = String;
    fn add(self, rhs: String) -> Self::Output {
        return match self.inner.as_ref() {
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
impl Add<Value<'_>> for &String {
    type Output = String;
    fn add(self, rhs: Value) -> Self::Output {
        return match rhs.inner.as_ref() {
            serde_json::Value::String(s) => {
                self.to_string() + s.as_str()
            }
            _ => {
                String::new()
            }
        };
    }
}

impl Add<&Value<'_>> for &String {
    type Output = String;
    fn add(self, rhs: &Value) -> Self::Output {
        return match rhs.inner.as_ref() {
            serde_json::Value::String(s) => {
                self.to_string() + s.as_str()
            }
            _ => {
                String::new()
            }
        };
    }
}

impl Add<&String> for Value<'_> {
    type Output = String;
    fn add(self, rhs: &String) -> Self::Output {
        let inner = match self.inner {
            Cow::Borrowed(b) => {
                b.to_owned()
            }
            Cow::Owned(o) => {
                o.to_owned()
            }
        };
        return match inner {
            serde_json::Value::String(s) => {
                s + rhs.as_str()
            }
            _ => {
                String::new()
            }
        };
    }
}

impl Add<&String> for &Value<'_> {
    type Output = String;
    fn add(self, rhs: &String) -> Self::Output {
        return match self.inner.as_ref() {
            serde_json::Value::String(s) => {
                s.to_string() + rhs.as_str()
            }
            _ => {
                String::new()
            }
        };
    }
}