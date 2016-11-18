//! Trait for iterating over text

use lex::TextLocation;
use std::iter::{Iterator, Peekable};

/// A specialized iterator (for tokenizing) which also implements `peek()`
/// and keeps track of its location.
pub trait TextIter : Iterator {
    fn peek(&mut self) -> Option<char>;
    fn next(&mut self) -> Option<char>;
    fn get_location(&self) -> TextLocation;
}

/// A `TextIter` which uses an internal `Peekable<T>`.
pub struct PeekTextIter<T> where T: Iterator<Item=char> {
    /// Iterator which does most of the work
    iter: Peekable<T>,
    /// Current line in the source
    line: usize,
    /// Current column in the source
    column: usize,
    /// Current byte in the source
    byte: usize
}

impl<T: Iterator<Item=char>> Iterator for PeekTextIter<T> {
    type Item = char;
    fn next(&mut self) -> Option<char> {
        self.iter.next()
    }
}

impl<T: Iterator<Item=char>> TextIter for PeekTextIter<T> {
    fn peek(&mut self) -> Option<char> {
        self.iter.peek().cloned()
    }
    fn next(&mut self) -> Option<char> {
        let result = self.iter.next();
        self.byte.saturating_add(1);
        match result {
            Some('\n') => {
                self.line.saturating_add(1);
                self.column = 0;
            },
            Some(_) => {
                self.column.saturating_add(1);
            },
            None => {}
        }
        result
    }
    fn get_location(&self) -> TextLocation {
        TextLocation {
            start_byte: self.byte,
            line: self.line,
            column: self.column
        }
    }
}
