//! Contains the lexer which reads constable syntax.

mod token;
mod error_codes;
mod textiter;
pub mod tokenizer;

pub use self::token::{Token, TokenType};
pub use self::textiter::{TextIter, PeekTextIter};
pub use self::tokenizer::{Tokenizer, StaticStrTokenizer, TokenData, TextLocation};
