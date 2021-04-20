use std::ops::BitOr;
use crate::Value;

impl BitOr for Value{
    type Output = bool;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.as_bool().unwrap_or(false) | rhs.as_bool().unwrap_or(false)
    }
}

impl BitOr<Value> for bool{
    type Output = bool;

    fn bitor(self, rhs: Value) -> Self::Output {
        self | rhs.as_bool().unwrap_or(false)
    }
}

//ref
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

//refmut
impl BitOr<Value> for &mut Value{
    type Output = bool;

    fn bitor(self, rhs: Value) -> Self::Output {
        self.as_bool().unwrap_or(false) | rhs.as_bool().unwrap_or(false)
    }
}
impl BitOr<&Value> for &mut Value{
    type Output = bool;

    fn bitor(self, rhs: &Value) -> Self::Output {
        self.as_bool().unwrap_or(false) | rhs.as_bool().unwrap_or(false)
    }
}

impl BitOr<bool> for &mut Value{
    type Output = bool;

    fn bitor(self, rhs: bool) -> Self::Output {
        self.as_bool().unwrap_or(false) | rhs
    }
}

//rhs ref
impl BitOr<&Value> for Value{
    type Output = bool;

    fn bitor(self, rhs: &Value) -> Self::Output {
        self.as_bool().unwrap_or(false) | rhs.as_bool().unwrap_or(false)
    }
}
impl BitOr<&Value> for bool{
    type Output = bool;

    fn bitor(self, rhs: &Value) -> Self::Output {
        self | rhs.as_bool().unwrap_or(false)
    }
}

//rhs ref mut
impl BitOr<&mut Value> for Value{
    type Output = bool;

    fn bitor(self, rhs: &mut Value) -> Self::Output {
        self.as_bool().unwrap_or(false) | rhs.as_bool().unwrap_or(false)
    }
}
impl BitOr<&mut Value> for bool{
    type Output = bool;

    fn bitor(self, rhs: &mut Value) -> Self::Output {
        self | rhs.as_bool().unwrap_or(false)
    }
}