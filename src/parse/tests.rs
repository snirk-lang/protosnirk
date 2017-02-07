use std::borrow::Cow;
use std::str::Chars;

use lex::{Token, TokenData, TextLocation, Tokenizer, IterTokenizer};
use lex::tests::make_tokenizer;
use parse::{Parser};
use parse::symbol::{self, Precedence};
use parse::ast::*;

pub fn expect_eq<T: ::std::fmt::Debug + PartialEq>(got: T, expected: T) {
    assert!(got == expected,
        "\nExpected: {:#?}\nActual: {:#?}", expected, got);
}

/// Produces a parser that parses the given input
pub fn parser(input: &'static str) -> Parser<IterTokenizer<Chars<'static>>> {
    let tokenizer = make_tokenizer(input);
    Parser::new(tokenizer)
}

/// Produces a parser that only returns new EOF tokens
pub fn eof_parser() -> Parser<IterTokenizer<Chars<'static>>> {
    parser("")
}

/// Check that two tokens are equal, without looking at locatoin
pub fn token_eq(expected: Token, got: Token) {
    assert_eq!(expected.data, got.data,
        "token_eq: {:?} != {:?}", expected, got);
    assert_eq!(expected.text, got.text,
        "token_eq: {:?} != {:?}", expected, got);
}

/// Ensure the values of two expressions match.
///
/// Ignores position information in tokens
pub fn expression_eq(expected: &Expression, got: &Expression) {
    match (expected, got) {
        (&Expression::Literal(ref lit), &Expression::Literal(ref lit2)) => {
            assert_eq!(lit.get_value(), lit2.get_value(),
                "Expression mismatch in literals: expected {}, got {}",
                lit.get_value(), lit2.get_value());
        },
        (&Expression::VariableRef(ref var), &Expression::VariableRef(ref var2)) => {
            assert_eq!(var.get_name(), var2.get_name(),
                "Variable reference mismatch: expected {:?}, got {:?}",
                var.get_name(), var2.get_name());
        },
        (&Expression::BinaryOp(ref bin), &Expression::BinaryOp(ref bin2)) => {
            assert_eq!(bin.get_operator(), bin2.get_operator(),
                "Binary expression mismatch:\nExpected {:#?}\nGot: {:#?}",
                bin, bin2);
            println!("Checking {:?} lhs equality", bin.get_operator());
            expression_eq(bin.get_left(), bin2.get_left());
            println!("Checking {:?} rhs equality", bin.get_operator());
            expression_eq(bin.get_right(), bin2.get_right());
        },
        (&Expression::UnaryOp(ref un), &Expression::UnaryOp(ref un2)) => {
            assert_eq!(un.get_operator(), un2.get_operator(),
                "Unary expression mismatch:\nExpected {:#?}\nGot: {:#?}",
                un, un2);
            println!("Checking {:?} equality", un.get_operator());
            expression_eq(un.get_inner(), un2.get_inner());
        },
        (&Expression::Assignment(ref assign), &Expression::Assignment(ref assign2)) => {
            assert_eq!(assign.get_lvalue(), assign2.get_lvalue(),
                "Assignment mismatch:\nExpected: {:#?}\nGot: {:#?}",
                assign.get_lvalue(), assign2.get_lvalue());
            println!("Checking assignment to {}", assign.get_lvalue().get_name());
            expression_eq(assign.get_rvalue(), assign2.get_rvalue());
        },
        (&Expression::Declaration(ref dec), &Expression::Declaration(ref dec2)) => {
            assert!(dec.get_name() == dec2.get_name() && dec.is_mut() == dec2.is_mut(),
                "Declaration mismatch:\nExpected: {:#?}\nGot: {:#?}",
                dec, dec2);
            println!("Checking declaration of {}", dec.get_name());
            expression_eq(dec.get_value(), dec2.get_value());
        },
        (ref other, ref other2) => {
            panic!("Expressions did not match:\nExpected {:#?}\nGot {:#?}",
                other, other2);
        }
    }
}

pub fn parse_fails() {

}

