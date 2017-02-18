//! If block/inline if parser.

use lex::{tokens, Token, Tokenizer, TokenData};
use parse::ast::*;
use parse::{Parser, ParseError, ParseResult};
use parse::symbol::{PrefixParser, Precedence};

/// Parses if blocks and inline if expressions.
///
/// # Examples
/// ```text
/// if expr \+ stmt* \- [else if expr \+ stmt* \-]* [else \+ stmt*]
/// ```
/// If the `=>` is detected signifying an inline if, the parser will
/// call out to `IfExpressionParser` and return that expression in
/// statement form.
///
/// This parser is allowed to assume it can parse an inline if expr
/// instead, but the inline if parser should assume that it is parsing
/// a context where only [inline] expressions are allowed.
#[derive(Debug)]
pub struct IfBlockParser { }
impl<T: Tokenizer> PrefixParser<Statement, T> for IfBlockParser {
    fn parse(&self, parser: &mut Parser<T>, token: Token) -> ParseResult<Statement> {
        debug_assert!(token.get_text() == "if",
            "Invalid token {:?} in IfBlockParser")
    }
}
