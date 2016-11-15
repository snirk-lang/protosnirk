//! A basic tokenizer using the `unicode_segmentation` crate.
//!
//! This tokenizer must be passed the entire input before it is able to tokenize.

use std::collections::HashMap;

use unicode_categories::UnicodeCategories;
use unicode_segmentation::{UnicodeSegmentation, GraphemeIndices};

use lex::{Token, TokenType, Tokenizer};

/// Generates a stream of tokens from a string.
/// Currently only good at recogniz
pub struct GraphemeIndicesTokenizer<'a> {
    iter: GraphemeIndices<'a>,
    indent: usize,
    indent_spaces: usize,
    index: usize,
    token_map: HashMap<&'static str, TokenType>
}
impl<'a> GraphemeIndicesTokenizer<'a> {
    /// Create a GraphemeIndicesTokenizer from the given `String`.
    ///
    /// The tokenizer will have a lifetime shorter than that of its input.
    pub fn from_string(input: &'a str) -> GraphemeIndicesTokenizer<'a> {
        let token_map = hashmap! [
            "=" => TokenType::Assign,
            "<" => TokenType::LeftAngle,
            "[" => TokenType::LeftBrace,
            "(" => TokenType::LeftParen,
            "{" => TokenType::LeftSquiggle,
            "let" => TokenType::Let,
            "-" => TokenType::Minus,
            "mut" => TokenType::Mut,
            "%" => TokenType::Percent,
            "+" => TokenType::Plus,
            "return" => TokenType::Return,
            ">" => TokenType::RightAngle,
            "]" => TokenType::RightBrace,
            ")" => TokenType::RightParen,
            "}" => TokenType::RightSquiggle,
            "/" => TokenType::Slash,
            "*" => TokenType::Star,
        ];

        GraphemeIndicesTokenizer {
            iter: input.grapheme_indices(false),
            index: 0,
            indent: 0,
            indent_spaces: 0,
            token_map: token_map
        }
    }
}

impl<'a> Tokenizer for GraphemeIndicesTokenizer<'a> {
    fn next(&mut self) -> Token {
        let mut state = TokenizerState::Begin;
        let mut acc = String::new();

        while let Some((size, slice)) = self.iter.next() {
            let start = self.index;
            self.index += size;
            debug_assert!(slice.len() > 0, "Unexpected 0 length char");
            let basic_char = slice.chars().next()
                .expect("Checked expect: >0 sized string");

            match state {
                TokenizerState::Begin => {
                    if basic_char.is_letter() {
                        state = TokenizerState::AccumulateWord;
                        acc.push_str(slice);
                        continue;
                    // For now we are ignoring all whitespace.
                    // Eventually stateful things will have to be done.
                    } else if basic_char.is_whitespace() {
                        continue;
                    } else if basic_char.is_number() {
                        state = TokenizerState::AccumulateLiteral;
                    // Check for tokens
                    } else if let Some(token_type) = self.token_map.get(slice) {
                        return Token::new(start, self.index, slice, *token_type)
                    }
                },
                TokenizerState::AccumulateWord => {
                    if basic_char.is_whitespace() {
                        if let Some(token_type) = self.token_map.get(slice) {
                            return Token::new(start, self.index, acc, *token_type)
                        } else {
                            return Token::new(start, self.index, acc, TokenType::Identifier)
                        }
                    // Continuing to parse an identifier
                    } else if basic_char == '_' || basic_char.is_letter() || basic_char.is_number() {
                        
                    }
                }
            }

        }
        return Token::end(self.index);

        unimplemented!();
    }
}

enum TokenizerState {
    Begin,
    AccumulateWord,
    AccumulateLiteral,
}
