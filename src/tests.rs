//! Integration tests n stuff

use parse::tests::make_parser;

use compile::Compiler;
use run::{VM, Value};

fn run_program(program: &'static str) -> Value {
    let program = make_parser(program).parse_program().unwrap();
    let chunk = Compiler { }.compile(program);
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
