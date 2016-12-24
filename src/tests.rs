//! Integration tests n stuff

use parse::tests::make_parser;

use compile::Compiler;
use run::{VM, Value};

fn run_program(program: &'static str) -> Value {
    let program = make_parser(program).parse_program();
    if !program.is_ok() {
        panic!("Error parsing program: {:#?}", program);
    }
    let chunk = Compiler { }.compile(program.unwrap());
    VM { }.eval_chunk(chunk)
}

#[test]
fn hello_world() {
    let program =
        "let x = 0 \n\
        let mut y = x + 1 \n\
        let z = 2 \n\
        y += z \n\
        return y - 2";
    assert_eq!(run_program(program), Value(1f64));
}

#[test]
fn all_math_operators() {
    let program =
        "let x = 0 \n\
        let mut y = -x - 1 \n\
        let z = 2 \n\
        y += z \n\
        let mut a = 5 % -2
        a *= 2
        a /= 2
        a %= 1
        a = a + 1
        y += a
        return y - 2";
    assert_eq!(run_program(program), Value(0f64));
}
