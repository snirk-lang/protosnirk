//! Lexer definition.

use std::iter::{Iterator, Peekable};

pub struct Lexer {

}

impl Lexer {
    pub fn next<'a>(&'a mut self) -> Token<'a> {
        for i in self.current .. self.text.len() {
            if is_alphanum(self.text[i]) {
                // begin parsing a keyword

            }
        }
    }
}

enum State {
    Start,
    Keyword,
    Ident,
    Space
}
