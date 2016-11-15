//! Mid-sized integration tests for protosnirk.
//! Most testing is done in the external tests.

use super::lex::{Tokenizer, Parser, Expression};

#[test]
fn parser_kinda_does_a_thing_maybe() {
    let input =
    "let x = 5 + 3 * 2 % 4 + 7".into();

    let mut tokenizer = Tokenizer::from_string(input);
}
