//! Statement parsers
//!
//! Statements are parsed in the parser's `statement()` method.
//! If a statement keyword parser is found (`let`, `if`, `do`, etc.)
//! then that statement is called, else the parser returns `Statement::Expression`
//! by calling its `expression()` method.
//! Note that assignment becomes a statement in this case.

use lex::{tokens, Token, Tokenizer, TokenData, TokenType};
use parse::{Parser, ParseResult, ParseError, Precedence};
use parse::symbol::{PrefixParser, InfixParser};
use parse::ast::{Expression, Statement, Block, Return, DoBlock};

/// Parses return statements
///
/// # Examples
/// ```text
/// return x + 1 + 3 * 4
///   ^    ->right:expression
/// ```
#[derive(Debug)]
pub struct ReturnParser { }
impl<T: Tokenizer> PrefixParser<Statement, T> for ReturnParser {
    fn parse(&self, parser: &mut Parser<T>, token: Token) -> ParseResult<Statement> {
        debug_assert!(token.text == tokens::Return,
                      "Return parser called with non-return {:?}", token);
        // If the next statement is on a newline then empty return.
        // Should also check for an indent block to ensure sprious indentation is an error.
        if parser.peek_is_newline(&token) {
            return Ok(Statement::Return(Return::new(token, None)))
        }
        let inner_expr = try!(parser.expression(Precedence::Return));
        let inner = try!(inner_expr.expect_value());
        Ok(Statement::Return(Return::new(token, Box::new(inner))))
    }
}

/// Parses a block statement using the prefix symol `do`.
///
/// # Examples
/// ```text
/// do    \+    let x = 0 stmt*
/// ^take ^take ^block
///
/// do     x += 5
/// ^take  ^expr
/// ```
/// Produces `Expression::Block`s.
#[derive(Debug)]
pub struct DoBlockParser { }
impl<T: Tokenizer> PrefixParser<Statement, T> for DoBlockParser {
    fn parse(&self, parser: &mut Parser<T>, token: Token) -> ParseResult<Statement> {
        debug_assert!(token.text == "do",
            "Invalid token {:?} in DoBlockParser", token);
        if parser.next_type() == TokenType::BeginBlock {
            parser.consume();
            let block = try!(parser.block());
            Ok(Statement::DoBlock(DoBlock::new(token, Box::new(block))))
        }
        else { // Allow for inline form `do <expr>`
            let expr = try!(parser.expression(Precedence::Min));
            let block = Block::new(vec![expr]);
            Ok(Statement::DoBlock(DoBlock::new(token, Box::new(block))))
        }
    }
}
