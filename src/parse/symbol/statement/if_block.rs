//! If block/inline if parser.

use lex::{tokens, Token, Tokenizer, TokenData, TokenType};
use parse::ast::*;
use parse::{Parser, ParseError, ParseResult};
use parse::symbol::{PrefixParser, Precedence};

/// Parses if blocks and inline if expressions.
///
/// # Examples
/// ```text
/// if expr \+ stmt* \- [else if expr \+ stmt* \-]* [else \+ stmt*]
/// ```
/// If the `=>` is detected signifying an inline if, the parser will
/// call out to `IfExpressionParser` and return that expression in
/// statement form.
///
/// This parser is allowed to assume it can parse an inline if expr
/// instead, but the inline if parser should assume that it is parsing
/// a context where only [inline] expressions are allowed.
#[derive(Debug)]
pub struct IfBlockParser { }
impl<T: Tokenizer> PrefixParser<Statement, T> for IfBlockParser {
    fn parse(&self, parser: &mut Parser<T>, token: Token) -> ParseResult<Statement> {
        debug_assert!(token.get_text() == "if",
            "Invalid token {:?} in IfBlockParser", token);
        let condition = try!(parser.expression(Precedence::Min));
        if parser.peek().get_text() == tokens::InlineArrow {
            let true_expr = try!(parser.expression(Precedence::Min));
            try!(parser.consume_name(TokenType::Keyword, tokens::Else));
            if parser.peek().get_text() == tokens::If {
                let error = "Cannot have an `else if` via inline if expression";
                return Err(ParseError::LazyString(error.to_string()))
            }
            let else_expr = try!(parser.expression(Precedence::Min));
            let if_expr = IfExpression::new(token,
                                            Box::new(condition),
                                            Box::new(true_expr),
                                            Box::new(else_expr));
            return Ok(Statement::Expression(Expression::IfExpression(if_expr)))
        }
        let true_block = try!(parser.block());
        let first_conditional = Conditional::new(token, condition, true_block);
        let mut conditionals = vec![first_conditional];
        loop {
            // keep parsing else ifs. Break on a lone else.
            if parser.peek().get_text() != tokens::Else {
                return Ok(Statement::IfBlock(IfBlock::new(conditionals, None)))
            }
            let else_token = parser.consume(); // else token
            if parser.peek().get_text() != tokens::If {
                // just an else here
                let else_block = try!(parser.block());
                return Ok(Statement::IfBlock(
                        IfBlock::new(conditionals, Some((else_token, else_block)))));
            }
            let if_token = parser.consume();
            let else_if_condition = try!(parser.expression(Precedence::Min));
            if parser.peek().get_text() == tokens::InlineArrow {
                let error = "Cannot have an inline `else if` via if block";
                return Err(ParseError::LazyString(error.to_string()))
            }
            let else_if_block = try!(parser.block());
            let else_if_conditional = Conditional::new(if_token,
                                                       else_if_condition,
                                                       else_if_block);
            conditionals.push(else_if_conditional);
        }
    }
}
