//! Mid-sized integration tests for protosnirk.
//! Most testing is done in the external tests.

use std::borrow::Cow;
use std::str::Chars;

use lex::{Token, TokenType, TokenData, TextLocation, Tokenizer, IterTokenizer};

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
                index: 0,
                line: 0,
                column: 0
            }
        },
        Token {
            data: TokenData::EOF,
            text: Cow::Borrowed(""),
            location: TextLocation {
                index: 3,
                line: 0,
                column: 3
            }
        }
    });
    let input = "mut";
    let mut tokenizer = make_tokenizer(input);
    match_tokens!(tokenizer {
        Token {
            data: TokenData::Keyword,
            text: Cow::Borrowed("mut"),
            location: TextLocation {
                index: 0,
                line: 0,
                column: 0
            }
        },
        Token {
            data: TokenData::EOF,
            text: Cow::Borrowed(""),
            location: TextLocation {
                index: 3,
                line: 0,
                column: 3
            }
        }
    });
    let input = "return";
    let mut tokenizer = make_tokenizer(input);
    match_tokens!(tokenizer {
        Token {
            data: TokenData::Keyword,
            text: Cow::Borrowed("return"),
            location: TextLocation {
                index: 0,
                line: 0,
                column: 0
            }
        },
        Token {
            data: TokenData::EOF,
            text: Cow::Borrowed(""),
            location: TextLocation {
                index: 6,
                line: 0,
                column: 6
            }
        }
    });
}

#[test]
fn it_grabs_prefix_symbol_at_end_of_file() {
    let input = "+";
    let mut tokenizer = make_tokenizer(input);
    match_tokens!(tokenizer {
        Token {
            data: TokenData::Symbol,
            text: Cow::Borrowed("+"),
            location: TextLocation::default()
        },
        Token {
            data: TokenData::EOF,
            text: Cow::Borrowed(""),
            location: TextLocation {
                index: 1,
                line: 0,
                column: 1
            }
        }
    });
}
#[test]
fn it_grabs_adjacent_prefix_symbols() {
    let input = "+-";
    let mut tokenizer = make_tokenizer(input);
    match_tokens!(tokenizer {
        Token {
            data: TokenData::Symbol,
            text: Cow::Borrowed("+"),
            location: TextLocation::default()
        },
        Token {
            data: TokenData::Symbol,
            text: Cow::Borrowed("-"),
            location: TextLocation {
                index: 1,
                line: 0,
                column: 1
            }
        },
        Token {
            data: TokenData::EOF,
            text: Cow::Borrowed(""),
            location: TextLocation {
                index: 2,
                line: 0,
                column: 2
            }
        }
    });
}

#[test]
fn it_grabs_prefix_symbol_mid_file() {
    let input = "+ ";
    let mut tokenizer = make_tokenizer(input);
    match_tokens!(tokenizer {
        Token {
            data: TokenData::Symbol,
            text: Cow::Borrowed("+"),
            location: TextLocation::default()
        },
        Token {
            data: TokenData::EOF,
            text: Cow::Borrowed(""),
            location: TextLocation {
                index: 2,
                line: 0,
                column: 2
            }
        }
    });
}


#[test]
fn it_gabs_unmatching_parens() {
    let input = "((";
    let mut tokenizer = make_tokenizer(input);
    match_tokens!(tokenizer {
        Token {
            data: TokenData::Symbol,
            text: Cow::Borrowed("("),
            location: TextLocation::default()
        },
        Token {
            data: TokenData::Symbol,
            text: Cow::Borrowed("("),
            location: TextLocation {
                index: 1,
                line: 0,
                column: 1
            }
        },
        Token {
            data: TokenData::EOF,
            text: Cow::Borrowed(""),
            location: TextLocation {
                index: 2,
                line: 0,
                column: 2
            }
        }
    });
}

#[test]
fn it_grabs_matching_parens() {
    let input = "()";
    let mut tokenizer = make_tokenizer(input);
    match_tokens!(tokenizer {
        Token {
            data: TokenData::Symbol,
            text: Cow::Borrowed("("),
            location: TextLocation::default()
        },
        Token {
            data: TokenData::Symbol,
            text: Cow::Borrowed(")"),
            location: TextLocation {
                index: 1,
                line: 0,
                column: 1
            }
        },
        Token {
            data: TokenData::EOF,
            text: Cow::Borrowed(""),
            location: TextLocation {
                index: 2,
                line: 0,
                column: 2
            }
        }
    });
}

