//! Parser for type alias declarations

use lex::{Token, Tokenizer, TokenType, TokenData};
use ast::*;
use parse::{Parser, ParseResult, ParseError};
use parse::symbol::{PrefixParser, Precedence};

/// Parses type alias declarations.
///
/// # Examples
/// ```txt
/// typedef Foo    =     float
/// ^take   ^ident ^take ^type_expr
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct TypeAliasDeclarationParser { }
impl<T: Tokenizer> PrefixParser<Item, T> for TypeAliasDeclarationParser {
    fn parse(&self, parser: &mut Parser<T>, token: Token) -> ParseResult<Item> {
        debug_assert!(token.get_type() == TokenType::Typedef,
            "Unexpected token {:?} to type alias parser", token);

        let name = try!(parser.lvalue());

        try!(parser.consume_type(TokenType::Equals));

        let type_ = try!(parser.type_expr());

        Ok(Item::TypeAliasDeclaration(TypeAliasDeclaration::new(
            token, name, type_
        )))
    }
}
