//! Contains the lexer which reads constable syntax.

mod token;
mod error_codes;
mod tokenizer;

#[cfg(test)]
mod tests;

pub use self::token::{Token, TokenType};
pub use self::tokenizer::{Tokenizer, StaticStrTokenizer, TokenData};
