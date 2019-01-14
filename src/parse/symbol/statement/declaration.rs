//! Parses variable declarations

// This will become more complex with tuple declarations
// and other pattern declaration types.

use lex::{Token, Tokenizer, TokenType};
use ast::*;
use parse::{Parser, ParseResult};
use parse::symbol::{PrefixParser, Precedence};

///
/// # Examples
/// ```text
/// let mut            x        :      type?   =         6 + 3
/// ^:.  ^:mutable  ->name:name ^check ^opt   (skip) ->value:expression
/// ```
#[derive(Debug)]
pub struct DeclarationParser { }
impl<T: Tokenizer> PrefixParser<Expression, T> for DeclarationParser {
    fn parse(&self, parser: &mut Parser<T>, token: Token) -> ParseResult<Expression> {
        debug_assert!(token.get_type() == TokenType::Let,
                      "Let parser called with non-let token {:?}", token);
        trace!("Parsing declaration for {}", token);
        let is_mutable = parser.next_type() == TokenType::Mut;
        if is_mutable {
            parser.consume();
        }
        trace!("Found mutability: {}", is_mutable);
        let name = try!(parser.lvalue());
        trace!("Got name {:?}", name);
        let decl_type = if parser.next_type() == TokenType::Colon {
            trace!("Found type declaration");
            parser.consume();
            Some(try!(parser.type_expr()))
        }
        else {
            trace!("No type declaration");
            None
        };
        try!(parser.consume_type(TokenType::Equals));
        trace!("Consumed =, parsing rvalue");
        let value_expr = try!(parser.expression(Precedence::Min));
        let value = try!(value_expr.expect_value());
        trace!("Got rvalue {:?}", value);
        Ok(Expression::Declaration(Declaration::new(
            name, is_mutable, decl_type, Box::new(value)
        )))
    }
}
