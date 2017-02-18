//! If expression parser.

use lex::{tokens, Token, Tokenizer, TokenType, TokenData};
use parse::ast::*;
use parse::{Parser, ParseError, ParseResult};
use parse::symbol::{PrefixParser, Precedence};

/// Parses block and inline forms of prefix expr/block `if`.
///
/// # Examples
/// Inline if expression:
/// ```text
/// if expr => expr else expr
/// ```
///
/// This parser may have been called from an `IfBlockParser`
/// in order to parse the inline if when it was in the expression form.
/// However, block if form is not allowed in all expression places.
#[derive(Debug)]
pub struct IfExpressionParser { }
impl<T: Tokenizer> PrefixParser<Expression, T> for IfExpressionParser {
    fn parse(&self, parser: &mut Parser<T>, token: Token) -> ParseResult<Expression> {
        debug_assert!(token.text == "if",
            "Invlaid token {:?} in IfExpressionParser", token);
        let condition = try!(parser.expression(Precedence::Min));
        if parser.peek().get_text() == tokens::InlineArrow {
            // Inline if:
            // no else ifs
            // else required
            parser.consume();
        }
    }
}
