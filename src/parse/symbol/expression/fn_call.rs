//! Function call - inline `(`

use lex::{Token, Tokenizer, TokenType, TokenData};
use parse::ast::*;
use parse::{Parser, ParseResult, ParseError, IndentationRule};
use parse::symbol::{InfixParser, Precedence};

/// Parses function calls by handling `(` as in infix operator.
///
/// # Examples
/// ```text
/// foo(bar    :     otherFnCall(),     baz    )
///    >^ident ^take ^expr        ^take ^ident ^take
/// ```
#[derive(Debug)]
pub struct FnCallParser { }
impl<T: Tokenizer> InfixParser<Expression, T> for FnCallParser {
    fn parse(&self, parser: &mut Parser<T>,
             left: Expression, token: Token) -> ParseResult<Expression> {
        trace!("Parsing a function call of {:?}", left);
        debug_assert!(token.get_type() == TokenType::LeftParen,
            "FnCallParser: called on token {:?}", token);

        let lvalue = try!(left.expect_identifier());

        let mut called_args = Vec::new();
        let mut arg_name = true;
        loop {
            if parser.next_type() == TokenType::RightParen {
                parser.consume();
                trace!("Function call complete");
                break
            }
            if arg_name {
                trace!("Parsing an argument");
                let arg = try!(parser.expression(Precedence::Min));
                if let Expression::VariableRef(ident) = arg {
                    trace!("Argument {} is probably named", ident.get_name());
                    if parser.next_type() == TokenType::Colon {
                        trace!("Argument {} is a named arg", ident.get_name());
                        parser.consume();
                        let arg_value = try!(parser.expression(Precedence::Min));
                        called_args.push(CallArgument::named(ident, arg_value));
                    }
                    else {
                        trace!("Adding inferred `{0} = {0}`", ident.get_name());
                        called_args.push(CallArgument::implicit_name(ident));
                    }
                }
                // TODO need to give better errors/handle multiple exprs
                // being written
                else {
                    try!(parser.consume_type_indented(TokenType::RightParen,
                                                      IndentationRule::NegateDeindent));
                    let fn_call = FnCall::single_expr(lvalue, token, arg);
                    return Ok(Expression::FnCall(fn_call))
                }
                arg_name = false;
            }
            else {
                try!(parser.consume_type_indented(TokenType::Comma,
                                                  IndentationRule::NegateDeindent));
                arg_name = true;
            }
        }
        let call = FnCall::named(lvalue, token, called_args);
        Ok(Expression::FnCall(call))
    }

    fn get_precedence(&self) -> Precedence {
        Precedence::Paren
    }
}
