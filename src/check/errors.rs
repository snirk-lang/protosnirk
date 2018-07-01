//! Result types for Verification

use lex::{Token};

/// Compiler error returned by an expression verifier.
///
/// Whether this error is actually a warning or lint depends on
/// compiler options. Errors are collected in an `ErrorCollector`.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CheckerError {
    //err: ErrorCode,
    offender: Token,
    references: Vec<Token>,
    text: String,
}
impl CheckerError {
    pub fn new(offender: Token,
               references: Vec<Token>,
               text: String) -> CheckerError {
        CheckerError {
            offender: offender,
            references: references,
            text: text,
        }
    }
    pub fn offender(&self) -> &Token {
        &self.offender
    }
    pub fn references(&self) -> &[Token] {
        &self.references
    }
    pub fn text(&self) -> &str {
        &self.text
    }
}
