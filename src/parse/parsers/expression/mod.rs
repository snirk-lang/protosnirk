mod literal;
mod identifier;
mod parens;
mod assignment;
mod assign_op;
mod if_expr;
mod fn_call;

pub use self::literal::LiteralParser;
pub use self::identifier::IdentifierParser;
pub use self::parens::ParensParser;
pub use self::assignment::AssignmentParser;
pub use self::assign_op::AssignOpParser;
pub use self::if_expr::IfExpressionParser;
pub use self::fn_call::FnCallParser;

use lex::{Token, Tokenizer};
use parse::{Parser, ParseResult};
use ast::*;
use parse::parsers::{Precedence, InfixParser, PrefixParser};

/// A parser which parses symbols used for binary operators.
///
/// Instances of this parser return `BinaryExpression`s.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BinOpExprSymbol { }

impl<T: Tokenizer> InfixParser<Expression, T> for BinOpExprSymbol {
    /// Parses a binary operator expression.
    fn parse(&self, parser: &mut Parser<T>,
             left: Expression, token: Token) -> ParseResult<Expression> {
        let precedence = Precedence::for_token(token.get_type(), false);
        let right: Expression = try!(parser.expression(precedence));
        let bin_operator = try!(parser.binary_operator(token.get_type()));
        Ok(Expression::BinaryOp(
            BinaryOperation::new(bin_operator, Box::new(left), Box::new(right))))
    }
}

/// Unary operator parser.
///
/// Returns a unary operator with the given token type and following expression
#[derive(Debug, PartialEq, Clone)]
pub struct UnaryOpExprSymbol { }

impl<T: Tokenizer> PrefixParser<Expression, T> for UnaryOpExprSymbol {
    fn parse(&self,
             parser: &mut Parser<T>, token: Token) -> ParseResult<Expression> {
        let start = token.location();
        let precedence = Precedence::for_token(token.get_type(), true);
        let right_expr = try!(parser.expression(precedence));
        let right_value = try!(right_expr.expect_value());
        let operator = try!(parser.unary_operator(token.get_type()));
        Ok(Expression::UnaryOp(UnaryOperation::new(start, operator, Box::new(right_value))))
    }
}
