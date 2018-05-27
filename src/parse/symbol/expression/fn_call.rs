//! Function call - inline `(`

use lex::{Token, Tokenizer, TokenType, TokenData};
use ast::*;
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

        let mut call_args = Vec::new();
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
                    if parser.next_type() == TokenType::Colon {
                        trace!("Argument {} is a named arg", ident.get_name());
                        parser.consume();
                        let arg_value = try!(parser.expression(Precedence::Min));
                        call_args.push(CallArgument::named(ident, arg_value));
                    }
                    else {
                        call_args.push(CallArgument::implicit(
                            Expression::VariableRef(ident)));
                    }
                }
                else {
                    try!(parser.consume_type_indented(TokenType::RightParen,
                                                      IndentationRule::NegateDeindent));
                    trace!("Function call complete");
                    break
                }
                arg_name = false;
            }
            else {
                try!(parser.consume_type_indented(TokenType::Comma,
                                                  IndentationRule::NegateDeindent));
                arg_name = true;
            }
        }
        let call = FnCall::new(lvalue, token, call_args);
        Ok(Expression::FnCall(call))
    }

    fn get_precedence(&self) -> Precedence {
        Precedence::Paren
    }
}
