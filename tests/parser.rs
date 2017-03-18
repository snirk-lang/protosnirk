extern crate protosnirk;

use std::io::prelude::*;
use std::str::Chars;

use protosnirk::lex::{TextIter, Tokenizer, IterTokenizer};
use protosnirk::parse::{Parser};

fn read_fixture_file(name: &'static str) -> (String, String) {
    
}
