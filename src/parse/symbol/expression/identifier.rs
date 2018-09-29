//! Identifier parser

// This is gonna be merged with lvalue parsers

use lex::{Token, Tokenizer};
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
    fn parse(&self, _parser: &mut Parser<T>, token: Token) -> ParseResult<Expression> {
        Ok(Expression::VariableRef(Identifier::new(token)))
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;
    use lex::{Token, TokenData, TextLocation};
    use ast::{Expression, Identifier};
    use parse::symbol::{PrefixParser, IdentifierParser};
    use parse::tests as parse_tests;

    const IDENT_TOKEN: Token = Token {
        data: TokenData::Ident,
        text: Cow::Borrowed("x"),
        location: TextLocation {
            line: 0, column: 0, index: 0
        }
    };

    #[test]
    fn it_parses_identifier() {
        let mut parser = parse_tests::eof_parser();
        let expected = Expression::VariableRef(Identifier::new(IDENT_TOKEN.clone()));
        let parsed = IdentifierParser { }.parse(&mut parser, IDENT_TOKEN.clone()).unwrap();
        parse_tests::expression_match(&parsed, &expected);
    }
}
