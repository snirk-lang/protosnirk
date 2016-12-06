//! Value type in register

use std::ops::*;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Value(pub f64);

macro_rules! impl_op {
    ($typ:ty, $fun:ident) => {
        impl $typ for Value {
            type Output = Self;
            fn $fun(self, other: Self) -> Self {
                Value(f64::$fun(self.0, other.0))
            }
        }
    };
}

impl_op!(Add, add);
impl_op!(Sub, sub);
impl_op!(Div, div);
impl_op!(Mul, mul);
impl_op!(Rem, rem);
