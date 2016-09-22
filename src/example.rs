//! ISO date parse example

#![allow(unused_imports, dead_code)]

use std::str;
use std::borrow::ToOwned;
use nom::{self, IResult};

pub struct ISODate {
    year: i16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8
}

named!(pub sign <&[u8], bool>,
       alt!(
           tag!("+") => { |_| true } |
           tag!("-") => { |_| false }
       ));

// A variable binding
#[derive(PartialEq, Eq, Debug)]
pub struct Binding {
    pub name: String,
    pub mutable: bool,
    pub value: String
}

// Example
// Parse `let x = 0` into Let { name: 'x', val: 0 }

macro_rules! keyword {
    ($kw:ident) => {
        named!(concat!(keyword_, $kw) <&[u8], &[u8]>, tag!(stringify!($kw)));
    };
    (pub $kw:ident) => {
        named!(pub concat!(keyword_, $kw) <&[u8], &[u8]>, tag!(stringify!($kw)));
    };

    ($name:ident = $kw:expr) => {
        named!($name <&[u8], &[u8]>, tag!($kw));
    };
    (pub $name:ident = $kw:expr) => {
        named!(pub $name <&[u8], &[u8]>, tag!($kw));
    };
}

keyword!(pub keyword_let = "let");
keyword!(pub keyword_eq = "=");
keyword!(pub keyword_mut = "mut");

named!(pub get_digits <&[u8], &[u8]>,
       take_while1!(
           nom::is_digit
       ));

named!(pub spacing <&[u8], &[u8]>,
       take_while1!(
           nom::is_space
       ));

named!(pub ident <&[u8], &[u8]>,
       take_while!(
           nom::is_alphanumeric
       ));

named!(pub declaration <&[u8], bool>,
       alt!(
           keyword_let => { |_| false } |
           keyword_mut => { |_| true }
       ));

named!(pub binding <&[u8], Binding>,
       chain!(
           decl: declaration ~
           spacing           ~
           name: ident       ~
           spacing           ~
           keyword_eq        ~
           spacing           ~
           val_str: get_digits,
           || {
               Binding {
                   name: get_string(name),
                   mutable: decl,
                   value: get_string(val_str)
               }
           }
       ));

fn get_string(input: &[u8]) -> String {
    str::from_utf8(input).unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::IResult;
    use nom::IResult::*;

    const EMPTY: &'static [u8] = &[];

    /// assert_parse!(parser <- input, Value);
    macro_rules! assert_parse {
        ($parser:ident <- $input:expr, $value:expr) => {
            assert_eq!($parser(&$input[..]), ::nom::IResult::Done(EMPTY, $value));
        };
    }

    #[test]
    fn sign_works() {
        assert_eq!(sign(&b"+"[..]), Done(EMPTY, true));
        assert_eq!(sign(&b"-"[..]), Done(EMPTY, false));
        assert_parse!(sign <- b"+", true);
    }

    #[test]
    fn binding_works() {
        assert_eq!(binding(&b"let foo = 12"[..]),
                           Done(EMPTY, Binding { name: "foo".to_string(), mutable: false, value: "12".to_string() }) );
        assert_parse!(binding <-
                      b"mut foobarbaz = 1234",
                      Binding {
                          name: "foobarbaz".into(),
                          mutable: true,
                          value: "1234".into()
                      });
        assert_parse!(
            binding <- b"let asdf = 22243",
            Binding {
                name: "asdf".into(),
                mutable: false,
                value: "22243".into()
            });
    }
}
