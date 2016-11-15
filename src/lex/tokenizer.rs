//! The tokenizer creates a stream of tokens for the parsers to turn into expressions

use lex::{Token, TokenType};

/// Trait for a tokenizer which can iterate over tokens.
pub trait Tokenizer {
    fn next(&mut self) -> Token;
}
