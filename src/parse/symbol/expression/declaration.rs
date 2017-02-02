//! Parses variable declarations

// This will become much more complex with tuple declarations
// and other pattern declaration types.

use lex::{tokens, Token, Tokenizer, TokenType, TokenData};
use parse::{Parser, ParseResult, ParseError, Precedence};
use parse::ast::*;
use parse::symbol::PrefixParser;

///
/// # Examples
/// ```text
/// mut            x          =         6 + 3
///  ^:mutable  ->name:name (skip) ->value:expression
/// ```
#[derive(Debug)]
pub struct DeclarationParser { }
impl<T: Tokenizer> PrefixParser<Expression, T> for DeclarationParser {
    fn parse(&self, parser: &mut Parser<T>, token: Token) -> ParseResult<Expression> {
        debug_assert!(token.text == tokens::Let,
                      "Let parser called with non-let token {:?}", token);
        trace!("Parsing declaration for {}", token);
        let is_mutable = parser.peek().text == tokens::Mut;
        if is_mutable {
            parser.consume();
        }
        trace!("Found mutability: {}", is_mutable);
        let name = try!(parser.lvalue());
        trace!("Got name {:?}", name);
        try!(parser.consume_name(TokenType::Symbol, tokens::Equals));
        trace!("Consumed =, parsing rvalue");
        // TODO allow for block here
        let value_expr = try!(parser.expression(Precedence::Min));
        let value = try!(value_expr.expect_value());
        println!("Got rvalue {:?}", value);
        Ok(Expression::Declaration(Declaration::new(name.into(), is_mutable, Box::new(value))))
    }
}
