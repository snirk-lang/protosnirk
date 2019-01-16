//! Identifier parser

// This is gonna be merged with lvalue parsers

use lex::{Token, Tokenizer};
use parse::{Parser, ParseResult};
use ast::*;
use parse::parsers::PrefixParser;

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
