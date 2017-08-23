//! Parser for function declarations

use lex::{tokens, Token, Tokenizer, TokenType, TokenData};
use parse::{Parser, ParseResult, ParseError, IndentationRule};
use parse::ast::*;
use parse::symbol::{PrefixParser, Precedence, AssignmentParser, InfixParser};

/// Parses a function declaration.
///
/// # Examples
/// ```txt
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

        // TODO Eventually params should be a separate parser?
        // altough the fn signature type parser would be a little different
        // from the first-class-fn type parser.

        // left paren cannot be indented
        try!(parser.consume_name(TokenType::Symbol, tokens::LeftParen));
        // S1 -> ")", done | name, S2
        // S2 -> ",", S1 | ")", done
        let mut params = Vec::new();
        let mut param_name = true;
        loop {
            if parser.peek().get_text() == tokens::RightParen {
                parser.consume(); // right paren
                break
            }
            // name
            if param_name {
                parser.apply_indentation(IndentationRule::NegateDeindent);
                let name = try!(parser.lvalue());
                try!(parser.consume_type(TokenType::Colon));
                let type_ = try!(parser.type_expr());
                params.push((name, type_));
                param_name = false;
            }
            // comma
            else {
                try!(parser.consume_name_indented(TokenType::Symbol,
                                                  tokens::Comma,
                                                  IndentationRule::NegateDeindent));
                param_name = true;
            }
        }

        // Explicitly differentiating between omitted return type for block fns
        // This is gonna be `None` for inline fns
        let return_type = if parser.next_type() == TokenType::Arrow {
            parser.consume();
            Some(try!(parser.type_expr()))
        }
        else {
            None
        };

        // This is gonna require a comment in the place of Python's `pass`.
        try!(parser.consume_type(TokenType::BeginBlock));
        let block = try!(parser.block());
        let fn_type = FnTypeExpression::new(params, return_type);
        Ok(Item::FnDeclaration(BlockFnDeclaration::new(
            token, name, fn_type, block
        )))
    }
}
