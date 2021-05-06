use std::ops::BitOr;
use crate::Value;

impl BitOr for Value<'_>{
    type Output = bool;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.as_bool().unwrap_or(false) | rhs.as_bool().unwrap_or(false)
    }
}

impl BitOr<Value<'_>> for bool{
    type Output = bool;

    fn bitor(self, rhs: Value) -> Self::Output {
        self | rhs.as_bool().unwrap_or(false)
    }
}

//ref
impl BitOr<Value<'_>> for &Value<'_>{
    type Output = bool;

    fn bitor(self, rhs: Value) -> Self::Output {
        self.as_bool().unwrap_or(false) | rhs.as_bool().unwrap_or(false)
    }
}
impl BitOr<&Value<'_>> for &Value<'_>{
    type Output = bool;

    fn bitor(self, rhs: &Value) -> Self::Output {
        self.as_bool().unwrap_or(false) | rhs.as_bool().unwrap_or(false)
    }
}

impl BitOr<bool> for &Value<'_>{
    type Output = bool;

    fn bitor(self, rhs: bool) -> Self::Output {
        self.as_bool().unwrap_or(false) | rhs
    }
}

//refmut
impl BitOr<Value<'_>> for &mut Value<'_>{
    type Output = bool;

    fn bitor(self, rhs: Value) -> Self::Output {
        self.as_bool().unwrap_or(false) | rhs.as_bool().unwrap_or(false)
    }
}
impl BitOr<&Value<'_>> for &mut Value<'_>{
    type Output = bool;

    fn bitor(self, rhs: &Value) -> Self::Output {
        self.as_bool().unwrap_or(false) | rhs.as_bool().unwrap_or(false)
    }
}

impl BitOr<bool> for &mut Value<'_>{
    type Output = bool;

    fn bitor(self, rhs: bool) -> Self::Output {
        self.as_bool().unwrap_or(false) | rhs
    }
}

//rhs ref
impl BitOr<&Value<'_>> for Value<'_>{
    type Output = bool;

    fn bitor(self, rhs: &Value) -> Self::Output {
        self.as_bool().unwrap_or(false) | rhs.as_bool().unwrap_or(false)
    }
}
impl BitOr<&Value<'_>> for bool{
    type Output = bool;

    fn bitor(self, rhs: &Value) -> Self::Output {
        self | rhs.as_bool().unwrap_or(false)
    }
}

//rhs ref mut
impl BitOr<&mut Value<'_>> for Value<'_>{
    type Output = bool;

    fn bitor(self, rhs: &mut Value) -> Self::Output {
        self.as_bool().unwrap_or(false) | rhs.as_bool().unwrap_or(false)
    }
}
impl BitOr<&mut Value<'_>> for bool{
    type Output = bool;

    fn bitor(self, rhs: &mut Value) -> Self::Output {
        self | rhs.as_bool().unwrap_or(false)
    }
}