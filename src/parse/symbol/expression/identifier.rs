//! Identifier parser

// This is gonna be merged with lvalue parsers

use lex::{Token, Tokenizer, TokenType};
use parse::{Parser, ParseResult};
use parse::ast::*;
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
    fn parse(&self, _parser: &mut Parser<T>, token: Token) -> ParseResult<Expression> {
        Ok(Expression::VariableRef(Identifier::new(token)))
    }
}

#[cfg(test)]
mod tests {
    // TODO test 
}
