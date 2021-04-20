use std::ops::BitAnd;
use crate::Value;

impl BitAnd for Value{
    type Output = bool;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.as_bool().unwrap_or(false) & rhs.as_bool().unwrap_or(false)
    }
}
impl BitAnd<&Value> for Value{
    type Output = bool;

    fn bitand(self, rhs: &Value) -> Self::Output {
        self.as_bool().unwrap_or(false) & rhs.as_bool().unwrap_or(false)
    }
}

impl BitAnd<Value> for &Value{
    type Output = bool;

    fn bitand(self, rhs: Value) -> Self::Output {
        self.as_bool().unwrap_or(false) & rhs.as_bool().unwrap_or(false)
    }
}

impl BitAnd<&Value> for &Value{
    type Output = bool;

    fn bitand(self, rhs: &Value) -> Self::Output {
        self.as_bool().unwrap_or(false) & rhs.as_bool().unwrap_or(false)
    }
}

impl BitAnd<bool> for &Value{
    type Output = bool;

    fn bitand(self, rhs: bool) -> Self::Output {
        self.as_bool().unwrap_or(false) & rhs
    }
}

impl BitAnd<Value> for bool{
    type Output = bool;

    fn bitand(self, rhs: Value) -> Self::Output {
        self & rhs.as_bool().unwrap_or(false)
    }
}
impl BitAnd<&Value> for bool{
    type Output = bool;

    fn bitand(self, rhs: &Value) -> Self::Output {
        self & rhs.as_bool().unwrap_or(false)
    }
}