//! Literal expression parsing

use lex::{Token, Tokenizer, TokenData, TokenType};
use parse::{Parser, ParseError, ParseResult};
use parse::symbol::PrefixParser;
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
        match *token.get_data() {
            TokenData::NumberLiteral(val) =>
                Ok(Expression::Literal(Literal::new(token, LiteralValue::Float(val)))),
            _ => Err(ParseError::ExpectedToken {
                    expected: TokenType::Literal,
                    got: token
                })
        }
    }
}

#[cfg(test)]
mod tests {
    use lex::{Token, TokenData, TokenType};
    use parse::ast::{Expression, Literal};
    use parse::symbol::{PrefixParser, LiteralParser};
    use parse::tests as parse_tests;

    #[cfg(test)]
    fn it_parses_literal_number() {
        let mut parser = parse_tests::parser("5");
        let expected_token = Token {
            data: TokenData::NumberLiteral(5f64),
            .. Default::default()
        };
        let expected = Expression::Literal(Literal::new(expected_token.clone()));
        let parsed = LiteralParser { }.parse(&mut parser, expected_token).unwrap();
        parse_tests::expression_match(&expected, &parsed);
    }
}
