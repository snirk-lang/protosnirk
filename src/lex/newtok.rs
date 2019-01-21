//! tokenizer

use lex::{Token, Tokenizer, PeekTextIter, TokenizerError};

/// Internal state of the tokenizer
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum TokenizerState {
    /// Emitting tokens for words until `\n` and emit a newline token.
    TokenizeUntilNewline,
    /// Emitting indentation token to match current indentation
    EmitIndentsUntilTextStart,
    /// Emitting outdentation tokens to match current indentation
    EmitOutdentsForTabChange,
}

#[derive(Debug)]
pub struct IterTokenizer<I> where I: Iterator<Item=char> {
    state: TokenizerState,
    match_indent: bool,
    indents: u32,
    iter: PeekTextIter<I>
}

impl<I: Iterator<Item=char>> IterTokenizer<I> {
    pub fn new(iter: I) -> IterTokenizer<I> {
        IterTokenizer {
            indents: 0,
            match_indent: true,
            state: TokenizerState::TokenizeUntilTextStart,
            iter: PeekTextIter::new(iter.peekable())
        }
    }

    fn peek_or_eof(&mut self) -> Result<char, Token> {
        match self.iter.peek() {
            Some(c) => Ok(c),
            None => Err(Token::new_eof(self.iter.location()))
        }
    }

    /// Produce the next token
    pub fn next(&mut self) -> Result<Token, TokenizerError> {
        use self::TokenizerState::*;
        trace!("next(): peeked {:?}", self.iter.peek());
        let peeked = try!(self.peek_or_eof());
        if peeked == '\t' {
            return Err(
                TokenizerError::TabCharacterFound(self.iter.location()))
        }
        match self.tokenizer_state {
            TokenizeUntilNewline => self.next_until_newline(),
            EmitIndentsUntilTextStart => self.next_indent(),
            EmitOutdentsForTabChange => self.next_outdent()
        }
    }

    pub fn next_until_newline(&mut self) -> Token {
        trace!("Emitting tokens until a newline");

    }

    pub fn next_indent(&mut self) -> Token {
        trace!("Emitting indentation");
        let mut raw_space_count = 0;
        let mut peeked = try!(self.peek_or_eof());

        // Gather up all the next spaces
        while peeked == ' ' {
            raw_space_count += 1;
            match self.iter.peek() {
                Ok(c) => { peeked = c; },
                None => break
            }
        }


    }

    pub fn next_outdent(&mut self) -> Token {
        // fn foo():
        //     let x = 0
        // >

    }
}

impl<I: Iterator<Item=char>> Tokenizer for IterTokenizer<I> {
    fn next(&mut self) -> Token {
        let next = self.next();
        trace!("Emitting {:?}", next);
        next
    }
}
