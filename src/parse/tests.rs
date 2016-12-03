use std::borrow::Cow;

use lex::{Tokenizer, IterTokenizer};
use lex::tests::make_tokenizer;
use parse::{Parser, Precedence};
use parse::symbol;
use parse::expression::*;

fn match_expr(got: Expression, expected: Expression) {
    assert!(got == expected,
        "\nExpected: {:#?}\nActual: {:#?}", expected, got);
}

fn make_parser(input: &'static str) -> Parser {
    let tokenizer = make_tokenizer(input);
    Parser::new(Box::new(tokenizer) as Box<Tokenizer>)
}

#[test]
fn it_parses_an_assignment_to_constant() {
    let mut parser = make_parser("let x = 0");
    let expr = parser.expression(Precedence::Max).unwrap();
    match_expr(expr, Expression::Declaration(
                        Declaration::new(Cow::Borrowed("x"), false, Box::new(Expression::Literal(Literal::new(0f64))))));
}

#[test]
fn it_parses_simple_addition_expression() {
    let mut parser = make_parser("x + 3/4");
    let expr = parser.expression(Precedence::Min).unwrap();
    match_expr(expr, Expression::Literal(Literal::new(1f64)));
}

#[test]
fn it_parses_a_multi_statement_block() {
    let mut parser = make_parser("let x = 0 return x + 1");
    let block = parser.block().unwrap();
    assert_eq!(block, Vec::new(), "Block: {:#?}", block);
}

#[test]
fn it_parses_complex_block() {
    let input = "\
    let mut x = 0 \n\
    let y = 0 \n\
    let z = x/y \n\
    x = x + y/2 + z % 5 \n\
    y += x / 2 \t\n\
    return z \
    ";
    let mut parser = make_parser(input);
    let block = parser.block().unwrap();
    assert_eq!(block, Vec::new());
}
