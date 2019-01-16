//! Return statement parser

use lex::{tokens, Token, Tokenizer, TokenType};
use ast::*;
use parse::{Parser, ParseResult};
use parse::symbol::{PrefixParser, Precedence};

/// Parses return statements
///
/// # Examples
/// ```text
/// return x + 1 + 3 * 4
///   ^    ->right:expression
/// ```
#[derive(Debug)]
pub struct ReturnParser { }
impl<T: Tokenizer> PrefixParser<Statement, T> for ReturnParser {
    fn parse(&self, parser: &mut Parser<T>, token: Token) -> ParseResult<Statement> {
        debug_assert!(token.text() == tokens::Return,
                      "Return parser called with non-return {:?}", token);
        let start = token.location();
        // If the next statement is on a newline then empty return.
        // Also empty return if next token is deindent
        // Should also check for an indent block to ensure sprious indentation is an error.
        if parser.peek_is_newline(&token) {
            return Ok(Statement::Return(Return::new(start, None)))
        }
        else if parser.peek().get_type() == TokenType::EOF {
            return Ok(Statement::Return(Return::new(start, None)))
        }
        let inner_expr = try!(parser.expression(Precedence::Return));
        let inner = try!(inner_expr.expect_value());
        Ok(Statement::Return(Return::new(start, Box::new(inner))))
    }
}
