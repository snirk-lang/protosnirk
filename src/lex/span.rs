//! Position information for the AST

use std::convert::From;
use std::cmp::{PartialOrd, Ord, Ordering};
use std::fmt::{Display, Debug, Formatter, Result as FmtResult};
use std::ops::{RangeInclusive};

/// Represents the location of a single `Token`.
#[derive(PartialEq, Eq, Clone, Copy, Hash, Default)]
pub struct Location {
    /// Index within source text
    index: u32,
    /// Line within source text
    line: u32,
    /// Column within source text
    column: u32,
}

impl Location {
    /// Creates a new `Location` with the given index, line, column, and length.
    pub fn of() -> LocationBuilder {
        LocationBuilder { index: 0, line: 0, column: 0 }
    }

    pub fn offset(self, offset: u32) -> Location {
        Location {
            index: self.index + offset,
            line: self.line,
            column: self.column + offset
        }
    }

    /// The starting index of the token within the source string.
    pub fn index(&self) -> u32 {
        self.index
    }

    /// The line on which the token appears in the source string (1-indexed).
    pub fn line(&self) -> u32 {
        self.line
    }

    /// The column on which the token appears in the source string (1-indexed).
    pub fn column(&self) -> u32 {
        self.column
    }
}

impl Debug for Location {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "(line: {}, col: {})", self.line, self.column)
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "line {}, column {}", self.line, self.column)
    }
}

/// Builder for constructing locations via `Location::build()`
#[derive(Debug, PartialEq)]
pub struct LocationBuilder {
    index: u32,
    line: u32,
    column: u32
}

impl LocationBuilder {
    pub fn index(&mut self, index: u32) -> &mut Self {
        self.index = index;
        self
    }

    pub fn line(&mut self, line: u32) -> &mut Self {
        self.line = line;
        self
    }

    pub fn column(&mut self, column: u32) -> &mut Self {
        self.column = column;
        self
    }

    pub fn build(&mut self) -> Location {
        Location { index: self.index, line: self.line, column: self.column }
    }
}

impl PartialOrd for Location {
    fn partial_cmp(&self, other: &Location) -> Option<Ordering> {
        self.index.partial_cmp(&other.index)
    }
}

impl Ord for Location {
    fn cmp(&self, other: &Location) -> Ordering {
        self.index.cmp(&other.index)
    }
}

/// Represents an area of text which is taken up by a node in the AST.
///
/// Spans may be multiline or represent an expression which uses part of a line.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Default)]
pub struct Span {
    /// Location of the first token in the span
    start: Location,
    /// Location of the last token in the span
    end: Location
}

impl Span {
    /// Creates a new span starting from a point across a number of characters in a line
    pub fn from_location(start: Location, offset: u32) -> Span {
        Span {
            start,
            end: Location {
                index: start.index + offset,
                line: start.line,
                column: start.column + offset
            }
        }
    }

    /// The starting token's location in the span.
    pub fn start(&self) -> Location {
        self.start
    }

    /// The ending token's location in the span.
    pub fn end(&self) -> Location {
        self.end
    }

    /// Total length of this span (in terms of characters)
    pub fn len(&self) -> u32 {
        self.end.index - self.start.index
    }

    // Number of lines in this span
    pub fn lines(&self) -> u32 {
        self.end.line - self.start.line
    }

    pub fn chars(&self) -> u32 {
        self.end.index - self.start.index
    }

    /// Whether this span encompasses multiple lines
    pub fn is_multiline(&self) -> bool {
        self.lines() > 0
    }

    pub fn is_multichar(&self) -> bool {
        self.chars() > 0
    }
}

impl From<RangeInclusive<Location>> for Span {
    fn from(r: RangeInclusive<Location>) -> Span {
        Span { start: *r.start(), end: *r.end() }
    }
}

impl From<RangeInclusive<Span>> for Span {
    fn from(r: RangeInclusive<Span>) -> Span {
        Span { start: r.start().start, end: r.end().end }
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        if self.is_multiline() {
            write!(f, "{} to {}", self.start, self.end)
        }
        else {
            write!(f, "line {}, column {} to {}",
                   self.start.line, self.start.column, self.end.column)
        }
    }
}

impl PartialOrd for Span {
    fn partial_cmp(&self, other: &Span) -> Option<Ordering> {
        (self.start.index, self.end.index).partial_cmp(&(other.start.index, other.end.index))
    }
}

impl Ord for Span {
    fn cmp(&self, other: &Span) -> Ordering {
        (self.start.index, self.end.index).cmp(&(other.start.index, other.end.index))
    }
}
