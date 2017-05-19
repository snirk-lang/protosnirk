mod expression;
mod statement;
mod item;
mod types;
mod precedence;

pub use self::expression::*;
pub use self::statement::*;
pub use self::item::*;
pub use self::types::*;
pub use self::precedence::Precedence;

use std::rc::Rc;

use lex::{Token, TokenType, Tokenizer};
use parse::{Parser, ParseError, ParseResult};
use parse::ast::{Expression, UnaryOperation, BinaryOperation};

// # Note
// The generic type `T: Tokenizer` is present so parsers can be made into objects
// and selected over dynamically (for custom keywords).
// Although at this point I'm _probably_ never going to use custom operators,
// at least not at the parser level.

/// Generic parser used to parse AST nodes of type E in the prefix position.
///
pub trait PrefixParser<E, T: Tokenizer> {
    fn parse(&self, parser: &mut Parser<T>, token: Token) -> ParseResult<E>;
}

/// Generic parser trait used to parse AST nodes of type E in the infix position.
pub trait InfixParser<E, T: Tokenizer> {
    fn parse(&self, parser: &mut Parser<T>, left: E, token: Token) -> ParseResult<E>;
    fn get_precedence(&self) -> Precedence;
}

// TODO This can't be implemented until we have a way of knowing when to stop calling
// the prefix parser, i.e. once a close paren is matched.

/// Parses a list of expressions using an `InfixParser`.
#[derive(Debug, Clone, Copy)]
pub struct CommaSeparatedParser<'p, E, T: Tokenizer> {
    parser: &'p PrefixParser<E, T>,
    end_type: TokenType
}
impl<'p, E, T: Tokenizer> CommaSeparatedParser<'p, E, T> {
    pub fn new(parser: &'p PrefixParser<E, T>, end_type: TokenType)
              -> CommaSeparatedParser<'p, E, T> {
        CommaSeparatedParser { parser: parser, end_type: end_type }
    }
}

// The issue with this parser is that it results in stanard looking error
// messges like "expected one of , or )" and we can do better. This means
// equipping parsers with some metadata if they're to be called in this
// fashion.

impl<'p, E, T: Tokenizer> PrefixParser for CommaSeparatedParser<'p, E, T> {
    fn parse(&self, parser: &mut Parser<T>, token: Token) -> ParseResult<Vec<E>> {
        if parser.next_type() == self.end_type {
            return Ok(Vec::new())
        }
        let mut found = Vec::new();
        loop {
            let next_token = parser.consume();
            if next_token.get_type() == TokenType::Comma {
                trace!("Comma parser: found leading comma");
                return Err(ParseError::LazyString("Unexpected comma in comma separated list"))
            }
            let next_expr = try!(self.parser.parse(parser, next_token));
            found.push(next_expr);
            match parser.next_type() {
                _next if _next == (self.end_type) => {
                    return Ok(found)
                },
                TokenType::Comma => {
                    parser.consume()
                },
                other => {
                    trace!("Comma parser: non-comma after expression");
                    return Err(ParseError::ExpectedToken {
                        expected: self.end_type,
                        got: parser.consume()
                    })
                }
            }
        }
    }
}
