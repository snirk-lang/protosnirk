//! Assign operator parser.

use lex::{Token, Tokenizer, TokenType, TokenData};
use ast::*;
use parse::{Parser, ParseResult, ParseError};
use parse::symbol::{InfixParser, Precedence};

/// Parses expresisons using the expr/assign style operators.
///
/// This parser will actually generate a regular `Assignment` expression, desugaring
/// the assignment+operation
/// # Examples
/// ```text
/// x        +=   5
/// ^lvalue  ^op  ^rvalue
/// ```
/// This will be parsed as `Assignment { Var { 'x' }, BinaryOp { +, Var { x }, Literal(5) } }`
#[derive(Debug)]
pub struct AssignOpParser { }
impl<T: Tokenizer> InfixParser<Expression, T> for AssignOpParser {
    fn parse(&self, parser: &mut Parser<T>,
             left: Expression, token: Token) -> ParseResult<Expression> {
        let lvalue = try!(left.expect_identifier());
        let right_expr = try!(parser.expression(Precedence::Min));
        let right_value = try!(right_expr.expect_value());
        let operator = try!(parser.operator(token.get_type(), &token.text));
        // We parse it here into an expanded expression.
        let right_expr = Expression::BinaryOp(BinaryOperation::new(
            operator,
            token,
            Box::new(Expression::VariableRef(lvalue.clone())),
            Box::new(right_value)));
        Ok(Expression::Assignment(Assignment::new(lvalue, Box::new(right_expr))))
    }
    fn get_precedence(&self) -> Precedence {
        Precedence::Assign
    }
}
