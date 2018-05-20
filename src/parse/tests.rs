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
pub fn expression_match(expected: &Expression, got: &Expression) {
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
            expression_match(bin.get_left(), bin2.get_left());
            println!("Checking {:?} rhs equality", bin.get_operator());
            expression_match(bin.get_right(), bin2.get_right());
        },
        (&Expression::UnaryOp(ref un), &Expression::UnaryOp(ref un2)) => {
            assert_eq!(un.get_operator(), un2.get_operator(),
                "Unary expression mismatch:\nExpected {:#?}\nGot: {:#?}",
                un, un2);
            println!("Checking {:?} equality", un.get_operator());
            expression_match(un.get_inner(), un2.get_inner());
        },
        (&Expression::IfExpression(ref left), &Expression::IfExpression(ref right)) => {
            println!("Checking ifexpr equality");
            expression_match(left.get_condition(), right.get_condition());
            expression_match(left.get_true_expr(), right.get_true_expr());
            expression_match(left.get_true_expr(), right.get_true_expr());
        },
        (&Expression::FnCall(ref left), &Expression::FnCall(ref right)) => {
            assert_eq!(left.get_text(), right.get_text(),
                "Fn call mismatch:\nExpected: {:#?}\nGot: {:#?}",
                left, right);
            // TODO match on args
        }
        (&Expression::Assignment(ref assign), &Expression::Assignment(ref assign2)) => {
            assert_eq!(assign.get_lvalue(), assign2.get_lvalue(),
                "Assignment mismatch:\nExpected: {:#?}\nGot: {:#?}",
                assign.get_lvalue(), assign2.get_lvalue());
            println!("Checking assignment to {}", assign.get_lvalue().get_name());
            expression_match(assign.get_rvalue(), assign2.get_rvalue());
        },
        (&Expression::Declaration(ref dec), &Expression::Declaration(ref dec2)) => {
            assert!(dec.get_name() == dec2.get_name() && dec.is_mut() == dec2.is_mut(),
                "Declaration mismatch:\nExpected: {:#?}\nGot: {:#?}",
                dec, dec2);
            println!("Checking declaration of {}", dec.get_name());
            expression_match(dec.get_value(), dec2.get_value());
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
            match (left.get_value(), right.get_value()) {
                (&Some(ref left_val), &Some(ref right_val)) => {
                    expression_match(left_val, right_val);
                },
                (&None, &None) => { },
                (ref left_val, ref right_val) => {
                    panic!("Return stmt values did not match:\nExpected {:#?}\nGot {:#?}",
                        left_val, right_val);
                }
            }
        },
        (&Statement::DoBlock(ref left), &Statement::DoBlock(ref right)) => {
            println!("Checking do block match");
            block_match(left.get_block(), right.get_block());
        },
        (&Statement::IfBlock(ref left), &Statement::IfBlock(ref right)) => {
            println!("Checking if blocks");
            let left_conditionals = left.get_conditionals();
            let right_conditionals = right.get_conditionals();
            assert_eq!(left_conditionals.len(), right_conditionals.len(),
                "If block conditional length mismatch:\nExpected {:#?}\nGot: {:#?}",
                left, right);
            for (left_cond, right_cond) in
                        left_conditionals.iter().zip(right_conditionals.iter()) {
                println!("Checking if block conditional");
                expression_match(left_cond.get_condition(), right_cond.get_condition());
                block_match(left_cond.get_block(), right_cond.get_block());
            }
            match (left.get_else(), right.get_else()) {
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
        assert_eq!(expected.get_stmts().len(), got.get_stmts().len(),
            "Blocks had differing stmt counts:\nExpected: {:#?}\nGot: {:#?}",
            expected, got);
        for (ref left, ref right) in
                    expected.get_stmts().iter().zip(got.get_stmts().iter()) {
            statement_match(left, right);
        }
    }
}

//#[test]
fn _parse_example() {
    let inputs = &[
r#"
fn factHelper(acc, n)
    if n == 0
        acc
    else
        factHelper(acc: acc * n, n: n - 1)
fn fact(n)
    factHelper(n: n, acc: 1)
"#,
];
    ::env_logger::Builder::new()
        .parse("TRACE")
        .init();
    for input in inputs {
        let mut parser = parser(input);
        trace!("Parsing input {:?}", input);
        trace!("Resulting program:\n{:#?}", parser.parse_unit());
    }
}
