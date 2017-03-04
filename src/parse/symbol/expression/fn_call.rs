//! Function call - inline `(`

use lex::{Token, Tokenizer, TokenType, TokenData};
use parse::ast::*;
use parse::{Parser, ParseResult, ParseError};
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
        debug_assert!(token.get_text() == tokens::LeftParen,
            "FnCallParser: called on token {:?}", token);

        let lvalue = try!(left.expect_identifier());

        let mut called_args = Vec::new();
        let mut arg_name = true;
        loop {
            if parser.peek().get_text() == tokens::RightParen {
                parser.consume();
                break
            }
            if arg_name {
                parser.apply_indentation(IndentationRule::NegateDeindent);
                let name = try!(parser.lvalue());
                if parser.peek().get_text() == tokens::Colon {
                    parser.consume();
                    let value = parser.expression(Precedence::Min);
                    called_args.push(CallArgument::var_value(name, value));
                }
                else {
                    called_args.push(CallArgument::var_name(name));
                }
                arg_name = false;
            }
            else {
                try!(parser.consume_name_indented(TokenType::Symbol,
                                                  tokens::Comma,
                                                  IndentationRule::NegateDeindent));
                arg_name = true;
            }
        }
        let call = FnCall::new(lvalue, token, called_args);
        Ok(Expression::FnCall(call));
    }
}
