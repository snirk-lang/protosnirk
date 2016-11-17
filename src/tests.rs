//! Mid-sized integration tests for protosnirk.
//! Most testing is done in the external tests.

use std::borrow::Cow;

use super::lex::{Tokenizer, StaticStrTokenizer, Token, TokenType, TextLocation};
use super::lex::tokenizer::{self, TokenData};
use super::parse::{Parser, Precedence};
use super::parse::expression::*;

struct VecTokenizer(Vec<tokenizer::Token>);
impl Tokenizer for VecTokenizer {
    fn next(&mut self) -> tokenizer::Token {
        if self.0.is_empty() {
            tokenizer::Token {
                location: TextLocation::default(),
                data: TokenData::EOF
            }
        } else {
            self.0.remove(0)
        }
    }
}

macro_rules! match_tokens {
    ($tokenizer:ident { $($value:expr),* }) => {
        $(
            let next = $tokenizer.next();
            let expected: TokenData = $value;
            assert_eq!(next.data, expected);
        )*
    }
}

macro_rules! token_list {
    ($($token_type:ident $value:expr),*) => {
        vec![
            $(Token::new(start: 0usize, end: $value.len(), text: $value.into(), type_: TokenType::$name)),*
        ]
    }
}

fn to_tokens(data: &[TokenData]) -> Vec<tokenizer::Token> {
    data.into_iter().map(|data|
        tokenizer::Token {
            location: TextLocation::default(),
            data: data.clone()
        }).collect()
}



#[test]
fn it_grabs_single_keyword() {
    let input = "let";
    let mut tokenizer = StaticStrTokenizer::new(input);
    match_tokens!(tokenizer {
        TokenData::Keyword(Cow::Owned("let".into())),
        TokenData::EOF
    });
}

#[test]
fn it_grabs_single_ident() {
    let input = "x_2";
    let mut tokenizer = StaticStrTokenizer::new(input);
    match_tokens!(tokenizer {
        TokenData::Ident(Cow::Owned("x_2".into())),
        TokenData::EOF
    });
}
#[test]
fn it_grabs_let_ident() {
    let input = "let x";
    let mut tokenizer = StaticStrTokenizer::new(input);
    match_tokens!(tokenizer {
        TokenData::Keyword(Cow::Borrowed("let")),
        TokenData::Ident(Cow::Borrowed("x")),
        TokenData::EOF
    });
}

#[test]
fn lexer_kinda_does_a_thing_maybe() {
    let input =
    "let x = y \
     y += 55e7\t \n\
     return y % x + 224.5".into();
    let mut tokenizer = StaticStrTokenizer::new(input);
    match_tokens!(tokenizer {
        TokenData::Keyword(Cow::Owned("let".into())),
        TokenData::Ident(Cow::Owned("x".into())),
        TokenData::Symbol(Cow::Owned("=".into())),
        TokenData::Ident(Cow::Owned("y".into())),
        TokenData::Ident(Cow::Owned("y".into())),
        TokenData::Symbol(Cow::Owned("+=".into())),
        TokenData::NumberLiteral(55e7f64),
        TokenData::Keyword(Cow::Borrowed("return")),
        TokenData::Ident(Cow::Borrowed("y")),
        TokenData::Symbol(Cow::Borrowed("%")),
        TokenData::Ident(Cow::Borrowed("x")),
        TokenData::Symbol(Cow::Borrowed("+")),
        TokenData::NumberLiteral(224.5f64),
        TokenData::EOF
    });
}

//#[test]
fn parser_kinda_parses_a_thing_maybe() {
    let tokenizer = VecTokenizer(to_tokens(&[
        TokenData::Keyword(Cow::Borrowed("return")),
        TokenData::Ident(Cow::Borrowed("x")),
        TokenData::Symbol(Cow::Borrowed("+")),
        TokenData::NumberLiteral(4f64)
    ]));
    let mut parser = Parser::new(Box::new(tokenizer));
    assert_eq!(parser.expression(Precedence::Min),
        Ok(Expression::Assignment(
            Assignment::new(Identifier::new("x".into()),
                            Box::new(
                                Expression::Literal(Literal::new(4f64))
                            ))
        )));
}
