pub use crate::Value;

impl From<&Value> for bool{
    fn from(arg: &Value) -> Self {
        arg.inner.as_bool().unwrap_or_default()
    }
}

impl From<Value> for bool{
    fn from(arg: Value) -> Self {
        arg.inner.as_bool().unwrap_or_default()
    }
}
