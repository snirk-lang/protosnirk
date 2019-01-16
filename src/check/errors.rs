//! Result types for Verification

use lex::Span;

/// Compiler error returned by an expression verifier.
///
/// Whether this error is actually a warning or lint depends on
/// compiler options. Errors are collected in an `ErrorCollector`.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CheckerError {
    spans: Vec<Span>,
    text: String,
}
impl CheckerError {
    pub fn new(spans: Vec<Span>,
               text: String) -> CheckerError {
        CheckerError { spans, text }
    }
    pub fn offender(&self) -> Option<Span> {
        self.spans.first().cloned()
    }
    pub fn spans(&self) -> &[Span] {
        &self.spans
    }
    pub fn text(&self) -> &str {
        &self.text
    }
}
