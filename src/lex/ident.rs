use nom::{Err, ErrorKind, IResult, Needed, is_digit, is_alphabetic};

/// Recognizes identifiers.
pub fn ident(input: &[u8]) -> IResult<&[u8], &[u8]> {
    if input.len() == 0 {
        return IResult::Incomplete(Needed::Size(1))
    }
    if is_digit(input[0]) {
        return IResult::Error(Err::Position(ErrorKind::AlphaNumeric, &input[0..1]))
    }
    for (ix, c) in input.iter().enumerate() {
        if !is_digit(*c)
            && !is_alphabetic(*c)
            && *c != b'_' {
                return IResult::Done(&input[ix..], &input[0..ix])
            }
    }
    return IResult::Done(&input[input.len()..], &input[0..input.len()])
}

#[cfg(test)]
mod test {
    use super::*;
    use nom::IResult::*;
    use nom::{Err, ErrorKind};

    #[test]
    fn it_gets_whole_idents() {
        assert_eq!(ident(&b"foo_bAr_baz_12"[..]),
                   Done(&b""[..], &b"foo_bAr_baz_12"[..]));
        assert_eq!(ident(&b"f000000000"[..]),
                   Done(&b""[..], &b"f000000000"[..]));
        assert_eq!(ident(&b"BAR_BAZ_BAR__"[..]),
                   Done(&b""[..], &b"BAR_BAZ_BAR__"[..]));
        assert_eq!(ident(&b"_foo"[..]),
                   Done(&b""[..], &b"_foo"[..]));

    }

    #[test]
    fn it_stops_after_a_space() {
        assert_eq!(ident(&b"fooBazBar foo"[..]),
                   Done(&b" foo"[..], &b"fooBazBar"[..]));
        assert_eq!(ident(&b"fooBazBar-foo"[..]),
                   Done(&b"-foo"[..], &b"fooBazBar"[..]));
        assert_eq!(ident(&b"fooBazBar*foo"[..]),
                   Done(&b"*foo"[..], &b"fooBazBar"[..]));
    }

    #[test]
    fn it_errors_leading_digits() {
        assert_eq!(ident(&b"12foobaz"[..]),
                   Error(Err::Position(ErrorKind::AlphaNumeric, &b"1"[..])));
    }
}
