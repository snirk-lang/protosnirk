//! Statement parsers
//!
//! Statements are parsed in the parser's `statement()` method.
//! If a statement keyword parser is found (`let`, `if`, `do`, etc.)
//! then that statement is called, else the parser returns `Statement::Expression`
//! by calling its `expression()` method.
//! Note that assignment becomes a statement in this case.

/// Parser for a given statement.
pub trait StatementSymbol<T: Tokenizer> {
    /// Parse a statement from the given prefix token.
    fn parse(&mut self, parser: &mut Parser<T>) -> ParseResult<Statement>;
}
