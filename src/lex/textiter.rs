//! Trait for iterating over text

use std::iter::{Iterator, Peekable};

use lex::Location;

/// A specialized iterator (for tokenizing) which also implements `peek()`
/// and keeps track of its location.
pub trait TextIter : Iterator {
    fn peek(&mut self) -> Option<char>;
    /// Current location of the iterator.
    fn location(&self) -> Location;
}

/// A `TextIter` which uses an internal `Peekable<T>`.
#[derive(Debug, Clone)]
pub struct PeekTextIter<T> where T: Iterator<Item=char> {
    /// Iterator which does most of the work
    iter: Peekable<T>,
    /// Current line in the source
    current_line: u32,
    /// Current column in the source
    current_column: u32,
    /// Current byte in the source
    current_char: u32
}
impl<T: Iterator<Item=char>> PeekTextIter<T> {
    pub fn new(iter: Peekable<T>) -> PeekTextIter<T> {
        PeekTextIter {
            iter: iter,
            current_line: 0,
            current_column: 0,
            current_char: 0
        }
    }
}

impl<T: Iterator<Item=char>> TextIter for PeekTextIter<T> {
    fn peek(&mut self) -> Option<char> {
        self.iter.peek().cloned()
    }

    fn location(&self) -> Location {
        Location::of()
            .index(self.current_char)
            .line(self.current_line)
            .column(self.current_column)
            .build()
    }
}

impl<T: Iterator<Item=char>> Iterator for PeekTextIter<T> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        let result = self.iter.next();
        self.current_char = self.current_char.saturating_add(1);
        match result {
            Some('\n') => {
                self.current_line = self.current_line.saturating_add(1);
                self.current_column = 0;
            },
            Some(_) => {
                self.current_column = self.current_column.saturating_add(1);
            },
            None => {}
        }
        trace!("> Next char {:?}", result);
        result
    }

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_starts_at_zero() {
        //let empty_peek = "".into().into_iter().peekable();
        //let empty_textiter = PeekTextIter::new(empty_peek);
    }
}
