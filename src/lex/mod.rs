//! Contains the lexer which reads protosnirk syntax.

mod token;
pub mod tokens;
mod textiter;
pub mod tokenizer;

#[cfg(test)]
pub mod tests;

pub use self::token::{Token, TokenData};
pub use self::tokens::TokenType;
pub use self::textiter::{TextLocation, TextIter, PeekTextIter};
pub use self::tokenizer::{Tokenizer, IterTokenizer};

/// Type representing a borrowed or owned string
pub type CowStr = ::std::borrow::Cow<'static, str>;

/// Rule for the symbol list indicating to the tokenizer
/// how to parse symbols.
#[derive(Debug, Clone, Copy)]
pub enum TokenizerSymbolRule {
    /// The symbol is completely parsed.
    Complete,
    /// The symbol is partially parsed.
    /// More characters *must* match for a complete symbol.
    Partial,
    /// The parse is a complete symbol, but it could be part of
    /// another symbol depending on the characters after it.
    CompletePrefix
}
