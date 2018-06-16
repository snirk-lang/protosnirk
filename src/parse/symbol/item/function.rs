//! Parser for function declarations

use lex::{Token, Tokenizer, TokenType, TokenData};
use ast::*;
use parse::{Parser, ParseResult, ParseError, IndentationRule};
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
        debug_assert!(token.get_type() == TokenType::Fn,
            "Unexpected token {:?} to fn parser", token);
        let name = try!(parser.lvalue());

        // Args

        // TODO Eventually params should be a separate parser?
        // altough the fn signature type parser would be a little different
        // from the first-class-fn type parser.

        // left paren cannot be indented
        try!(parser.consume_type(TokenType::LeftParen));
        // S1 -> ")", done | name, S2
        // S2 -> ",", S1 | ")", done
        let mut params = Vec::new();
        let mut param_name = true;
        loop {
            if parser.next_type() == TokenType::RightParen {
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
                try!(parser.consume_type_indented(TokenType::Comma,
                                                  IndentationRule::NegateDeindent));
                param_name = true;
            }
        }

        // Explicitly differentiating between omitted return type for block fns
        // This is gonna be `None` for inline fns
        let (return_ty, explicit) = if parser.next_type() == TokenType::Arrow {
            parser.consume();
            (try!(parser.type_expr()), true)
        }
        else {
            (TypeExpression::Named(NamedTypeExpression::new(Identifier::new(
                Token::new_ident("()",
                        name.token().location().clone())))), false)
        };

        // This is gonna require a comment in the place of Python's `pass`.
        try!(parser.consume_type(TokenType::BeginBlock));
        let block = try!(parser.block());
        Ok(Item::BlockFnDeclaration(BlockFnDeclaration::new(
            token, name, params, return_ty, explicit, block
        )))
    }
}
