extern crate protosnirk;

use std::io::prelude::*;
use std::io::{BufReader};
use std::fs::File;

use protosnirk::lex::{TokenType, IterTokenizer};

fn verify_keywords_list(fixture_name: &'static str) {
    let input_name = format!("tests/lex/{}.input", fixture_name);
    let output_name = format!("tests/lex/{}.output", fixture_name);
    let mut input = String::with_capacity(100);
    File::open(&input_name)
        .expect(&input_name)
        .read_to_string(&mut input)
        .expect(&input_name);
    println!("Read input file {}", input_name);
    let mut output_lines = BufReader::new(File::open(&output_name).expect(&output_name)).lines();
    println!("Opened expected file {}", output_name);
    let mut current_line = 0usize;
    let mut tokenizer = IterTokenizer::new(input.chars());
    loop {
        let token = tokenizer.next();
        let next_line = output_lines.next();
        current_line += 1;

        if let Some(line_result) = next_line {
            let found = line_result.expect("Error reading line from tokens");
            let line = found.trim();
            if line == "\\+" {
                assert_eq!(TokenType::BeginBlock, token.get_type(),
                    "\nExpected \\+ from line {}, got {:?}", current_line, token);
                continue
            }
            else if line == "\\-" {
                assert_eq!(TokenType::EndBlock, token.get_type(),
                    "\nExpected \\- from line {}, got {:?}", current_line, token);
                continue
            }
            let split = line.splitn(2, " ").collect::<Vec<_>>();
            let (token_type, token_text) = (split[0], split[1]);
            if token_type == "li" {
                // todoo
                continue
            }
            let expected_type = match token_type {
                "kw" => TokenType::Keyword,
                "sy" => TokenType::Symbol,
                "id" => TokenType::Ident,
                _ => panic!("Invalid line {} of {}: {}",
                        current_line, output_name, line)
            };
            assert_eq!(expected_type, token.get_type(),
                "\nExpected {:?} from line {}, got {:?}", expected_type, current_line, token);
            println!("{}: Matched type {:?}", current_line, expected_type);
            assert_eq!(token_text, token.get_text(),
                "\nExpected {} from line {}, got {}", token_text, current_line, token.get_text());
            println!("{}: Matched text {}", current_line, token_text);
        }
        else if token.get_type() != TokenType::EOF {
            println!("Expected EOF after {} tokens, got token {:?}", current_line, token);
            panic!("Invalid input {}", fixture_name);
        }
        else {
            return
        }
    }
}

#[test]
fn all_keywords() {
    verify_keywords_list("all-keywords");
}

#[test]
fn fn_indent() {
    verify_keywords_list("fn-indent");
}

#[test]
fn block_indent() {
    verify_keywords_list("block-indent");
}
