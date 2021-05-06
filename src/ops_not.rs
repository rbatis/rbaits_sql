use std::ops::Not;

use crate::Value;

impl Not for Value<'_> {
    type Output = bool;

    fn not(self) -> Self::Output {
        match self.inner.as_ref() {
            serde_json::Value::Bool(b) => { !*b }
            _ => { true }
        }
    }
}

impl Not for &Value<'_> {
    type Output = bool;
    fn not(self) -> Self::Output {
        match self.inner.as_ref() {
            serde_json::Value::Bool(b) => { !*b }
            _ => { true }
        }
    }
}

impl Not for &mut Value<'_> {
    type Output = bool;
    fn not(self) -> Self::Output {
        match self.inner.as_ref() {
            serde_json::Value::Bool(b) => { !*b }
            _ => { true }
        }
    }
}