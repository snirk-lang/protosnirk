//! Value type in register

use std::ops::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Value {
    Null,
    Number(i32)
}

macro_rules! impl_op {
    ($typ:ty, $fun:ident) => {
        impl $typ for Value {
            type Output = Self;
            fn $fun(self, other: Self) -> Self {
                match (self, other) {
                    (Value::Number(num), Value::Number(other)) =>
                        Value::Number(i32::$fun(num, other)),
                    _ => panic!("Expected to compare empty values!")
                }
            }
        }
    };
}

impl_op!(Add, add);
impl_op!(Sub, sub);
impl_op!(Div, div);
impl_op!(Mul, mul);
impl_op!(Rem, rem);
