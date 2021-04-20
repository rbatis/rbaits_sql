use std::ops::Not;

use crate::Value;

impl Not for Value {
    type Output = bool;

    fn not(self) -> Self::Output {
        match self.inner {
            serde_json::Value::Bool(b) => { !b }
            _ => { true }
        }
    }
}

impl Not for &Value {
    type Output = bool;
    fn not(self) -> Self::Output {
        match self.inner {
            serde_json::Value::Bool(b) => { !b }
            _ => { true }
        }
    }
}

impl Not for &mut Value {
    type Output = bool;
    fn not(self) -> Self::Output {
        match self.inner {
            serde_json::Value::Bool(b) => { !b }
            _ => { true }
        }
    }
}