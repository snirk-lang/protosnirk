//! Assignment parser

use lex::{Token, Tokenizer, TokenType};
use ast::*;
use parse::{Parser, ParseResult};
use parse::parsers::{InfixParser, Precedence};

/// Parses an assignment expresion.
///
/// # Examples
/// ```text
///   x    =   y + 2
/// (left) ^ ->right:expression
/// ```
#[derive(Debug)]
pub struct AssignmentParser { }
impl<T: Tokenizer> InfixParser<Expression, T> for AssignmentParser {
    fn parse(&self, parser: &mut Parser<T>,
             left: Expression, _token: Token) -> ParseResult<Expression> {
        debug_assert!(_token.get_type() == TokenType::Equals,
            "Assign parser called with non-assign token {:?}", _token);
        let ident = try!(left.expect_identifier());
        let right_expr = try!(parser.expression(Precedence::Assign));
        let right = try!(right_expr.expect_value());
        Ok(Expression::Assignment(Assignment::new(ident, Box::new(right))))
    }
}
