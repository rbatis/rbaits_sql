use std::ops::BitAnd;
use crate::Value;

impl BitAnd for Value{
    type Output = bool;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.as_bool().unwrap_or(false) & rhs.as_bool().unwrap_or(false)
    }
}

impl BitAnd<Value> for bool{
    type Output = bool;

    fn bitand(self, rhs: Value) -> Self::Output {
        self & rhs.as_bool().unwrap_or(false)
    }
}

//ref value
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

//ref value mut
impl BitAnd<Value> for &mut Value{
    type Output = bool;

    fn bitand(self, rhs: Value) -> Self::Output {
        self.as_bool().unwrap_or(false) & rhs.as_bool().unwrap_or(false)
    }
}

impl BitAnd<&Value> for &mut Value{
    type Output = bool;

    fn bitand(self, rhs: &Value) -> Self::Output {
        self.as_bool().unwrap_or(false) & rhs.as_bool().unwrap_or(false)
    }
}

impl BitAnd<bool> for &mut Value{
    type Output = bool;

    fn bitand(self, rhs: bool) -> Self::Output {
        self.as_bool().unwrap_or(false) & rhs
    }
}

//rhs ref
impl BitAnd<&Value> for Value{
    type Output = bool;

    fn bitand(self, rhs: &Value) -> Self::Output {
        self.as_bool().unwrap_or(false) & rhs.as_bool().unwrap_or(false)
    }
}

impl BitAnd<&Value> for bool{
    type Output = bool;

    fn bitand(self, rhs: &Value) -> Self::Output {
        self & rhs.as_bool().unwrap_or(false)
    }
}

//rhs ref mut
impl BitAnd<&mut Value> for Value{
    type Output = bool;

    fn bitand(self, rhs: &mut Value) -> Self::Output {
        self.as_bool().unwrap_or(false) & rhs.as_bool().unwrap_or(false)
    }
}

impl BitAnd<&mut Value> for bool{
    type Output = bool;

    fn bitand(self, rhs: &mut Value) -> Self::Output {
        self & rhs.as_bool().unwrap_or(false)
    }
}