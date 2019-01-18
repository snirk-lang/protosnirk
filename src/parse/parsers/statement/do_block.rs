//! Block literal `do` statement.

use lex::{Token, Tokenizer, TokenType};
use ast::*;
use parse::{Parser, ParseResult};
use parse::parsers::PrefixParser;

/// Parses a block statement using the prefix symol `do`.
///
/// # Examples
/// ```text
/// do    \+    stmt*
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
        debug_assert!(token.text() == "do",
                      "Invalid token {:?} in DoBlockParser", token);
        let start = token.start();
        if parser.next_type() == TokenType::BeginBlock {
            parser.consume();
            let block = try!(parser.block());
            Ok(Statement::DoBlock(DoBlock::new(start, Box::new(block))))
        }
        else { // Allow for inline form `do <expr>`
            // Parsing a statement here may be useless
            // We might want only expressions.
            // Also allows for do do do do x
            let stmt = try!(parser.statement());
            let block = Block::new(stmt.span().start(), vec![stmt]);
            Ok(Statement::DoBlock(DoBlock::new(start, Box::new(block))))
        }
    }
}

#[cfg(test)]
mod tests {
    // TODO test
    // - Do with multistatement expr
    // - Do with 0 exprs but indentation (move to general indent tests?)
}
