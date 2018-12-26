//! Identifier parser

// This is gonna be merged with lvalue parsers

use lex::{Token, TokenData, Tokenizer};
use parse::{Parser, ParseResult};
use ast::*;
use parse::symbol::PrefixParser;

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
    fn parse(&self, _parser: &mut Parser<T>, mut token: Token) -> ParseResult<Expression> {
        if token.text() == "true" {
            token.data = TokenData::BoolLiteral(true);
            Ok(Expression::Literal(Literal::new(token, LiteralValue::Bool(true))))
        }
        else if token.text() == "false" {
            token.data = TokenData::BoolLiteral(false);
            Ok(Expression::Literal(Literal::new(token, LiteralValue::Bool(false))))
        }
        else {
            Ok(Expression::VariableRef(Identifier::new(token)))
        }
    }
}
