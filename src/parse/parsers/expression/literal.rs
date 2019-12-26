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
    fn parse(&self, _parser: &mut Parser<T>, token: Token)
             -> ParseResult<Expression> {
        match token.data() {
            TokenData::NumberLiteral => {
                match token.text().parse::<f64>() {
                    Ok(val) =>
                        Ok(Expression::Literal(
                            Literal::new_float(token, val))),
                    Err(_) =>
                        // This is an internal error: tokenizer should've bailed
                        Err(ParseError::ExpectedToken {
                            expected: TokenType::Literal,
                            got: token.get_type(),
                            token: token
                        })
                }
            },
            TokenData::BoolLiteral => {
                match token.text() {
                    "true" =>
                        Ok(Expression::Literal(
                            Literal::new_bool(token, true))),
                    "false" =>
                        Ok(Expression::Literal(
                            Literal::new_bool(token, false))),
                    // This is an unexpected internal error.
                    _ => Err(ParseError::ExpectedToken {
                        expected: TokenType::Literal,
                        got: token.get_type(),
                        token: token
                    })
                }
            },
            TokenData::UnitLiteral => {
                Ok(Expression::Literal(Literal::new_unit(token)))
            },
            // This is an unexpected internal error.
            _ => Err(ParseError::ExpectedToken {
                expected: TokenType::Literal,
                got: token.get_type(),
                token: token
            })
        }
    }
}
