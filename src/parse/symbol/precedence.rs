//! Symbol definitions for Pratt parsing

use std::mem;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Precedence {
    /// Extra value on the end
    Min,
    /// Return <expr> statements
    Return,
    /// Assignment and declaration statements
    Assign,
    ///  The `==` and `!=` operators
    Equality,
    /// Less than and greater than
    EqualityCompare,
    /// Add and subtract infix expressions
    AddSub,
    /// Multiply and divide infix expressions
    MulDiv,
    /// The remainder operator
    Modulo,
    /// Negate or positive operator
    NumericPrefix,
    /// The `not` keyword
    NotKeyword,
    /// Parens binder, used for both prefix and infix fns
    Paren,
    /// Extra value on the end
    Max
}

#[cfg(test)]
mod test {
    use std::mem;
    use super::*;

    #[test]
    fn it_has_derived_ord() {
        assert!(Precedence::Max > Precedence::Min);
        assert!(Precedence::MulDiv > Precedence::AddSub);
    }
}
