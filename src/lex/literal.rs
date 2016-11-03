//! Literal values

use nom::is_digit;
use std::str;

/// Parses number literals into strings
named!(pub lit_number <&[u8], &[u8]>, take_while!(call!(is_digit)));

/// Attempts to convert a slice to an f64.
pub fn f64_from_slice(input: &[u8]) -> Option<f64> {
    str::from_utf8(input).ok()
        .and_then(|s| s.parse().ok())
}

#[cfg(test)]
mod test {
    use super::*;
    use nom::IResult::*;

    #[test]
    fn it_gets_a_number() {
        assert_eq!(lit_number(&b"1234567890"[..]),
                   Done(&b""[..], &b"1234567890"[..]));
    }

    #[test]
    fn it_does_not_get_not_a_number() {
        assert_eq!(lit_number(&b"twelve 22"[..]),
                   Done(&b"twelve 22"[..], &b""[..]));
    }
}