#[test]
fn it_grabs_single_ident() {
    let input = "anIdentifier_2";
    let mut tokenizer = make_tokenizer(input);
    match_tokens!(tokenizer {
        Token {
            data: TokenData::Ident,
            text: Cow::Borrowed("anIdentifier_2"),
            location: TextLocation::default()
        },
        Token {
            data: TokenData::EOF,
            text: Cow::Borrowed(""),
            location: TextLocation {
                index: 14,
                line: 0,
                column: 14
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
            location: TextLocation::default()
        },
        Token {
            data: TokenData::Ident,
            text: Cow::Borrowed("x"),
            location: TextLocation {
                index: 4,
                line: 0,
                column: 4
            }
        },
        Token {
            data: TokenData::EOF,
            text: Cow::Borrowed(""),
            location: TextLocation {
                index: 5,
                line: 0,
                column: 5
            }
        }
    });
}

#[test]
fn it_grabs_float_literal() {
    let input = "224";
    let mut tokenizer = make_tokenizer(input);
    match_tokens!(tokenizer {
        Token {
            data: TokenData::NumberLiteral(224f64),
            text: Cow::Borrowed("224"),
            location: TextLocation::default()
        },
        Token {
            data: TokenData::EOF,
            text: Cow::Borrowed(""),
            location: TextLocation {
                index: 3,
                line: 0,
                column: 3
            }
        }
    });
    let input = "2.4";
    let mut tokenizer = make_tokenizer(input);
    match_tokens!(tokenizer {
        Token {
            data: TokenData::NumberLiteral(2.4f64),
            text: Cow::Borrowed("2.4"),
            location: TextLocation::default()
        },
        Token {
            data: TokenData::EOF,
            text: Cow::Borrowed(""),
            location: TextLocation {
                index: 3,
                line: 0,
                column: 3
            }
        }
    });
    let input = "2e4";
    let mut tokenizer = make_tokenizer(input);
    match_tokens!(tokenizer {
        Token {
            data: TokenData::NumberLiteral(2e4f64),
            text: Cow::Borrowed("2e4"),
            location: TextLocation::default()
        },
        Token {
            data: TokenData::EOF,
            text: Cow::Borrowed(""),
            location: TextLocation {
                index: 3,
                line: 0,
                column: 3
            }
        }
    });
    let input = "2.4e4";
    let mut tokenizer = make_tokenizer(input);
    match_tokens!(tokenizer {
        Token {
            data: TokenData::NumberLiteral(2.4e4f64),
            text: Cow::Borrowed("2.4e4"),
            location: TextLocation::default()
        },
        Token {
            data: TokenData::EOF,
            text: Cow::Borrowed(""),
            location: TextLocation {
                index: 5,
                line: 0,
                column: 5
            }
        }
    });
}

#[test]
fn it_ignores_whitespace() {
    let input = "\n\t\r\n";
    let mut tokenizer = make_tokenizer(input);
    match_tokens!(tokenizer {
        Token {
            data: TokenData::EOF,
            text: Cow::Borrowed(""),
            location: TextLocation {
                index: 4,
                line: 2,
                column: 0
            }
        }
    });
}

#[test]
fn it_ignores_line_comment() {
    let input =
    "//comment\nlet x";
    let mut tokenizer = make_tokenizer(input);
    match_tokens!(tokenizer {
        Token {
            data: TokenData::Keyword,
            text: Cow::Borrowed("let"),
            location: TextLocation {
                index: 10,
                line: 1,
                column: 0
            }
        },
        Token {
            data: TokenData::Ident,
            text: Cow::Borrowed("x"),
            location: TextLocation {
                index: 14,
                line: 1,
                column: 4
            }
        },
        Token {
            data: TokenData::EOF,
            text: Cow::Borrowed(""),
            location: TextLocation {
                index: 15,
                line: 1,
                column: 5
            }
        }
    });
}

#[test]
fn it_lexes_complex_input() {
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
                index: 0,
                line: 0,
                column: 0
            }
        },
        Token {
            data: TokenData::Ident,
            text: Cow::Borrowed("x"),
            location: TextLocation {
                index: 4,
                line: 0,
                column: 4
            }
        },
        Token {
            data: TokenData::Symbol,
            text: Cow::Borrowed("="),
            location: TextLocation {
                index: 6,
                line: 0,
                column: 6
            }
        },
        Token {
            data: TokenData::Ident,
            text: Cow::Borrowed("y"),
            location: TextLocation {
                index: 8,
                line: 0,
                column: 8
            }
        },
        Token {
            data: TokenData::Ident,
            text: Cow::Borrowed("y"),
            location: TextLocation {
                index: 10,
                line: 0,
                column: 10
            }
        },
        Token {
            data: TokenData::Symbol,
            text: Cow::Borrowed("+="),
            location: TextLocation {
                index: 12,
                line: 0,
                column: 12
            }
        },
        Token {
            data: TokenData::NumberLiteral(55e7f64),
            text: Cow::Borrowed("55e7"),
            location: TextLocation {
                index: 15,
                line: 0,
                column: 15
            }
        },
        Token {
            data: TokenData::Keyword,
            text: Cow::Borrowed("return"),
            location: TextLocation {
                index: 22,
                line: 1,
                column: 0
            }
        },
        Token {
            data: TokenData::Ident,
            text: Cow::Borrowed("y"),
            location: TextLocation {
                index: 29,
                line: 1,
                column: 7
            }
        },
        Token {
            data: TokenData::Symbol,
            text: Cow::Borrowed("%"),
            location: TextLocation {
                index: 31,
                line: 1,
                column: 9
            }
        },
        Token {
            data: TokenData::Ident,
            text: Cow::Borrowed("x"),
            location: TextLocation {
                index: 33,
                line: 1,
                column: 11
            }
        },
        Token {
            data: TokenData::Symbol,
            text: Cow::Borrowed("+"),
            location: TextLocation {
                index: 35,
                line: 1,
                column: 13
            }
        },
        Token {
            data: TokenData::NumberLiteral(224.5f64),
            text: Cow::Borrowed("224.5"),
            location: TextLocation {
                index: 37,
                line: 1,
                column: 15
            }
        },
        Token {
            data: TokenData::EOF,
            text: Cow::Borrowed(""),
            location: TextLocation {
                index: 42,
                line: 1,
                column: 20
            }
        }
    });
}
