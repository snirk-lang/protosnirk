//! Symbol definitions for Pratt parsing

use std::mem;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Precedence {
    /// Extra value on the end
    Min,
    /// Return <expr> statements
    Return,
    /// Assignment and declaration statements
    Assign,
    /// Add and subtract infix expressions
    AddSub,
    /// Multiply and divide infix expressions
    MulDiv,
    /// The remainder operator
    Modulo,
    /// Negate or positive operator
    NumericPrefix,
    /// Parens binder
    Paren,
    /// Extra value on the end
    Max
}

impl Precedence {
    /// Get an operator precedence one bigger than the current one
    pub fn bigger(&self) -> Self {
        debug_assert!(*self != Precedence::Max,
                      "Cannot increment Precedence::Max");
        let num_self = *self as u8;
        unsafe {
            mem::transmute::<u8, Precedence>(num_self + 1)
        }
    }

    /// Get an operator precedence one smaller than the current one
    pub fn smaller(&self) -> Self {
        debug_assert!(*self != Precedence::Min,
                      "Cannot decrement Precedence::Min");
        let num_self = *self as u8;
        unsafe {
            mem::transmute::<u8, Precedence>(num_self - 1)
        }
    }
}

// macros for unexpected/etc symbols

#[cfg(test)]
mod test {
    use std::mem;
    use super::*;

    #[test]
    fn it_has_derived_ord() {
        assert!(Precedence::Max > Precedence::Min);
        assert!(Precedence::MulDiv > Precedence::AddSub);
    }

    #[test]
    fn it_is_repr_as_a_u8() {
        let _value = Precedence::Max as u8;
        assert_eq!(mem::size_of::<Precedence>(), mem::size_of::<u8>());
    }

    #[test]
    fn it_makes_a_bigger_precedence() {
        let min = Precedence::Min;
        assert!(min.bigger() == Precedence::Return);
    }
}
