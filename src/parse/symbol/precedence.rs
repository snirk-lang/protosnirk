//! Symbol definitions for Pratt parsing

use lex::TokenType;

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

impl Precedence {
    /// Source of truth for precedence in parsing expressions
    pub fn for_token(token_type: TokenType, prefix: bool) -> Precedence {
        use self::TokenType::*;
        match token_type {
            Return => Precedence::Return,
            Equals
            | PlusEquals
            | MinusEquals
            | StarEquals
            | SlashEquals
            | PercentEquals => Precedence::Assign,
            DoubleEquals | NotEquals => Precedence::Equality,
            LeftAngle | RightAngle | LessThanEquals | GreaterThanEquals => Precedence::EqualityCompare,
            Plus | Minus => {
                if prefix {Precedence::NumericPrefix }
                else {Precedence::AddSub }
            },
            Star | Slash => Precedence::MulDiv,
            Percent => Precedence::Modulo,
            LeftParen => Precedence::Paren,
            _ => Precedence::Min
        }
    }
}
