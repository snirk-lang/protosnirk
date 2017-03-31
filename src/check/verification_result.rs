//! Result types for Verification

use lex::{Token};

/// Compiler error returned by an expression verifier.
///
/// Whether this error is actually a warning or lint depends on
/// compiler options. Errors are collected in an `ErrorCollector`.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct VerifyError {
    //err: ErrorCode,
    offender: Token,
    references: Vec<Token>,
    text: String,
}
impl VerifyError {
    pub fn new(offender: Token, references: Vec<Token>, text: String) -> VerifyError {
        VerifyError {
            offender: offender,
            references: references,
            text: text,
        }
    }
    pub fn get_offender(&self) -> &Token {
        &self.offender
    }
    pub fn get_references(&self) -> &[Token] {
        &self.references
    }
    pub fn get_text(&self) -> &str {
        &self.text
    }
}
