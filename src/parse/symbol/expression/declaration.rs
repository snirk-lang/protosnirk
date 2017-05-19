//! Parses variable declarations

// This will become much more complex with tuple declarations
// and other pattern declaration types.

use lex::{tokens, Token, Tokenizer, TokenType, TokenData};
use parse::{Parser, ParseResult, ParseError};
use parse::ast::*;
use parse::symbol::{PrefixParser, Precedence};

///
/// # Examples
/// ```text
/// let mut            x          =         6 + 3
/// ^:.  ^:mutable  ->name:name (skip) ->value:expression
/// ```
#[derive(Debug)]
pub struct DeclarationParser { }
impl<T: Tokenizer> PrefixParser<Expression, T> for DeclarationParser {
    fn parse(&self, parser: &mut Parser<T>, token: Token) -> ParseResult<Expression> {
        debug_assert!(token.text == tokens::Let,
                      "Let parser called with non-let token {:?}", token);
        trace!("Parsing declaration for {}", token);
        let is_mutable = parser.peek().text == tokens::Mut;
        if is_mutable {
            parser.consume();
        }
        trace!("Found mutability: {}", is_mutable);
        let name = try!(parser.lvalue());
        trace!("Got name {:?}", name);
        try!(parser.consume_name(TokenType::Symbol, tokens::Equals));
        trace!("Consumed =, parsing rvalue");
        let value_expr = try!(parser.expression(Precedence::Min));
        let value = try!(value_expr.expect_value());
        trace!("Got rvalue {:?}", value);
        Ok(Expression::Declaration(Declaration::new(token, is_mutable, name, Box::new(value))))
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;
    use std::cell::RefCell;

    use lex::{Token, TokenData, TokenType, TextLocation};
    use parse::ast::{Declaration, Expression, Statement, Block, Literal, Identifier};
    use parse::symbol::{PrefixParser, DeclarationParser};
    use parse::ScopedId;
    use parse::tests as parse_tests;

    const LET_TOKEN: Token = Token {
        data: TokenData::Keyword,
        text: Cow::Borrowed("let"),
        location: TextLocation {
            column: 0, line: 0, index: 0
        }
    };

    const X_TOKEN: Token = Token {
        data: TokenData::Ident,
        text: Cow::Borrowed("x"),
        location: TextLocation {
            column: 0, line: 0, index: 0
        }
    };


    const LITERAL_ZERO: Expression = Expression::Literal(Literal {
        token: Token {
            data: TokenData::NumberLiteral(0f64),
            text: Cow::Borrowed("0"),
            location: TextLocation {
                column: 0, line: 0, index: 0
            }
        }
    });

    #[test]
    fn it_parses_let_var_eq_value() {
        let mut parser = parse_tests::parser("x = 0");
        let ident = Identifier {
            index: RefCell::new(ScopedId::default()),
            token: X_TOKEN.clone() // Not looking at token here?
        };
        let expected = Declaration::new(LET_TOKEN.clone(), false, ident, Box::new(LITERAL_ZERO.clone()));
        let parsed = DeclarationParser { }.parse(&mut parser, LET_TOKEN.clone()).unwrap();
        parse_tests::expression_match(&Expression::Declaration(expected), &parsed);
    }

    #[test]
    fn it_parses_let_mut_var_eq_value() {
        let mut parser = parse_tests::parser("mut x = 0");
        let ident = Identifier {
            index: RefCell::new(ScopedId::default()),
            token: X_TOKEN.clone() // Not looking at token here?
        };
        let expected = Declaration::new(LET_TOKEN.clone(), true, ident, Box::new(LITERAL_ZERO.clone()));
        let parsed = DeclarationParser { }.parse(&mut parser, LET_TOKEN.clone()).unwrap();
        parse_tests::expression_match(&Expression::Declaration(expected), &parsed);
    }
}
