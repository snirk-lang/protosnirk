//! Literal expression parsing

use lex::{Token, Tokenizer, TokenData, TokenType};
use parse::{Parser, ParseError, ParseResult};
use parse::parsers::PrefixParser;
use ast::*;

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
        match *token.data() {
            TokenData::NumberLiteral(val) =>
                Ok(Expression::Literal(Literal::new(token, LiteralValue::Float(val)))),
            TokenData::BoolLiteral(b) =>
                Ok(Expression::Literal(Literal::new(token, LiteralValue::Bool(b)))),
            _ => Err(ParseError::ExpectedToken {
                expected: TokenType::Literal,
                got: token.get_type(),
                token: token
            })
        }
    }
}
