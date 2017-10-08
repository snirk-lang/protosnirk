//! Assign operator parser.

use lex::{Token, Tokenizer, TokenType, TokenData};
use parse::ast::*;
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

#[cfg(test)]
mod tests {
    use std::borrow::Cow;
    use lex::{Token, TokenData, TokenType};
    use parse::ast::{Expression, Literal, Identifier};
    use parse::symbol::{InfixParser, AssignOpParser};
    use parse::tests as parse_tests;

    // TODO test assign ident, assign non-ident
    fn it_parses_lvalue_eq_expr() {
        let mut parser = parse_tests::parser("5");
        let lvalue = Expression::VariableRef(Identifier::new(Token {
            data: TokenData::Ident,
            text: Cow::Borrowed("x"),
            .. Default::default()
        }));
        let assign_token = Token {
            data: TokenData::Symbol,
            text: Cow::Borrowed("+="),
            .. Default::default()
        };
        let _expr = AssignOpParser { }.parse(&mut parser, lvalue, assign_token);
    }

    #[test]
    fn it_fails_lvalue_eq_block() {
        let mut parser = parse_tests::parser("do\n    return x");
        let lvalue = Expression::Literal(Literal::new(Token {
            data: TokenData::NumberLiteral(5f64),
            .. Default::default()
        }));
        let assign_token = Token {
            data: TokenData::Symbol,
            text: Cow::Borrowed("*="),
            .. Default::default()
        };
        let expr = AssignOpParser { }.parse(&mut parser, lvalue, assign_token);
        assert!(expr.is_err());

    }

    #[test]
    fn it_fails_for_bad_lvalue() {
        let mut parser = parse_tests::parser("5");
        let lvalue = Expression::Literal(Literal::new(Token {
            data: TokenData::NumberLiteral(5f64),
            .. Default::default()
        }));
        let assign_token = Token {
            data: TokenData::Symbol,
            text: Cow::Borrowed("/="),
            .. Default::default()
        };
        let expr = AssignOpParser { }.parse(&mut parser, lvalue, assign_token);
        assert!(expr.is_err());
    }
}
