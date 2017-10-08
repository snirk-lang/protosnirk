//! If block/inline if parser.

use lex::{Token, Tokenizer, TokenData, TokenType};
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
        debug_assert!(token.get_type() == TokenType::If,
            "Invalid token {:?} in IfBlockParser", token);
        trace!("Parsing conditional of if statement");
        let condition = try!(parser.expression(Precedence::Min));
        trace!("Parsed conditional");
        if parser.peek().get_type() == TokenType::InlineArrow {
            trace!("Next char is =>, doing infix expr");
            parser.consume();
            let true_expr = try!(parser.expression(Precedence::Min));
            trace!("Parsed infix if true expr");
            try!(parser.consume_type(TokenType::Else));
            if parser.next_type() == TokenType::If {
                let error = "Cannot have an `else if` via inline if expression";
                return Err(ParseError::LazyString(error.to_string()))
            }
            let else_expr = try!(parser.expression(Precedence::Min));
            trace!("Parsed infix if false expr");
            let if_expr = IfExpression::new(token,
                                            Box::new(condition),
                                            Box::new(true_expr),
                                            Box::new(else_expr));
            return Ok(Statement::Expression(Expression::IfExpression(if_expr)))
        }
        trace!("Parsing if block");
        try!(parser.consume_type(TokenType::BeginBlock));
        let true_block = try!(parser.block());
        let first_conditional = Conditional::new(token, condition, true_block);
        let mut conditionals = vec![first_conditional];
        loop {
            // keep parsing else ifs. Break on a lone else.
            // If there isn't an `else` after the if, it's done
            if parser.next_type() != TokenType::Else {
                return Ok(Statement::IfBlock(IfBlock::new(conditionals, None)))
            }
            let else_token = parser.consume(); // else token
            trace!("Got an else token {:?}", else_token);
            // we have else \+ ... so we have an else block
            if parser.next_type() == TokenType::BeginBlock {
                trace!("Found an empty else, parsing else block");
                parser.consume();
                let else_block = try!(parser.block());
                return Ok(Statement::IfBlock(
                    IfBlock::new(conditionals, Some((else_token, else_block)))
                ))
            }
            // we have else if ... so we have an else if expr
            else if parser.next_type() == TokenType::If {
                let if_token = parser.consume();
                let else_if_condition = try!(parser.expression(Precedence::Min));
                if parser.next_type() == TokenType::InlineArrow {
                    let error = "Cannot have an inline `else if` via if block";
                    return Err(ParseError::LazyString(error.to_string()))
                }
                // Peel off begin block of else if
                try!(parser.consume_type(TokenType::BeginBlock));
                let else_if_block = try!(parser.block());
                let else_if_conditional = Conditional::new(if_token,
                                                           else_if_condition,
                                                           else_if_block);
                conditionals.push(else_if_conditional);
            }
            else {
                return Err(ParseError::LazyString(format!(
                    "Got unexpected token {:?} after an else", parser.peek()
                )));
            }
        }
    }
}
