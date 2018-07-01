//! Assign operator parser.

use lex::{Token, Tokenizer};
use ast::*;
use parse::{Parser, ParseResult};
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
        let operator = try!(parser.binary_operator(token.get_type()));
        // We parse it here into an expanded expression.
        let right_expr = Expression::BinaryOp(BinaryOperation::new(
            operator,
            token,
            Box::new(Expression::VariableRef(lvalue.clone())),
            Box::new(right_value)));
        Ok(Expression::Assignment(Assignment::new(lvalue, Box::new(right_expr))))
    }
    fn precedence(&self) -> Precedence {
        Precedence::Assign
    }
}
