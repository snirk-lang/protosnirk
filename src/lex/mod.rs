//! Contains the lexer which reads constable syntax.

mod token;
pub mod tokens;
mod error_codes;
mod textiter;
pub mod tokenizer;

#[cfg(test)]
mod tests;

pub use self::token::{Token, TokenType, TokenData, TextLocation};
pub use self::textiter::{TextIter, PeekTextIter};
pub use self::tokenizer::{Tokenizer, IterTokenizer};

/// Type representing a borrowed or owned string
pub type CowStr = ::std::borrow::Cow<'static, str>;
