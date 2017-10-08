//! Parser for `(`.

// This parser will be one of the first to be heavily
// overloaded (tuple parsing vs expression recedence in expr prefix).

use lex::{Token, Tokenizer, TokenType, TokenData};
use parse::{Parser, ParseResult, ParseError};
use parse::ast::*;
use parse::symbol::{PrefixParser, Precedence};

/// Parses expressions wrapped in parentheses
///
/// # Examples
/// ```text
/// (        x + 1          )
/// ^  ->right:expression (skip)
/// ```
#[derive(Debug)]
pub struct ParensParser { }
impl<T: Tokenizer> PrefixParser<Expression, T> for ParensParser {
    fn parse(&self, parser: &mut Parser<T>, _token: Token) -> ParseResult<Expression> {
        debug_assert!(_token.get_type() == TokenType::LeftParen,
                      "Parens parser called with non-left-paren {:?}", _token);
        let inner_expr = try!(parser.expression(Precedence::Min));
        let inner = try!(inner_expr.expect_value());
        try!(parser.consume_type(TokenType::RightParen));
        Ok(inner)
    }
}

#[cfg(test)]
mod tests {
    // TODO test basic parens, unmatching, indentation
}