/*
#[test]
fn it_parses_an_assignment_to_constant() {
    let mut parser = parser("let x = 0");
    let expr = parser.expression(Precedence::Max).unwrap();
    let expected = Expression::Declaration(Declaration {
        mutable: false,
        token: Token {
            location: TextLocation {
                index: 0,
                line: 0,
                column: 0
            },
            text: Cow::Borrowed("let"),
            data: TokenData::Keyword
        },
        ident: Identifier {
            index: ScopeIndex
        },
        value: Box::new(Expression::Literal(Literal {
            token: Token {
                location: TextLocation {
                    index: 8,
                    line: 0,
                    column: 8
                },
                text: Cow::Borrowed("0"),
                data: TokenData::NumberLiteral(0f64)
            }
        }))
    });
    assert_eq!(expr, expected, "\nExpected: {:#?}\nGot: {:#?}", expected, expr);
}

#[test]
fn it_parses_simple_addition_expression() {
    let mut parser = parser("x + 3/4");
    let expr = parser.expression(Precedence::Min).unwrap();
    let expected = Expression::BinaryOp(BinaryOperation {
        operator: Operator::Addition,
        op_token: Token {
            location: TextLocation {
                index: 2,
                line: 0,
                column: 2
            },
            text: Cow::Borrowed("+"),
            data: TokenData::Symbol
        },
        left: Box::new(Expression::VariableRef(Identifier {
            token: Token {
                location: TextLocation {
                    index: 0,
                    line: 0,
                    column: 0
                },
                text: Cow::Borrowed("x"),
                data: TokenData::Ident
            }
        })),
        right: Box::new(Expression::BinaryOp(BinaryOperation {
            operator: Operator::Division,
            op_token: Token {
                location: TextLocation {
                    index: 5,
                    line: 0,
                    column: 5
                },
                text: Cow::Borrowed("/"),
                data: TokenData::Symbol
            },
            left: Box::new(Expression::Literal(Literal {
                token: Token {
                    location: TextLocation {
                        index: 4,
                        line: 0,
                        column: 4
                    },
                    text: Cow::Borrowed("3"),
                    data: TokenData::NumberLiteral(3f64)
                }
            })),
            right: Box::new(Expression::Literal(Literal {
                token: Token {
                    location: TextLocation {
                        index: 6,
                        line: 0,
                        column: 6
                    },
                    text: Cow::Borrowed("4"),
                    data: TokenData::NumberLiteral(4f64)
                }
            }))
        }))
    });
    assert_eq!(expr, expected, "\nExpected: {:#?}\nGot: {:#?}", expected, expr);
}

#[test]
fn it_parses_a_multi_statement_block() {
    let mut parser = parser("let x = 0 return x + 1");
    let block = parser.block().unwrap();
    let expected = vec![
        Expression::Declaration(Declaration {
            mutable: false,
            token: Token {
                location: TextLocation {
                    index: 4,
                    line: 0,
                    column: 4
                },
                text: Cow::Borrowed("x"),
                data: TokenData::Ident
            },
            value: Box::new(Expression::Literal(Literal {
                token: Token {
                    location: TextLocation {
                        index: 8,
                        line: 0,
                        column: 8
                    },
                    text: Cow::Borrowed("0"),
                    data: TokenData::NumberLiteral(0f64)
                }
            }))
        }),
        Expression::Return(Return {
            token: Token {
                location: TextLocation {
                    index: 10,
                    line: 0,
                    column: 10
                },
                text: Cow::Borrowed("return"),
                data: TokenData::Keyword
            },
            value: Some(Box::new(Expression::BinaryOp(BinaryOperation {
                operator: Operator::Addition,
                op_token: Token {
                    location: TextLocation {
                        index: 19,
                        line: 0,
                        column: 19
                    },
                    text: Cow::Borrowed("+"),
                    data: TokenData::Symbol
                },
                left: Box::new(Expression::VariableRef(Identifier {
                    token: Token {
                        location: TextLocation {
                            index: 17,
                            line: 0,
                            column: 17
                        },
                        text: Cow::Borrowed("x"),
                        data: TokenData::Ident
                    }
                })),
                right: Box::new(Expression::Literal(Literal {
                    token: Token {
                        location: TextLocation {
                            index: 21,
                            line: 0,
                            column: 21
                        },
                        text: Cow::Borrowed("1"),
                        data: TokenData::NumberLiteral(1f64)
                    }
                }))
            })))
        })
    ];
    assert_eq!(block.statements, expected, "Got {:#?}", block);
}
*/

