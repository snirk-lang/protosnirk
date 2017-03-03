//! Parser for function declarations

use lex::{tokens, Token, Tokenizer, TokenType, TokenData};
use parse::{Parser, ParseResult, ParseError, IndentationRule};
use parse::ast::*;
use parse::symbol::{PrefixParser, Precedence, AssignmentParser, InfixParser};

/// Parses a function declaration.
///
/// # Examples
/// ```
/// fn foo(bar, baz,
///        bliz)
///        -> int
///     stmt*
///
/// fn foo (bar, baz, \+ bliz) -> int \- \+ stmt* \-
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct FnDeclarationParser { }
impl<T: Tokenizer> PrefixParser<Item, T> for FnDeclarationParser {
    fn parse(&self, parser: &mut Parser<T>, token: Token) -> ParseResult<Item> {
        debug_assert!(token.get_text() == tokens::Fn,
            "Unexpected token {:?} to fn parser", token);
        let name = try!(parser.lvalue());

        // Args
        // left paren cannot be indented
        try!(parser.consume_name(TokenType::Symbol, tokens::LeftParen));
        // S1 -> ")", done | name, S2
        // S2 -> ",", S1 | ")", done
        let mut args = Vec::new();
        let mut arg_name = true;
        loop {
            if parser.peek().get_text() == tokens::RightParen {
                parser.consume(); // right paren
                break
            }
            // name
            if arg_name {
                parser.apply_indentation(IndentationRule::NegateDeindent);
                let name = try!(parser.lvalue());
                args.push(name);
                arg_name = false;
            }
            // comma
            else {
                try!(parser.consume_name_indented(TokenType::Symbol,
                                                  tokens::Comma,
                                                  IndentationRule::NegateDeindent));
                arg_name = true;
            }
        }
        // TODO -> result type

        // Function body
        try!(parser.consume_type(TokenType::BeginBlock));
        let block = try!(parser.block());

        let decl = FnDeclaration::new(token, name, args, block);
        Ok(Item::FnDeclaration(decl))
    }
}
