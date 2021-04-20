use std::ops::BitOr;
use crate::Value;

impl BitOr for Value{
    type Output = bool;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.as_bool().unwrap_or(false) | rhs.as_bool().unwrap_or(false)
    }
}
impl BitOr<&Value> for Value{
    type Output = bool;

    fn bitor(self, rhs: &Value) -> Self::Output {
        self.as_bool().unwrap_or(false) | rhs.as_bool().unwrap_or(false)
    }
}
impl BitOr<Value> for &Value{
    type Output = bool;

    fn bitor(self, rhs: Value) -> Self::Output {
        self.as_bool().unwrap_or(false) | rhs.as_bool().unwrap_or(false)
    }
}
impl BitOr<&Value> for &Value{
    type Output = bool;

    fn bitor(self, rhs: &Value) -> Self::Output {
        self.as_bool().unwrap_or(false) | rhs.as_bool().unwrap_or(false)
    }
}

impl BitOr<bool> for &Value{
    type Output = bool;

    fn bitor(self, rhs: bool) -> Self::Output {
        self.as_bool().unwrap_or(false) | rhs
    }
}

impl BitOr<Value> for bool{
    type Output = bool;

    fn bitor(self, rhs: Value) -> Self::Output {
        self | rhs.as_bool().unwrap_or(false)
    }
}
impl BitOr<&Value> for bool{
    type Output = bool;

    fn bitor(self, rhs: &Value) -> Self::Output {
        self | rhs.as_bool().unwrap_or(false)
    }
}