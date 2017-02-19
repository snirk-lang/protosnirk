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

#[test]
fn parse_examples() {
    let inputs = &[
r#"let mut x = 0
if x + 0
    x += 1
x"#,

r#"1 > 2 => 1 else 2"#,

r#"let x = 1 > 2 => 1 else 2"#,

r#"let mut x = 5
if x + 5
    x + 1
else if x - 5
    x - 1
else
    x"#
];
    for input in inputs {
        let mut parser = parser(input);
        println!("{:#?}", parser.parse_unit());
    }
    panic!("Test needs to fail");
}
