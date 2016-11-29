//! Mid-sized integration tests for protosnirk.
//! Most testing is done in the external tests.

use std::borrow::Cow;
use std::str::Chars;

use super::lex::{Token, TokenType, TokenData, TextLocation, Tokenizer, IterTokenizer};
use super::parse::{Parser, Precedence};
use super::parse::expression::*;

struct VecTokenizer(Vec<Token>);
impl Tokenizer for VecTokenizer {
    fn next(&mut self) -> Token {
        if self.0.is_empty() {
            Token {
                location: TextLocation::default(),
                text: Cow::Borrowed(""),
                data: TokenData::EOF
            }
        } else {
            self.0.remove(0)
        }
    }
}

macro_rules! match_tokens {
    ($tokenizer:ident { $($token:expr),* }) => {
        $(
            let next = $tokenizer.next();
            let expected = $token;
            assert!(next == expected,
                "\nExpected: {:#?}\nActual: {:#?}", expected, next);
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

fn make_tokenizer<'a>(input: &'a str) -> IterTokenizer<Chars<'a>> {
    IterTokenizer::new(input.chars())
}

#[test]
fn it_grabs_single_keyword() {
    let input = "let";
    let mut tokenizer = make_tokenizer(input);
    match_tokens!(tokenizer {
        Token {
            data: TokenData::Keyword,
            text: Cow::Borrowed("let"),
            location: TextLocation {
                start_char: 0,
                start_line: 0,
                start_column: 0
            }
        },
        Token {
            data: TokenData::EOF,
            text: Cow::Borrowed(""),
            location: TextLocation {
                start_char: 3,
                start_line: 0,
                start_column: 3
            }
        }
    });
}

#[test]
fn it_grabs_single_ident() {
    let input = "x_2";
    let mut tokenizer = make_tokenizer(input);
    match_tokens!(tokenizer {
        Token {
            data: TokenData::Ident,
            text: Cow::Borrowed("x_2"),
            location: TextLocation::default()
        },
        Token {
            data: TokenData::EOF,
            text: Cow::Borrowed(""),
            location: TextLocation {
                start_char: 3,
                start_line: 0,
                start_column: 3
            }
        }
    });
}
#[test]
fn it_grabs_let_ident() {
    let input = "let x";
    let mut tokenizer = make_tokenizer(input);
    match_tokens!(tokenizer {
        Token {
            data: TokenData::Keyword,
            text: Cow::Borrowed("let"),
            location: TextLocation {
                start_char: 0,
                start_line: 0,
                start_column: 0
            }
        },
        Token {
            data: TokenData::Ident,
            text: Cow::Borrowed("x"),
            location: TextLocation {
                start_char: 4,
                start_line: 0,
                start_column: 4
            }
        },
        Token {
            data: TokenData::EOF,
            text: Cow::Borrowed(""),
            location: TextLocation {
                start_char: 5,
                start_line: 0,
                start_column: 5
            }
        }
    });
}

#[test]
fn lexer_kinda_does_a_thing_maybe() {
    let input =
    "let x = y \
     y += 55e7\t \n\
     return y % x + 224.5".into();
    let mut tokenizer = make_tokenizer(input);
    match_tokens!(tokenizer {
        Token {
            data: TokenData::Keyword,
            text: Cow::Borrowed("let"),
            location: TextLocation {
                start_char: 0,
                start_line: 0,
                start_column: 0
            }
        },
        Token {
            data: TokenData::Ident,
            text: Cow::Borrowed("x"),
            location: TextLocation {
                start_char: 4,
                start_line: 0,
                start_column: 4
            }
        },
        Token {
            data: TokenData::Symbol,
            text: Cow::Borrowed("="),
            location: TextLocation {
                start_char: 6,
                start_line: 0,
                start_column: 6
            }
        },
        Token {
            data: TokenData::Ident,
            text: Cow::Borrowed("y"),
            location: TextLocation {
                start_char: 8,
                start_line: 0,
                start_column: 8
            }
        },
        Token {
            data: TokenData::Ident,
            text: Cow::Borrowed("y"),
            location: TextLocation {
                start_char: 10,
                start_line: 0,
                start_column: 10
            }
        },
        Token {
            data: TokenData::Symbol,
            text: Cow::Borrowed("+="),
            location: TextLocation {
                start_char: 12,
                start_line: 0,
                start_column: 12
            }
        },
        Token {
            data: TokenData::NumberLiteral(55e7f64),
            text: Cow::Borrowed("55e7"),
            location: TextLocation {
                start_char: 15,
                start_line: 0,
                start_column: 15
            }
        },
        Token {
            data: TokenData::Keyword,
            text: Cow::Borrowed("return"),
            location: TextLocation {
                start_char: 22,
                start_line: 1,
                start_column: 0
            }
        },
        Token {
            data: TokenData::Ident,
            text: Cow::Borrowed("y"),
            location: TextLocation {
                start_char: 29,
                start_line: 1,
                start_column: 7
            }
        },
        Token {
            data: TokenData::Symbol,
            text: Cow::Borrowed("%"),
            location: TextLocation {
                start_char: 31,
                start_line: 1,
                start_column: 9
            }
        },
        Token {
            data: TokenData::Ident,
            text: Cow::Borrowed("x"),
            location: TextLocation {
                start_char: 33,
                start_line: 1,
                start_column: 11
            }
        },
        Token {
            data: TokenData::Symbol,
            text: Cow::Borrowed("+"),
            location: TextLocation {
                start_char: 35,
                start_line: 1,
                start_column: 13
            }
        },
        Token {
            data: TokenData::NumberLiteral(224.5f64),
            text: Cow::Borrowed("224.5"),
            location: TextLocation {
                start_char: 37,
                start_line: 1,
                start_column: 15
            }
        },
        Token {
            data: TokenData::EOF,
            text: Cow::Borrowed(""),
            location: TextLocation {
                start_char: 42,
                start_line: 1,
                start_column: 20
            }
        }
    });
}
/*
#[test]
fn it_ignores_line_comment() {
    let input =
    "//comment\nlet x";
    let mut tokenizer = StaticStrTokenizer::new(input);
    match_tokens!(tokenizer {
        TokenData::Keyword(Cow::Borrowed("let")),
        TokenData::Ident(Cow::Borrowed("x"))
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
}*/
