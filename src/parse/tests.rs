use std::str::Chars;

use lex::{Token, IterTokenizer};
use lex::tests::make_tokenizer;
use ast::*;
use parse::{Parser};

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
    assert_eq!(expected.data(), got.data(),
        "token_eq: {:?} != {:?}", expected, got);
    assert_eq!(expected.text(), got.text(),
        "token_eq: {:?} != {:?}", expected, got);
}

/// Ensure the values of two expressions match.
///
/// Ignores position information in tokens
pub fn expression_match(expected: &Expression, got: &Expression) {
    match (expected, got) {
        (&Expression::Literal(ref lit), &Expression::Literal(ref lit2)) => {
            assert_eq!(lit.value(), lit2.value(),
                "Expression mismatch in literals: expected {:?}, got {:?}",
                lit.value(), lit2.value());
        },
        (&Expression::VariableRef(ref var), &Expression::VariableRef(ref var2)) => {
            assert_eq!(var.name(), var2.name(),
                "Variable reference mismatch: expected {:?}, got {:?}",
                var.name(), var2.name());
        },
        (&Expression::BinaryOp(ref bin), &Expression::BinaryOp(ref bin2)) => {
            assert_eq!(bin.operator(), bin2.operator(),
                "Binary expression mismatch:\nExpected {:#?}\nGot: {:#?}",
                bin, bin2);
            println!("Checking {:?} lhs equality", bin.operator());
            expression_match(bin.left(), bin2.left());
            println!("Checking {:?} rhs equality", bin.operator());
            expression_match(bin.right(), bin2.right());
        },
        (&Expression::UnaryOp(ref un), &Expression::UnaryOp(ref un2)) => {
            assert_eq!(un.operator(), un2.operator(),
                "Unary expression mismatch:\nExpected {:#?}\nGot: {:#?}",
                un, un2);
            println!("Checking {:?} equality", un.operator());
            expression_match(un.inner(), un2.inner());
        },
        (&Expression::IfExpression(ref left), &Expression::IfExpression(ref right)) => {
            println!("Checking ifexpr equality");
            expression_match(left.condition(), right.condition());
            expression_match(left.true_expr(), right.true_expr());
            expression_match(left.true_expr(), right.true_expr());
        },
        (&Expression::FnCall(ref left), &Expression::FnCall(ref right)) => {
            assert_eq!(left.text(), right.text(),
                "Fn call mismatch:\nExpected: {:#?}\nGot: {:#?}",
                left, right);
            // TODO match on args
        }
        (&Expression::Assignment(ref assign), &Expression::Assignment(ref assign2)) => {
            assert_eq!(assign.lvalue(), assign2.lvalue(),
                "Assignment mismatch:\nExpected: {:#?}\nGot: {:#?}",
                assign.lvalue(), assign2.lvalue());
            println!("Checking assignment to {}", assign.lvalue().name());
            expression_match(assign.rvalue(), assign2.rvalue());
        },
        (&Expression::Declaration(ref dec), &Expression::Declaration(ref dec2)) => {
            assert!(dec.name() == dec2.name() && dec.is_mut() == dec2.is_mut(),
                "Declaration mismatch:\nExpected: {:#?}\nGot: {:#?}",
                dec, dec2);
            println!("Checking declaration of {}", dec.name());
            expression_match(dec.value(), dec2.value());
        },
        (ref other, ref other2) => {
            panic!("Expressions did not match:\nExpected {:#?}\nGot {:#?}",
                other, other2);
        }
    }
}

pub fn statement_match(expected: &Statement, got: &Statement) {
    match (expected, got) {
        (&Statement::Expression(ref left), &Statement::Expression(ref right)) => {
            expression_match(left, right);
        },
        (&Statement::Return(ref left), &Statement::Return(ref right)) => {
            println!("Checking return stmts");
            match (left.value(), right.value()) {
                (Some(ref left_val), Some(ref right_val)) => {
                    expression_match(left_val, right_val);
                },
                (None, None) => { },
                (ref left_val, ref right_val) => {
                    panic!("Return stmt values did not match:\nExpected {:#?}\nGot {:#?}",
                        left_val, right_val);
                }
            }
        },
        (&Statement::DoBlock(ref left), &Statement::DoBlock(ref right)) => {
            println!("Checking do block match");
            block_match(left.block(), right.block());
        },
        (&Statement::IfBlock(ref left), &Statement::IfBlock(ref right)) => {
            println!("Checking if blocks");
            let left_conditionals = left.conditionals();
            let right_conditionals = right.conditionals();
            assert_eq!(left_conditionals.len(), right_conditionals.len(),
                "If block conditional length mismatch:\nExpected {:#?}\nGot: {:#?}",
                left, right);
            for (left_cond, right_cond) in
                        left_conditionals.iter().zip(right_conditionals.iter()) {
                println!("Checking if block conditional");
                expression_match(left_cond.condition(), right_cond.condition());
                block_match(left_cond.block(), right_cond.block());
            }
            match (left.else_block(), right.else_block()) {
                (Some(&(_, ref left)), Some(&(_, ref right))) => {
                    println!("Checking else blocks");
                    block_match(left, right);
                },
                (None, None) => {},
                (ref left_else, ref right_else) => {
                    panic!("If block elses did not match:\nExpected {:#?}\nGot: {:#?}",
                        left_else, right_else);
                }
            }
        },
        (ref left, ref right) => {
            panic!("Statements did not match:\nExpected {:#?}\nGot: {:#?}",
                left, right);
        }
    }

    fn block_match(expected: &Block, got: &Block) {
        assert_eq!(expected.stmts().len(), got.stmts().len(),
            "Blocks had differing stmt counts:\nExpected: {:#?}\nGot: {:#?}",
            expected, got);
        for (ref left, ref right) in
                    expected.stmts().iter().zip(got.stmts().iter()) {
            statement_match(left, right);
        }
    }
}

pub const FACT_AND_HELPER: &str =
r#"
fn factHelper(acc: float, n: float) -> float
    if n == 0
        acc
    else
        factHelper(acc: acc * n, n: n - 1)
fn fact(n: float) -> float
    factHelper(n: n, acc: 1)
"#;

pub const BLOCKS_IN_BLOCKS: &str =
r#"
fn blocksInBlocks(x: float) -> float
    do
        do
            if x < 0
                do
                    let y = x + 1
                    y + 2
            else if x == 1
                let z = x - 1
                z + 2
            else
                x + 1
"#;

pub const FLOAT_TYPE_ALIAS: &str =
r#"
typedef MyFloat = float

fn foo(x: MyFloat) -> float
    x
"#;

#[ignore]
#[test]
fn parse_example() {
    let inputs = &[FLOAT_TYPE_ALIAS];
    ::env_logger::Builder::new()
        .parse("TRACE")
        .target(::env_logger::Target::Stdout)
        .init();
    for input in inputs {
        let mut parser = parser(input);
        trace!("Parsing input:\n{}\n", input);
        match parser.parse_unit() {
            Ok(unit) => { trace!("Parsed unit: {:#?}", unit); }
            Err(err) => { trace!("Error while parsing unit: {:?}", err); }
        }
    }
}
