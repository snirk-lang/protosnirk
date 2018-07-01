//! Symbol definitions for Pratt parsing

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
