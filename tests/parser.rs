extern crate protosnirk;

use std::io::prelude::*;
use std::vec::IntoIter;
use std::fs::File;

use protosnirk::lex::{IterTokenizer};
use protosnirk::parse::{Parser};

fn read_fixture_file(name: &'static str) -> Parser<IterTokenizer<IntoIter<char>>> {
    let file_name = format!("tests/parse/{}.protosnirk", name);
    let mut text = String::with_capacity(128usize);
    File::open(file_name).expect(name).read_to_string(&mut text).expect(name);
    println!("Input file:\n{}", text);
    Parser::new(IterTokenizer::new(text.chars().collect::<Vec<_>>().into_iter()))
}

macro_rules! ensure_parse_tests {
    ( $($name:ident: $parse_method:ident,)* ) => {
        $(#[test]
        fn $name() {
            let mut parser = read_fixture_file(stringify!($name));
            parser.$parse_method().unwrap();
        })*
    }
}

ensure_parse_tests! {
    block_conditional: block,
    empty_fns: parse_unit,
    fn_def: item,
    fn_indented_params: item,
    fn_inline: item,
    factorial: parse_unit,
}
