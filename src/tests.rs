//! Mid-sized integration tests for protosnirk.
//! Most testing is done in the external tests.

use super::lex::{Tokenizer, StaticStrTokenizer, TokenData};

macro_rules! match_tokens {
    ($tokenizer:ident { $($name:ident $value:expr),* }) => {
        $(
            let next = $tokenizer.next();
            let expected = TokenData::$name(::std::borrow::Cow::Borrowed($value));
            assert_eq!(next.data, expected);
        )*
    }
}

#[test]
fn parser_kinda_does_a_thing_maybe() {
    let input =
    "let x".into();

    let mut tokenizer = StaticStrTokenizer::new(input);
    match_tokens!(tokenizer {
        Keyword "let",
        Ident "x"
    });
}
