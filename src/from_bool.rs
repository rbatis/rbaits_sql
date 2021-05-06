pub use crate::Value;

impl From<&Value<'_>> for bool{
    fn from(arg: &Value) -> Self {
        arg.inner.as_bool().unwrap_or_default()
    }
}

impl From<Value<'_>> for bool{
    fn from(arg: Value) -> Self {
        arg.inner.as_bool().unwrap_or_default()
    }
}
