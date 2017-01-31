//! Expression parsers
//!
//! Expression parsers are defined from TDOP.

use lex::{tokens, Token, Tokenizer, TokenType, TokenData};
use parse::{Parser, Precedence, ParseResult, ParseError};
use parse::symbol::{PrefixParser, InfixParser};
use parse::ast::*;


/// Returns a literal expression
///
/// # Examples
/// ```text
/// 34
/// ^literal
/// ```
pub struct LiteralParser { }
impl<T: Tokenizer> PrefixParser<Expression, T> for LiteralParser {
    fn parse(&self, _parser: &mut Parser<T>, token: Token) -> ParseResult<Expression> {
        match token.data {
            TokenData::NumberLiteral(_) =>
                Ok(Expression::Literal(Literal::new(token))),
            _ => Err(ParseError::ExpectedToken {
                    expected: TokenType::Literal,
                    got: token
                })
        }
    }
}

/// Returns an identifier
///
/// # Examples
/// ```text
/// x
/// ^:name
/// ```
#[derive(Debug)]
pub struct IdentifierParser { }
impl<T: Tokenizer> PrefixParser<Expression, T> for IdentifierParser {
    fn parse(&self, _parser: &mut Parser<T>, token: Token) -> ParseResult<Expression> {
        Ok(Expression::VariableRef(Identifier::new(token)))
    }
}


/// Parses expressions wrapped in parentheses
///
/// # Examples
/// ```text
/// (        x + 1          )
/// ^  ->right:expression (skip)
/// ```
#[derive(Debug)]
pub struct ParensParser { }
impl<T: Tokenizer> PrefixParser<Expression, T> for ParensParser {
    fn parse(&self, parser: &mut Parser<T>, _token: Token) -> ParseResult<Expression> {
        debug_assert!(_token.text == tokens::LeftParen,
                      "Parens parser called with non-left-paren {:?}", _token);
        let inner_expr = try!(parser.expression(Precedence::Min));
        let inner = try!(inner_expr.expect_value());
        try!(parser.consume_name(TokenType::Symbol, tokens::RightParen));
        Ok(inner)
    }
}

/// Parses a declaration/// Parses an assignment expresion.
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
        debug_assert!(_token.text == tokens::Equals,
                      "Assign parser called with non-assign token {:?}", _token);
        let ident = try!(left.expect_identifier());
        let right_expr = try!(parser.expression(Precedence::Assign));
        let right = try!(right_expr.expect_value());
        Ok(Expression::Assignment(Assignment::new(ident, Box::new(right))))
    }
    fn get_precedence(&self) -> Precedence {
        Precedence::Assign
    }
}

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
        let operator = try!(parser.operator(token.data.get_type(), &token.text));
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

///
/// # Examples
/// ```text
/// mut            x          =         6 + 3
///  ^:mutable  ->name:name (skip) ->value:expression
/// ```
#[derive(Debug)]
pub struct DeclarationParser { }
impl<T: Tokenizer> PrefixParser<Expression, T> for DeclarationParser {
    fn parse(&self, parser: &mut Parser<T>, token: Token) -> ParseResult<Expression> {
        debug_assert!(token.text == tokens::Let,
                      "Let parser called with non-let token {:?}", token);
        trace!("Parsing declaration for {}", token);
        let is_mutable = parser.peek().text == tokens::Mut;
        if is_mutable {
            parser.consume();
        }
        trace!("Found mutability: {}", is_mutable);
        let name = try!(parser.lvalue());
        trace!("Got name {:?}", name);
        try!(parser.consume_name(TokenType::Symbol, tokens::Equals));
        trace!("Consumed =, parsing rvalue");
        // TODO allow for block here
        let value_expr = try!(parser.expression(Precedence::Min));
        let value = try!(value_expr.expect_value());
        println!("Got rvalue {:?}", value);
        Ok(Expression::Declaration(Declaration::new(name.into(), is_mutable, Box::new(value))))
    }
}
