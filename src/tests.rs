//! Integration tests n stuff

use parse::tests::parser;

use compile::Compiler;
use run::{VM, Value};

fn run_program(program: &'static str) -> Value {
    let program = parser(program).parse_unit();
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
        let mut a = 5 % -2 \n\
        a *= 2 \n\
        a /= 2 \n\
        a %= 1 \n\
        a = a + 1 \n\
        y += a \n\
        return y - 2";
    assert_eq!(run_program(program), Value(0f64));
}