/*
#[test]
fn it_parses_pemdas() {
    let input =
        "let x = 1 + 2 * 3 - 4 / 5 % 6 x";
    let mut parser = parser(input);
    let block = parser.block();
    println!("{:#?}", block);
    let expected = vec![
        Expression::Declaration(Declaration {
            mutable: false,
            token: Token {
                location: TextLocation { index: 4, line: 0, column: 4 },
                text: Cow::Borrowed("x"),
                data: TokenData::Ident
            },
            value: Box::new(Expression::BinaryOp(BinaryOperation {
            operator: Operator::Subtraction,
            op_token: Token {
                location: TextLocation { index: 18, line: 0, column: 18 },
                text: Cow::Borrowed("-"),
                data: TokenData::Symbol
            },
            left: Box::new(Expression::BinaryOp(BinaryOperation {
                operator: Operator::Addition,
                op_token: Token {
                    location: TextLocation { index: 10, line: 0, column: 10 },
                    text: Cow::Borrowed("+"),
                    data: TokenData::Symbol
                },
                left: Box::new(Expression::Literal(Literal {
                    token: Token {
                        location: TextLocation { index: 8, line: 0, column: 8 },
                        text: Cow::Borrowed("1"),
                        data: TokenData::NumberLiteral(1f64)
                    }
                })),
                right: Box::new(Expression::BinaryOp(BinaryOperation {
                    operator: Operator::Multiplication,
                    op_token: Token {
                        location: TextLocation { index: 14, line: 0, column: 14 },
                        text: Cow::Borrowed("*"),
                        data: TokenData::Symbol
                    },
                    left: Box::new(Expression::Literal(Literal {
                        token: Token {
                            location: TextLocation { index: 12, line: 0, column: 12 },
                            text: Cow::Borrowed("2"),
                            data: TokenData::NumberLiteral(2f64)
                        }
                    })),
                    right: Box::new(Expression::Literal(Literal {
                        token: Token {
                            location: TextLocation { index: 16, line: 0, column: 16 },
                            text: Cow::Borrowed("3"),
                            data: TokenData::NumberLiteral(3f64)
                        }
                    }))
                }))
            })),
            right: Box::new(Expression::BinaryOp(BinaryOperation {
                operator: Operator::Division,
                op_token: Token {
                    location: TextLocation { index: 22, line: 0, column: 22 },
                    text: Cow::Borrowed("/"),
                    data: TokenData::Symbol
                },
                left: Box::new(Expression::Literal(Literal {
                    token: Token {
                        location: TextLocation { index: 20, line: 0, column: 20 },
                        text: Cow::Borrowed("4"),
                        data: TokenData::NumberLiteral(4f64)
                    }
                })),
                right: Box::new(Expression::BinaryOp(BinaryOperation {
                    operator: Operator::Modulus,
                    op_token: Token {
                        location: TextLocation { index: 26, line: 0, column: 26 },
                        text: Cow::Borrowed("%"),
                        data: TokenData::Symbol
                    },
                    left: Box::new(Expression::Literal(Literal {
                        token: Token {
                            location: TextLocation { index: 24, line: 0, column: 24 },
                            text: Cow::Borrowed("5"),
                            data: TokenData::NumberLiteral(5f64)
                        }
                    })),
                    right: Box::new(Expression::Literal(Literal {
                        token: Token {
                            location: TextLocation { index: 28, line: 0, column: 28 },
                            text: Cow::Borrowed("6"),
                            data: TokenData::NumberLiteral(6f64)
                        }
                    }))
                }))
            }))
        }))
    }),
    Expression::VariableRef(Identifier {
        token: Token {
            location: TextLocation { index: 30, line: 0, column: 30 },
            text: Cow::Borrowed("x"),
            data: TokenData::Ident
        }
    })];
    assert_eq!(block.unwrap().statements, expected);
}
*/
