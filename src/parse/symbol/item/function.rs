//! Parser for function declarations

use lex::{tokens, Token, Tokenizer, TokenType, TokenData};
use parse::{Parser, ParseResult, ParseError, IndentationRule};
use parse::ast::*;
use parse::symbol::{PrefixParser, Precedence};

/// Parses a function declaration.
///
/// # Examples
/// ```
/// fn foo(bar, baz,
///        bliz)
///        -> int
///     stmt*
///
/// fn foo (bar, baz, \+ bliz) -> int \- \+
/// ```

#[derive(Debug, PartialEq, Clone)]
pub struct FnDeclarationParser { }
impl<T: Tokenizer> PrefixParser<Item, T> for FnDeclarationParser {
    fn parse(&self, parser: &mut Parser<T>, token: Token) -> ParseResult<Item> {
        debug_assert!(token.get_text() == tokens::Fn,
            "Unexpected token {:?} to fn parser", token);
        let _name = try!(parser.consume_type(TokenType::Ident));
        /*let open_paren = try!(parser.consume_name_indented(TokenType::Symbol,
                                                           tokens::LeftParen,
                                                           IndentationRule::IgnoreIndent));*/
        unimplemented!()
    }
}
