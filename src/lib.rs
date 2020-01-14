//! Adaptors to add Unicode-aware parsing to Nom.

use nom::AsChar;

// HELPERS

/// nom::AsChar for only unicode-aware character types.
pub trait IsChar: AsChar {
}

impl IsChar for char {
}

impl<'a> IsChar for &'a char {
}

// Generates `is_x` implied helper functions.
macro_rules! is_impl {
    ($($name:ident)*) => ($(
        #[inline(always)]
        fn $name<T: IsChar>(item: T) -> bool {
            item.as_char().$name()
        }
    )*);
}

is_impl! {
    is_alphabetic
    is_lowercase
    is_uppercase
    is_whitespace
    is_alphanumeric
    is_control
    is_numeric
    is_ascii
}

// Macro to dynamically document a generated function.
macro_rules! doc {
    ($x:expr, $item:item) => (
        #[doc = $x]
        $item
    );
}

// COMPLETE

/// Nom complete parsing API functions.
pub mod complete {
    use super::*;
    use nom::{IResult, InputTakeAtPosition};
    use nom::error::{ErrorKind, ParseError};

    // Dynamically generate both the zero and 1 parse APIs.
    macro_rules! parse_impl {
        ($($name0:ident, $name1:ident, $kind:ident, $callback:ident, $comment:expr)*) => ($(
            doc!(concat!("Recognizes zero or more ", $comment),
                #[inline]
                pub fn $name0<T, Error>(input: T)
                    -> IResult<T, T, Error>
                    where T: InputTakeAtPosition,
                          <T as InputTakeAtPosition>::Item: IsChar,
                          Error: ParseError<T>
                {
                  input.split_at_position_complete(|item| !$callback(item))
                }
            );

            doc!(concat!("Recognizes one or more ", $comment),
                #[inline]
                pub fn $name1<T, Error>(input: T)
                    -> IResult<T, T, Error>
                    where T: InputTakeAtPosition,
                          <T as InputTakeAtPosition>::Item: IsChar,
                          Error: ParseError<T>
                {
                  input.split_at_position1_complete(|item| !$callback(item), ErrorKind::$kind)
                }
            );
        )*);
    }

    parse_impl! {
        alpha0,         alpha1,         Alpha,          is_alphabetic,      "lowercase and uppercase alphabetic Unicode characters."
        lower0,         lower1,         Alpha,          is_lowercase,       "lowercase alphabetic Unicode characters."
        upper0,         upper1,         Alpha,          is_uppercase,       "lowercase alphabetic Unicode characters."
        space0,         space1,         Space,          is_whitespace,      "whitespace Unicode characters."
        alphanumeric0,  alphanumeric1,  AlphaNumeric,   is_alphanumeric,    "alphabetic and numeric Unicode characters."
        control0,       control1,       TakeWhile1,     is_control,         "control Unicode characters."
        digit0,         digit1,         Digit,          is_numeric,         "numeric Unicode characters."
        ascii0,         ascii1,         TakeWhile1,     is_ascii,           "ASCII characters."
    }
}

// STREAMING

/// Nom streaming parsing API functions.
pub mod streaming {
    use super::*;
    use nom::{IResult, InputTakeAtPosition};
    use nom::error::{ErrorKind, ParseError};

    // Dynamically generate both the zero and 1 parse APIs.
    macro_rules! parse_impl {
        ($($name0:ident, $name1:ident, $kind:ident, $callback:ident, $comment:expr)*) => ($(
            doc!(concat!("Recognizes zero or more ", $comment),
                #[inline]
                pub fn $name0<T, Error>(input: T)
                    -> IResult<T, T, Error>
                    where T: InputTakeAtPosition,
                          <T as InputTakeAtPosition>::Item: IsChar,
                          Error: ParseError<T>
                {
                  input.split_at_position(|item| !$callback(item))
                }
            );

            doc!(concat!("Recognizes one or more ", $comment),
                #[inline]
                pub fn $name1<T, Error>(input: T)
                    -> IResult<T, T, Error>
                    where T: InputTakeAtPosition,
                          <T as InputTakeAtPosition>::Item: IsChar,
                          Error: ParseError<T>
                {
                  input.split_at_position1(|item| !$callback(item), ErrorKind::$kind)
                }
            );
        )*);
    }

    parse_impl! {
        alpha0,         alpha1,         Alpha,          is_alphabetic,      "lowercase and uppercase alphabetic Unicode characters."
        lower0,         lower1,         Alpha,          is_lowercase,       "lowercase alphabetic Unicode characters."
        upper0,         upper1,         Alpha,          is_uppercase,       "lowercase alphabetic Unicode characters."
        space0,         space1,         Space,          is_whitespace,      "whitespace Unicode characters."
        alphanumeric0,  alphanumeric1,  AlphaNumeric,   is_alphanumeric,    "alphabetic and numeric Unicode characters."
        control0,       control1,       TakeWhile1,     is_control,         "control Unicode characters."
        digit0,         digit1,         Digit,          is_numeric,         "numeric Unicode characters."
        ascii0,         ascii1,         TakeWhile1,     is_ascii,           "ASCII characters."
    }
}

// TESTS
// -----

#[cfg(test)]
mod tests {
    use nom::{IResult, InputTakeAtPosition, AsChar};
    use nom::error::ErrorKind;
    use nom::Err::{Error, Incomplete};
    use nom::Needed::Size;
    use super::*;

    /// Call data for simplified testing (removes the error parameter).
    fn call<T, F>(f: F, input: T)
        -> IResult<T, T>
        where T: InputTakeAtPosition,
              <T as InputTakeAtPosition>::Item: AsChar,
              F: Fn(T) -> IResult<T, T>
    {
        f(input)
    }

    fn run_tests<'a, F>(f: &F, tests: &[(&'a str, IResult<&'a str, &'a str>)])
        where F: Fn(&'a str) -> IResult<&'a str, &'a str>
    {
        for test in tests.iter() {
            assert_eq!(call(f, test.0), test.1);
        }
    }

    // COMPLETE

    #[test]
    fn alpha0_complete_test() {
        run_tests(&complete::alpha0, &[
            ("latin", Ok(("", "latin"))),
            ("latin123", Ok(("123", "latin"))),
            ("LATIN", Ok(("", "LATIN"))),
            ("LATIN123", Ok(("123", "LATIN"))),
            ("123", Ok(("123", ""))),
            ("erfüllen", Ok(("", "erfüllen"))),
            ("erfüllen123", Ok(("123", "erfüllen"))),
            ("조선글", Ok(("", "조선글"))),
            ("조선글123", Ok(("123", "조선글"))),
            (" \t\n\x08", Ok((" \t\n\x08", ""))),
            ("\u{200b}", Ok(("\u{200b}", ""))),
            ("\u{1680}\u{200a}\u{2028}\u{202f}\u{205f}\u{3000}\u{200b}", Ok(("\u{1680}\u{200a}\u{2028}\u{202f}\u{205f}\u{3000}\u{200b}", ""))),
            ("\x00\x01\x02\u{80}", Ok(("\x00\x01\x02\u{80}", ""))),
            ("\u{94}\u{100}", Ok(("\u{94}\u{100}", ""))),
            ("", Ok(("", "")))
        ]);
    }

    #[test]
    fn alpha1_complete_test() {
        run_tests(&complete::alpha1, &[
            ("latin", Ok(("", "latin"))),
            ("latin123", Ok(("123", "latin"))),
            ("LATIN", Ok(("", "LATIN"))),
            ("LATIN123", Ok(("123", "LATIN"))),
            ("123", Err(Error(("123", ErrorKind::Alpha)))),
            ("erfüllen", Ok(("", "erfüllen"))),
            ("erfüllen123", Ok(("123", "erfüllen"))),
            ("조선글", Ok(("", "조선글"))),
            ("조선글123", Ok(("123", "조선글"))),
            (" \t\n\x08", Err(Error((" \t\n\x08", ErrorKind::Alpha)))),
            ("\u{200b}", Err(Error(("\u{200b}", ErrorKind::Alpha)))),
            ("\u{1680}\u{200a}\u{2028}\u{202f}\u{205f}\u{3000}\u{200b}", Err(Error(("\u{1680}\u{200a}\u{2028}\u{202f}\u{205f}\u{3000}\u{200b}", ErrorKind::Alpha)))),
            ("\x00\x01\x02\u{80}", Err(Error(("\x00\x01\x02\u{80}", ErrorKind::Alpha)))),
            ("\u{94}\u{100}", Err(Error(("\u{94}\u{100}", ErrorKind::Alpha)))),
            ("", Err(Error(("", ErrorKind::Alpha))))
        ]);
    }

    #[test]
    fn lower0_complete_test() {
        run_tests(&complete::lower0, &[
            ("latin", Ok(("", "latin"))),
            ("latin123", Ok(("123", "latin"))),
            ("LATIN", Ok(("LATIN", ""))),
            ("LATIN123", Ok(("LATIN123", ""))),
            ("123", Ok(("123", ""))),
            ("erfüllen", Ok(("", "erfüllen"))),
            ("erfüllen123", Ok(("123", "erfüllen"))),
            ("조선글", Ok(("조선글", ""))),
            ("조선글123", Ok(("조선글123", ""))),
            (" \t\n\x08", Ok((" \t\n\x08", ""))),
            ("\u{200b}", Ok(("\u{200b}", ""))),
            ("\u{1680}\u{200a}\u{2028}\u{202f}\u{205f}\u{3000}\u{200b}", Ok(("\u{1680}\u{200a}\u{2028}\u{202f}\u{205f}\u{3000}\u{200b}", ""))),
            ("\x00\x01\x02\u{80}", Ok(("\x00\x01\x02\u{80}", ""))),
            ("\u{94}\u{100}", Ok(("\u{94}\u{100}", ""))),
            ("", Ok(("", "")))
        ]);
    }

    #[test]
    fn lower1_complete_test() {
        run_tests(&complete::lower1, &[
            ("latin", Ok(("", "latin"))),
            ("latin123", Ok(("123", "latin"))),
            ("LATIN", Err(Error(("LATIN", ErrorKind::Alpha)))),
            ("LATIN123", Err(Error(("LATIN123", ErrorKind::Alpha)))),
            ("123", Err(Error(("123", ErrorKind::Alpha)))),
            ("erfüllen", Ok(("", "erfüllen"))),
            ("erfüllen123", Ok(("123", "erfüllen"))),
            ("조선글", Err(Error(("조선글", ErrorKind::Alpha)))),
            ("조선글123", Err(Error(("조선글123", ErrorKind::Alpha)))),
            (" \t\n\x08", Err(Error((" \t\n\x08", ErrorKind::Alpha)))),
            ("\u{200b}", Err(Error(("\u{200b}", ErrorKind::Alpha)))),
            ("\u{1680}\u{200a}\u{2028}\u{202f}\u{205f}\u{3000}\u{200b}", Err(Error(("\u{1680}\u{200a}\u{2028}\u{202f}\u{205f}\u{3000}\u{200b}", ErrorKind::Alpha)))),
            ("\x00\x01\x02\u{80}", Err(Error(("\x00\x01\x02\u{80}", ErrorKind::Alpha)))),
            ("\u{94}\u{100}", Err(Error(("\u{94}\u{100}", ErrorKind::Alpha)))),
            ("", Err(Error(("", ErrorKind::Alpha))))
        ]);
    }

    #[test]
    fn upper0_complete_test() {
        run_tests(&complete::upper0, &[
            ("latin", Ok(("latin", ""))),
            ("latin123", Ok(("latin123", ""))),
            ("LATIN", Ok(("", "LATIN"))),
            ("LATIN123", Ok(("123", "LATIN"))),
            ("123", Ok(("123", ""))),
            ("erfüllen", Ok(("erfüllen", ""))),
            ("erfüllen123", Ok(("erfüllen123", ""))),
            ("조선글", Ok(("조선글", ""))),
            ("조선글123", Ok(("조선글123", ""))),
            (" \t\n\x08", Ok((" \t\n\x08", ""))),
            ("\u{200b}", Ok(("\u{200b}", ""))),
            ("\u{1680}\u{200a}\u{2028}\u{202f}\u{205f}\u{3000}\u{200b}", Ok(("\u{1680}\u{200a}\u{2028}\u{202f}\u{205f}\u{3000}\u{200b}", ""))),
            ("\x00\x01\x02\u{80}", Ok(("\x00\x01\x02\u{80}", ""))),
            ("\u{94}\u{100}", Ok(("\u{94}\u{100}", ""))),
            ("", Ok(("", "")))
        ]);
    }

    #[test]
    fn upper1_complete_test() {
        run_tests(&complete::upper1, &[
            ("latin", Err(Error(("latin", ErrorKind::Alpha)))),
            ("latin123", Err(Error(("latin123", ErrorKind::Alpha)))),
            ("LATIN", Ok(("", "LATIN"))),
            ("LATIN123", Ok(("123", "LATIN"))),
            ("123", Err(Error(("123", ErrorKind::Alpha)))),
            ("erfüllen", Err(Error(("erfüllen", ErrorKind::Alpha)))),
            ("erfüllen123", Err(Error(("erfüllen123", ErrorKind::Alpha)))),
            ("조선글", Err(Error(("조선글", ErrorKind::Alpha)))),
            ("조선글123", Err(Error(("조선글123", ErrorKind::Alpha)))),
            (" \t\n\x08", Err(Error((" \t\n\x08", ErrorKind::Alpha)))),
            ("\u{200b}", Err(Error(("\u{200b}", ErrorKind::Alpha)))),
            ("\u{1680}\u{200a}\u{2028}\u{202f}\u{205f}\u{3000}\u{200b}", Err(Error(("\u{1680}\u{200a}\u{2028}\u{202f}\u{205f}\u{3000}\u{200b}", ErrorKind::Alpha)))),
            ("\x00\x01\x02\u{80}", Err(Error(("\x00\x01\x02\u{80}", ErrorKind::Alpha)))),
            ("\u{94}\u{100}", Err(Error(("\u{94}\u{100}", ErrorKind::Alpha)))),
            ("", Err(Error(("", ErrorKind::Alpha))))
        ]);
    }

    #[test]
    fn space0_complete_test() {
        run_tests(&complete::space0, &[
            ("latin", Ok(("latin", ""))),
            ("latin123", Ok(("latin123", ""))),
            ("LATIN", Ok(("LATIN", ""))),
            ("LATIN123", Ok(("LATIN123", ""))),
            ("123", Ok(("123", ""))),
            ("erfüllen", Ok(("erfüllen", ""))),
            ("erfüllen123", Ok(("erfüllen123", ""))),
            ("조선글", Ok(("조선글", ""))),
            ("조선글123", Ok(("조선글123", ""))),
            (" \t\n\x08", Ok(("\x08", " \t\n"))),
            ("\u{200b}", Ok(("\u{200b}", ""))),
            ("\u{1680}\u{200a}\u{2028}\u{202f}\u{205f}\u{3000}\u{200b}", Ok(("\u{200b}", "\u{1680}\u{200a}\u{2028}\u{202f}\u{205f}\u{3000}"))),
            ("\x00\x01\x02\u{80}", Ok(("\x00\x01\x02\u{80}", ""))),
            ("\u{94}\u{100}", Ok(("\u{94}\u{100}", ""))),
            ("", Ok(("", "")))
        ]);
    }

    #[test]
    fn space1_complete_test() {
        run_tests(&complete::space1, &[
            ("latin", Err(Error(("latin", ErrorKind::Space)))),
            ("latin123", Err(Error(("latin123", ErrorKind::Space)))),
            ("LATIN", Err(Error(("LATIN", ErrorKind::Space)))),
            ("LATIN123", Err(Error(("LATIN123", ErrorKind::Space)))),
            ("123", Err(Error(("123", ErrorKind::Space)))),
            ("erfüllen", Err(Error(("erfüllen", ErrorKind::Space)))),
            ("erfüllen123", Err(Error(("erfüllen123", ErrorKind::Space)))),
            ("조선글", Err(Error(("조선글", ErrorKind::Space)))),
            ("조선글123", Err(Error(("조선글123", ErrorKind::Space)))),
            (" \t\n\x08", Ok(("\x08", " \t\n"))),
            ("\u{200b}", Err(Error(("\u{200b}", ErrorKind::Space)))),
            ("\u{1680}\u{200a}\u{2028}\u{202f}\u{205f}\u{3000}\u{200b}", Ok(("\u{200b}", "\u{1680}\u{200a}\u{2028}\u{202f}\u{205f}\u{3000}"))),
            ("\x00\x01\x02\u{80}", Err(Error(("\x00\x01\x02\u{80}", ErrorKind::Space)))),
            ("\u{94}\u{100}", Err(Error(("\u{94}\u{100}", ErrorKind::Space)))),
            ("", Err(Error(("", ErrorKind::Space))))
        ]);
    }

    #[test]
    fn alphanumeric0_complete_test() {
        run_tests(&complete::alphanumeric0, &[
            ("latin", Ok(("", "latin"))),
            ("latin123", Ok(("", "latin123"))),
            ("LATIN", Ok(("", "LATIN"))),
            ("LATIN123", Ok(("", "LATIN123"))),
            ("123", Ok(("", "123"))),
            ("erfüllen", Ok(("", "erfüllen"))),
            ("erfüllen123", Ok(("", "erfüllen123"))),
            ("조선글", Ok(("", "조선글"))),
            ("조선글123", Ok(("", "조선글123"))),
            (" \t\n\x08", Ok((" \t\n\x08", ""))),
            ("\u{200b}", Ok(("\u{200b}", ""))),
            ("\u{1680}\u{200a}\u{2028}\u{202f}\u{205f}\u{3000}\u{200b}", Ok(("\u{1680}\u{200a}\u{2028}\u{202f}\u{205f}\u{3000}\u{200b}", ""))),
            ("\x00\x01\x02\u{80}", Ok(("\x00\x01\x02\u{80}", ""))),
            ("\u{94}\u{100}", Ok(("\u{94}\u{100}", ""))),
            ("", Ok(("", "")))
        ]);
    }

    #[test]
    fn alphanumeric1_complete_test() {
        run_tests(&complete::alphanumeric1, &[
            ("latin", Ok(("", "latin"))),
            ("latin123", Ok(("", "latin123"))),
            ("LATIN", Ok(("", "LATIN"))),
            ("LATIN123", Ok(("", "LATIN123"))),
            ("123", Ok(("", "123"))),
            ("erfüllen", Ok(("", "erfüllen"))),
            ("erfüllen123", Ok(("", "erfüllen123"))),
            ("조선글", Ok(("", "조선글"))),
            ("조선글123", Ok(("", "조선글123"))),
            (" \t\n\x08", Err(Error((" \t\n\x08", ErrorKind::AlphaNumeric)))),
            ("\u{200b}", Err(Error(("\u{200b}", ErrorKind::AlphaNumeric)))),
            ("\u{1680}\u{200a}\u{2028}\u{202f}\u{205f}\u{3000}\u{200b}", Err(Error(("\u{1680}\u{200a}\u{2028}\u{202f}\u{205f}\u{3000}\u{200b}", ErrorKind::AlphaNumeric)))),
            ("\x00\x01\x02\u{80}", Err(Error(("\x00\x01\x02\u{80}", ErrorKind::AlphaNumeric)))),
            ("\u{94}\u{100}", Err(Error(("\u{94}\u{100}", ErrorKind::AlphaNumeric)))),
            ("", Err(Error(("", ErrorKind::AlphaNumeric))))
        ]);
    }

    #[test]
    fn control0_complete_test() {
        run_tests(&complete::control0, &[
            ("latin", Ok(("latin", ""))),
            ("latin123", Ok(("latin123", ""))),
            ("LATIN", Ok(("LATIN", ""))),
            ("LATIN123", Ok(("LATIN123", ""))),
            ("123", Ok(("123", ""))),
            ("erfüllen", Ok(("erfüllen", ""))),
            ("erfüllen123", Ok(("erfüllen123", ""))),
            ("조선글", Ok(("조선글", ""))),
            ("조선글123", Ok(("조선글123", ""))),
            (" \t\n\x08", Ok((" \t\n\x08", ""))),
            ("\u{200b}", Ok(("\u{200b}", ""))),
            ("\u{1680}\u{200a}\u{2028}\u{202f}\u{205f}\u{3000}\u{200b}", Ok(("\u{1680}\u{200a}\u{2028}\u{202f}\u{205f}\u{3000}\u{200b}", ""))),
            ("\x00\x01\x02\u{80}", Ok(("", "\x00\x01\x02\u{80}"))),
            ("\u{94}\u{100}", Ok(("\u{100}", "\u{94}"))),
            ("", Ok(("", "")))
        ]);
    }

    #[test]
    fn control1_complete_test() {
        run_tests(&complete::control1, &[
            ("latin", Err(Error(("latin", ErrorKind::TakeWhile1)))),
            ("latin123", Err(Error(("latin123", ErrorKind::TakeWhile1)))),
            ("LATIN", Err(Error(("LATIN", ErrorKind::TakeWhile1)))),
            ("LATIN123", Err(Error(("LATIN123", ErrorKind::TakeWhile1)))),
            ("123", Err(Error(("123", ErrorKind::TakeWhile1)))),
            ("erfüllen", Err(Error(("erfüllen", ErrorKind::TakeWhile1)))),
            ("erfüllen123", Err(Error(("erfüllen123", ErrorKind::TakeWhile1)))),
            ("조선글", Err(Error(("조선글", ErrorKind::TakeWhile1)))),
            ("조선글123", Err(Error(("조선글123", ErrorKind::TakeWhile1)))),
            (" \t\n\x08", Err(Error((" \t\n\x08", ErrorKind::TakeWhile1)))),
            ("\u{200b}", Err(Error(("\u{200b}", ErrorKind::TakeWhile1)))),
            ("\u{1680}\u{200a}\u{2028}\u{202f}\u{205f}\u{3000}\u{200b}", Err(Error(("\u{1680}\u{200a}\u{2028}\u{202f}\u{205f}\u{3000}\u{200b}", ErrorKind::TakeWhile1)))),
            ("\x00\x01\x02\u{80}", Ok(("", "\x00\x01\x02\u{80}"))),
            ("\u{94}\u{100}", Ok(("\u{100}", "\u{94}"))),
            ("", Err(Error(("", ErrorKind::TakeWhile1))))
        ]);
    }

    #[test]
    fn digit0_complete_test() {
        run_tests(&complete::digit0, &[
            ("latin", Ok(("latin", ""))),
            ("latin123", Ok(("latin123", ""))),
            ("LATIN", Ok(("LATIN", ""))),
            ("LATIN123", Ok(("LATIN123", ""))),
            ("123", Ok(("", "123"))),
            ("erfüllen", Ok(("erfüllen", ""))),
            ("erfüllen123", Ok(("erfüllen123", ""))),
            ("조선글", Ok(("조선글", ""))),
            ("조선글123", Ok(("조선글123", ""))),
            (" \t\n\x08", Ok((" \t\n\x08", ""))),
            ("\u{200b}", Ok(("\u{200b}", ""))),
            ("\u{1680}\u{200a}\u{2028}\u{202f}\u{205f}\u{3000}\u{200b}", Ok(("\u{1680}\u{200a}\u{2028}\u{202f}\u{205f}\u{3000}\u{200b}", ""))),
            ("\x00\x01\x02\u{80}", Ok(("\x00\x01\x02\u{80}", ""))),
            ("\u{94}\u{100}", Ok(("\u{94}\u{100}", ""))),
            ("", Ok(("", "")))
        ]);
    }

    #[test]
    fn digit1_complete_test() {
        run_tests(&complete::digit1, &[
            ("latin", Err(Error(("latin", ErrorKind::Digit)))),
            ("latin123", Err(Error(("latin123", ErrorKind::Digit)))),
            ("LATIN", Err(Error(("LATIN", ErrorKind::Digit)))),
            ("LATIN123", Err(Error(("LATIN123", ErrorKind::Digit)))),
            ("123", Ok(("", "123"))),
            ("erfüllen", Err(Error(("erfüllen", ErrorKind::Digit)))),
            ("erfüllen123", Err(Error(("erfüllen123", ErrorKind::Digit)))),
            ("조선글", Err(Error(("조선글", ErrorKind::Digit)))),
            ("조선글123", Err(Error(("조선글123", ErrorKind::Digit)))),
            (" \t\n\x08", Err(Error((" \t\n\x08", ErrorKind::Digit)))),
            ("\u{200b}", Err(Error(("\u{200b}", ErrorKind::Digit)))),
            ("\u{1680}\u{200a}\u{2028}\u{202f}\u{205f}\u{3000}\u{200b}", Err(Error(("\u{1680}\u{200a}\u{2028}\u{202f}\u{205f}\u{3000}\u{200b}", ErrorKind::Digit)))),
            ("\x00\x01\x02\u{80}", Err(Error(("\x00\x01\x02\u{80}", ErrorKind::Digit)))),
            ("\u{94}\u{100}", Err(Error(("\u{94}\u{100}", ErrorKind::Digit)))),
            ("", Err(Error(("", ErrorKind::Digit))))
        ]);
    }

    #[test]
    fn ascii0_complete_test() {
        run_tests(&complete::ascii0, &[
            ("latin", Ok(("", "latin"))),
            ("latin123", Ok(("", "latin123"))),
            ("LATIN", Ok(("", "LATIN"))),
            ("LATIN123", Ok(("", "LATIN123"))),
            ("123", Ok(("", "123"))),
            ("erfüllen", Ok(("üllen", "erf"))),
            ("erfüllen123", Ok(("üllen123", "erf"))),
            ("조선글", Ok(("조선글", ""))),
            ("조선글123", Ok(("조선글123", ""))),
            (" \t\n\x08", Ok(("", " \t\n\x08"))),
            ("\u{200b}", Ok(("\u{200b}", ""))),
            ("\u{1680}\u{200a}\u{2028}\u{202f}\u{205f}\u{3000}\u{200b}", Ok(("\u{1680}\u{200a}\u{2028}\u{202f}\u{205f}\u{3000}\u{200b}", ""))),
            ("\x00\x01\x02\u{80}", Ok(("\u{80}", "\x00\x01\x02"))),
            ("\u{94}\u{100}", Ok(("\u{94}\u{100}", ""))),
            ("", Ok(("", "")))
        ]);
    }

    #[test]
    fn ascii1_complete_test() {
        run_tests(&complete::ascii1, &[
            ("latin", Ok(("", "latin"))),
            ("latin123", Ok(("", "latin123"))),
            ("LATIN", Ok(("", "LATIN"))),
            ("LATIN123", Ok(("", "LATIN123"))),
            ("123", Ok(("", "123"))),
            ("erfüllen", Ok(("üllen", "erf"))),
            ("erfüllen123", Ok(("üllen123", "erf"))),
            ("조선글", Err(Error(("조선글", ErrorKind::TakeWhile1)))),
            ("조선글123", Err(Error(("조선글123", ErrorKind::TakeWhile1)))),
            (" \t\n\x08", Ok(("", " \t\n\x08"))),
            ("\u{200b}", Err(Error(("\u{200b}", ErrorKind::TakeWhile1)))),
            ("\u{1680}\u{200a}\u{2028}\u{202f}\u{205f}\u{3000}\u{200b}", Err(Error(("\u{1680}\u{200a}\u{2028}\u{202f}\u{205f}\u{3000}\u{200b}", ErrorKind::TakeWhile1)))),
            ("\x00\x01\x02\u{80}", Ok(("\u{80}", "\x00\x01\x02"))),
            ("\u{94}\u{100}", Err(Error(("\u{94}\u{100}", ErrorKind::TakeWhile1)))),
            ("", Err(Error(("", ErrorKind::TakeWhile1))))
        ]);
    }

    // STREAMING

    #[test]
    fn alpha0_streaming_test() {
        run_tests(&streaming::alpha0, &[
            ("latin", Err(Incomplete(Size(1)))),
            ("latin123", Ok(("123", "latin"))),
            ("LATIN", Err(Incomplete(Size(1)))),
            ("LATIN123", Ok(("123", "LATIN"))),
            ("123", Ok(("123", ""))),
            ("erfüllen", Err(Incomplete(Size(1)))),
            ("erfüllen123", Ok(("123", "erfüllen"))),
            ("조선글", Err(Incomplete(Size(1)))),
            ("조선글123", Ok(("123", "조선글"))),
            (" \t\n\x08", Ok((" \t\n\x08", ""))),
            ("\u{200b}", Ok(("\u{200b}", ""))),
            ("\u{1680}\u{200a}\u{2028}\u{202f}\u{205f}\u{3000}\u{200b}", Ok(("\u{1680}\u{200a}\u{2028}\u{202f}\u{205f}\u{3000}\u{200b}", ""))),
            ("\x00\x01\x02\u{80}", Ok(("\x00\x01\x02\u{80}", ""))),
            ("\u{94}\u{100}", Ok(("\u{94}\u{100}", ""))),
            ("", Err(Incomplete(Size(1))))
        ]);
    }

    #[test]
    fn alpha1_streaming_test() {
        run_tests(&streaming::alpha1, &[
            ("latin", Err(Incomplete(Size(1)))),
            ("latin123", Ok(("123", "latin"))),
            ("LATIN", Err(Incomplete(Size(1)))),
            ("LATIN123", Ok(("123", "LATIN"))),
            ("123", Err(Error(("123", ErrorKind::Alpha)))),
            ("erfüllen", Err(Incomplete(Size(1)))),
            ("erfüllen123", Ok(("123", "erfüllen"))),
            ("조선글", Err(Incomplete(Size(1)))),
            ("조선글123", Ok(("123", "조선글"))),
            (" \t\n\x08", Err(Error((" \t\n\x08", ErrorKind::Alpha)))),
            ("\u{200b}", Err(Error(("\u{200b}", ErrorKind::Alpha)))),
            ("\u{1680}\u{200a}\u{2028}\u{202f}\u{205f}\u{3000}\u{200b}", Err(Error(("\u{1680}\u{200a}\u{2028}\u{202f}\u{205f}\u{3000}\u{200b}", ErrorKind::Alpha)))),
            ("\x00\x01\x02\u{80}", Err(Error(("\x00\x01\x02\u{80}", ErrorKind::Alpha)))),
            ("\u{94}\u{100}", Err(Error(("\u{94}\u{100}", ErrorKind::Alpha)))),
            ("", Err(Incomplete(Size(1))))

        ]);
    }

    #[test]
    fn lower0_streaming_test() {
        run_tests(&streaming::lower0, &[
            ("latin", Err(Incomplete(Size(1)))),
            ("latin123", Ok(("123", "latin"))),
            ("LATIN", Ok(("LATIN", ""))),
            ("LATIN123", Ok(("LATIN123", ""))),
            ("123", Ok(("123", ""))),
            ("erfüllen", Err(Incomplete(Size(1)))),
            ("erfüllen123", Ok(("123", "erfüllen"))),
            ("조선글", Ok(("조선글", ""))),
            ("조선글123", Ok(("조선글123", ""))),
            (" \t\n\x08", Ok((" \t\n\x08", ""))),
            ("\u{200b}", Ok(("\u{200b}", ""))),
            ("\u{1680}\u{200a}\u{2028}\u{202f}\u{205f}\u{3000}\u{200b}", Ok(("\u{1680}\u{200a}\u{2028}\u{202f}\u{205f}\u{3000}\u{200b}", ""))),
            ("\x00\x01\x02\u{80}", Ok(("\x00\x01\x02\u{80}", ""))),
            ("\u{94}\u{100}", Ok(("\u{94}\u{100}", ""))),
            ("", Err(Incomplete(Size(1))))
        ]);
    }

    #[test]
    fn lower1_streaming_test() {
        run_tests(&streaming::lower1, &[
            ("latin", Err(Incomplete(Size(1)))),
            ("latin123", Ok(("123", "latin"))),
            ("LATIN", Err(Error(("LATIN", ErrorKind::Alpha)))),
            ("LATIN123", Err(Error(("LATIN123", ErrorKind::Alpha)))),
            ("123", Err(Error(("123", ErrorKind::Alpha)))),
            ("erfüllen", Err(Incomplete(Size(1)))),
            ("erfüllen123", Ok(("123", "erfüllen"))),
            ("조선글", Err(Error(("조선글", ErrorKind::Alpha)))),
            ("조선글123", Err(Error(("조선글123", ErrorKind::Alpha)))),
            (" \t\n\x08", Err(Error((" \t\n\x08", ErrorKind::Alpha)))),
            ("\u{200b}", Err(Error(("\u{200b}", ErrorKind::Alpha)))),
            ("\u{1680}\u{200a}\u{2028}\u{202f}\u{205f}\u{3000}\u{200b}", Err(Error(("\u{1680}\u{200a}\u{2028}\u{202f}\u{205f}\u{3000}\u{200b}", ErrorKind::Alpha)))),
            ("\x00\x01\x02\u{80}", Err(Error(("\x00\x01\x02\u{80}", ErrorKind::Alpha)))),
            ("\u{94}\u{100}", Err(Error(("\u{94}\u{100}", ErrorKind::Alpha)))),
            ("", Err(Incomplete(Size(1))))
        ]);
    }

    #[test]
    fn upper0_streaming_test() {
        run_tests(&streaming::upper0, &[
            ("latin", Ok(("latin", ""))),
            ("latin123", Ok(("latin123", ""))),
            ("LATIN", Err(Incomplete(Size(1)))),
            ("LATIN123", Ok(("123", "LATIN"))),
            ("123", Ok(("123", ""))),
            ("erfüllen", Ok(("erfüllen", ""))),
            ("erfüllen123", Ok(("erfüllen123", ""))),
            ("조선글", Ok(("조선글", ""))),
            ("조선글123", Ok(("조선글123", ""))),
            (" \t\n\x08", Ok((" \t\n\x08", ""))),
            ("\u{200b}", Ok(("\u{200b}", ""))),
            ("\u{1680}\u{200a}\u{2028}\u{202f}\u{205f}\u{3000}\u{200b}", Ok(("\u{1680}\u{200a}\u{2028}\u{202f}\u{205f}\u{3000}\u{200b}", ""))),
            ("\x00\x01\x02\u{80}", Ok(("\x00\x01\x02\u{80}", ""))),
            ("\u{94}\u{100}", Ok(("\u{94}\u{100}", ""))),
            ("", Err(Incomplete(Size(1))))
        ]);
    }

    #[test]
    fn upper1_streaming_test() {
        run_tests(&streaming::upper1, &[
            ("latin", Err(Error(("latin", ErrorKind::Alpha)))),
            ("latin123", Err(Error(("latin123", ErrorKind::Alpha)))),
            ("LATIN", Err(Incomplete(Size(1)))),
            ("LATIN123", Ok(("123", "LATIN"))),
            ("123", Err(Error(("123", ErrorKind::Alpha)))),
            ("erfüllen", Err(Error(("erfüllen", ErrorKind::Alpha)))),
            ("erfüllen123", Err(Error(("erfüllen123", ErrorKind::Alpha)))),
            ("조선글", Err(Error(("조선글", ErrorKind::Alpha)))),
            ("조선글123", Err(Error(("조선글123", ErrorKind::Alpha)))),
            (" \t\n\x08", Err(Error((" \t\n\x08", ErrorKind::Alpha)))),
            ("\u{200b}", Err(Error(("\u{200b}", ErrorKind::Alpha)))),
            ("\u{1680}\u{200a}\u{2028}\u{202f}\u{205f}\u{3000}\u{200b}", Err(Error(("\u{1680}\u{200a}\u{2028}\u{202f}\u{205f}\u{3000}\u{200b}", ErrorKind::Alpha)))),
            ("\x00\x01\x02\u{80}", Err(Error(("\x00\x01\x02\u{80}", ErrorKind::Alpha)))),
            ("\u{94}\u{100}", Err(Error(("\u{94}\u{100}", ErrorKind::Alpha)))),
            ("", Err(Incomplete(Size(1))))
        ]);
    }

    #[test]
    fn space0_streaming_test() {
        run_tests(&streaming::space0, &[
            ("latin", Ok(("latin", ""))),
            ("latin123", Ok(("latin123", ""))),
            ("LATIN", Ok(("LATIN", ""))),
            ("LATIN123", Ok(("LATIN123", ""))),
            ("123", Ok(("123", ""))),
            ("erfüllen", Ok(("erfüllen", ""))),
            ("erfüllen123", Ok(("erfüllen123", ""))),
            ("조선글", Ok(("조선글", ""))),
            ("조선글123", Ok(("조선글123", ""))),
            (" \t\n\x08", Ok(("\x08", " \t\n"))),
            ("\u{200b}", Ok(("\u{200b}", ""))),
            ("\u{1680}\u{200a}\u{2028}\u{202f}\u{205f}\u{3000}\u{200b}", Ok(("\u{200b}", "\u{1680}\u{200a}\u{2028}\u{202f}\u{205f}\u{3000}"))),
            ("\x00\x01\x02\u{80}", Ok(("\x00\x01\x02\u{80}", ""))),
            ("\u{94}\u{100}", Ok(("\u{94}\u{100}", ""))),
            ("", Err(Incomplete(Size(1))))
        ]);
    }

    #[test]
    fn space1_streaming_test() {
        run_tests(&streaming::space1, &[
            ("latin", Err(Error(("latin", ErrorKind::Space)))),
            ("latin123", Err(Error(("latin123", ErrorKind::Space)))),
            ("LATIN", Err(Error(("LATIN", ErrorKind::Space)))),
            ("LATIN123", Err(Error(("LATIN123", ErrorKind::Space)))),
            ("123", Err(Error(("123", ErrorKind::Space)))),
            ("erfüllen", Err(Error(("erfüllen", ErrorKind::Space)))),
            ("erfüllen123", Err(Error(("erfüllen123", ErrorKind::Space)))),
            ("조선글", Err(Error(("조선글", ErrorKind::Space)))),
            ("조선글123", Err(Error(("조선글123", ErrorKind::Space)))),
            (" \t\n\x08", Ok(("\x08", " \t\n"))),
            ("\u{200b}", Err(Error(("\u{200b}", ErrorKind::Space)))),
            ("\u{1680}\u{200a}\u{2028}\u{202f}\u{205f}\u{3000}\u{200b}", Ok(("\u{200b}", "\u{1680}\u{200a}\u{2028}\u{202f}\u{205f}\u{3000}"))),
            ("\x00\x01\x02\u{80}", Err(Error(("\x00\x01\x02\u{80}", ErrorKind::Space)))),
            ("\u{94}\u{100}", Err(Error(("\u{94}\u{100}", ErrorKind::Space)))),
            ("", Err(Incomplete(Size(1))))
        ]);
    }

    #[test]
    fn alphanumeric0_streaming_test() {
        run_tests(&streaming::alphanumeric0, &[
            ("latin", Err(Incomplete(Size(1)))),
            ("latin123", Err(Incomplete(Size(1)))),
            ("LATIN", Err(Incomplete(Size(1)))),
            ("LATIN123", Err(Incomplete(Size(1)))),
            ("123", Err(Incomplete(Size(1)))),
            ("erfüllen", Err(Incomplete(Size(1)))),
            ("erfüllen123", Err(Incomplete(Size(1)))),
            ("조선글", Err(Incomplete(Size(1)))),
            ("조선글123", Err(Incomplete(Size(1)))),
            (" \t\n\x08", Ok((" \t\n\x08", ""))),
            ("\u{200b}", Ok(("\u{200b}", ""))),
            ("\u{1680}\u{200a}\u{2028}\u{202f}\u{205f}\u{3000}\u{200b}", Ok(("\u{1680}\u{200a}\u{2028}\u{202f}\u{205f}\u{3000}\u{200b}", ""))),
            ("\x00\x01\x02\u{80}", Ok(("\x00\x01\x02\u{80}", ""))),
            ("\u{94}\u{100}", Ok(("\u{94}\u{100}", ""))),
            ("", Err(Incomplete(Size(1))))
        ]);
    }

    #[test]
    fn alphanumeric1_streaming_test() {
        run_tests(&streaming::alphanumeric1, &[
            ("latin", Err(Incomplete(Size(1)))),
            ("latin123", Err(Incomplete(Size(1)))),
            ("LATIN", Err(Incomplete(Size(1)))),
            ("LATIN123", Err(Incomplete(Size(1)))),
            ("123", Err(Incomplete(Size(1)))),
            ("erfüllen", Err(Incomplete(Size(1)))),
            ("erfüllen123", Err(Incomplete(Size(1)))),
            ("조선글", Err(Incomplete(Size(1)))),
            ("조선글123", Err(Incomplete(Size(1)))),
            (" \t\n\x08", Err(Error((" \t\n\x08", ErrorKind::AlphaNumeric)))),
            ("\u{200b}", Err(Error(("\u{200b}", ErrorKind::AlphaNumeric)))),
            ("\u{1680}\u{200a}\u{2028}\u{202f}\u{205f}\u{3000}\u{200b}", Err(Error(("\u{1680}\u{200a}\u{2028}\u{202f}\u{205f}\u{3000}\u{200b}", ErrorKind::AlphaNumeric)))),
            ("\x00\x01\x02\u{80}", Err(Error(("\x00\x01\x02\u{80}", ErrorKind::AlphaNumeric)))),
            ("\u{94}\u{100}", Err(Error(("\u{94}\u{100}", ErrorKind::AlphaNumeric)))),
            ("", Err(Incomplete(Size(1))))

        ]);
    }

    #[test]
    fn control0_streaming_test() {
        run_tests(&streaming::control0, &[
            ("latin", Ok(("latin", ""))),
            ("latin123", Ok(("latin123", ""))),
            ("LATIN", Ok(("LATIN", ""))),
            ("LATIN123", Ok(("LATIN123", ""))),
            ("123", Ok(("123", ""))),
            ("erfüllen", Ok(("erfüllen", ""))),
            ("erfüllen123", Ok(("erfüllen123", ""))),
            ("조선글", Ok(("조선글", ""))),
            ("조선글123", Ok(("조선글123", ""))),
            (" \t\n\x08", Ok((" \t\n\x08", ""))),
            ("\u{200b}", Ok(("\u{200b}", ""))),
            ("\u{1680}\u{200a}\u{2028}\u{202f}\u{205f}\u{3000}\u{200b}", Ok(("\u{1680}\u{200a}\u{2028}\u{202f}\u{205f}\u{3000}\u{200b}", ""))),
            ("\x00\x01\x02\u{80}", Err(Incomplete(Size(1)))),
            ("\u{94}\u{100}", Ok(("\u{100}", "\u{94}"))),
            ("", Err(Incomplete(Size(1))))
        ]);
    }

    #[test]
    fn control1_streaming_test() {
        run_tests(&streaming::control1, &[
            ("latin", Err(Error(("latin", ErrorKind::TakeWhile1)))),
            ("latin123", Err(Error(("latin123", ErrorKind::TakeWhile1)))),
            ("LATIN", Err(Error(("LATIN", ErrorKind::TakeWhile1)))),
            ("LATIN123", Err(Error(("LATIN123", ErrorKind::TakeWhile1)))),
            ("123", Err(Error(("123", ErrorKind::TakeWhile1)))),
            ("erfüllen", Err(Error(("erfüllen", ErrorKind::TakeWhile1)))),
            ("erfüllen123", Err(Error(("erfüllen123", ErrorKind::TakeWhile1)))),
            ("조선글", Err(Error(("조선글", ErrorKind::TakeWhile1)))),
            ("조선글123", Err(Error(("조선글123", ErrorKind::TakeWhile1)))),
            (" \t\n\x08", Err(Error((" \t\n\x08", ErrorKind::TakeWhile1)))),
            ("\u{200b}", Err(Error(("\u{200b}", ErrorKind::TakeWhile1)))),
            ("\u{1680}\u{200a}\u{2028}\u{202f}\u{205f}\u{3000}\u{200b}", Err(Error(("\u{1680}\u{200a}\u{2028}\u{202f}\u{205f}\u{3000}\u{200b}", ErrorKind::TakeWhile1)))),
            ("\x00\x01\x02\u{80}", Err(Incomplete(Size(1)))),
            ("\u{94}\u{100}", Ok(("\u{100}", "\u{94}"))),
            ("", Err(Incomplete(Size(1))))
        ]);
    }

    #[test]
    fn digit0_streaming_test() {
        run_tests(&streaming::digit0, &[
            ("latin", Ok(("latin", ""))),
            ("latin123", Ok(("latin123", ""))),
            ("LATIN", Ok(("LATIN", ""))),
            ("LATIN123", Ok(("LATIN123", ""))),
            ("123", Err(Incomplete(Size(1)))),
            ("erfüllen", Ok(("erfüllen", ""))),
            ("erfüllen123", Ok(("erfüllen123", ""))),
            ("조선글", Ok(("조선글", ""))),
            ("조선글123", Ok(("조선글123", ""))),
            (" \t\n\x08", Ok((" \t\n\x08", ""))),
            ("\u{200b}", Ok(("\u{200b}", ""))),
            ("\u{1680}\u{200a}\u{2028}\u{202f}\u{205f}\u{3000}\u{200b}", Ok(("\u{1680}\u{200a}\u{2028}\u{202f}\u{205f}\u{3000}\u{200b}", ""))),
            ("\x00\x01\x02\u{80}", Ok(("\x00\x01\x02\u{80}", ""))),
            ("\u{94}\u{100}", Ok(("\u{94}\u{100}", ""))),
            ("", Err(Incomplete(Size(1))))
        ]);
    }

    #[test]
    fn digit1_streaming_test() {
        run_tests(&streaming::digit1, &[
            ("latin", Err(Error(("latin", ErrorKind::Digit)))),
            ("latin123", Err(Error(("latin123", ErrorKind::Digit)))),
            ("LATIN", Err(Error(("LATIN", ErrorKind::Digit)))),
            ("LATIN123", Err(Error(("LATIN123", ErrorKind::Digit)))),
            ("123", Err(Incomplete(Size(1)))),
            ("erfüllen", Err(Error(("erfüllen", ErrorKind::Digit)))),
            ("erfüllen123", Err(Error(("erfüllen123", ErrorKind::Digit)))),
            ("조선글", Err(Error(("조선글", ErrorKind::Digit)))),
            ("조선글123", Err(Error(("조선글123", ErrorKind::Digit)))),
            (" \t\n\x08", Err(Error((" \t\n\x08", ErrorKind::Digit)))),
            ("\u{200b}", Err(Error(("\u{200b}", ErrorKind::Digit)))),
            ("\u{1680}\u{200a}\u{2028}\u{202f}\u{205f}\u{3000}\u{200b}", Err(Error(("\u{1680}\u{200a}\u{2028}\u{202f}\u{205f}\u{3000}\u{200b}", ErrorKind::Digit)))),
            ("\x00\x01\x02\u{80}", Err(Error(("\x00\x01\x02\u{80}", ErrorKind::Digit)))),
            ("\u{94}\u{100}", Err(Error(("\u{94}\u{100}", ErrorKind::Digit)))),
            ("", Err(Incomplete(Size(1))))
        ]);
    }

    #[test]
    fn ascii0_streaming_test() {
        run_tests(&streaming::ascii0, &[
            ("latin", Err(Incomplete(Size(1)))),
            ("latin123", Err(Incomplete(Size(1)))),
            ("LATIN", Err(Incomplete(Size(1)))),
            ("LATIN123", Err(Incomplete(Size(1)))),
            ("123", Err(Incomplete(Size(1)))),
            ("erfüllen", Ok(("üllen", "erf"))),
            ("erfüllen123", Ok(("üllen123", "erf"))),
            ("조선글", Ok(("조선글", ""))),
            ("조선글123", Ok(("조선글123", ""))),
            (" \t\n\x08", Err(Incomplete(Size(1)))),
            ("\u{200b}", Ok(("\u{200b}", ""))),
            ("\u{1680}\u{200a}\u{2028}\u{202f}\u{205f}\u{3000}\u{200b}", Ok(("\u{1680}\u{200a}\u{2028}\u{202f}\u{205f}\u{3000}\u{200b}", ""))),
            ("\x00\x01\x02\u{80}", Ok(("\u{80}", "\x00\x01\x02"))),
            ("\u{94}\u{100}", Ok(("\u{94}\u{100}", ""))),
            ("", Err(Incomplete(Size(1))))
        ]);
    }

    #[test]
    fn ascii1_streaming_test() {
        run_tests(&streaming::ascii1, &[
            ("latin", Err(Incomplete(Size(1)))),
            ("latin123", Err(Incomplete(Size(1)))),
            ("LATIN", Err(Incomplete(Size(1)))),
            ("LATIN123", Err(Incomplete(Size(1)))),
            ("123", Err(Incomplete(Size(1)))),
            ("erfüllen", Ok(("üllen", "erf"))),
            ("erfüllen123", Ok(("üllen123", "erf"))),
            ("조선글", Err(Error(("조선글", ErrorKind::TakeWhile1)))),
            ("조선글123", Err(Error(("조선글123", ErrorKind::TakeWhile1)))),
            (" \t\n\x08", Err(Incomplete(Size(1)))),
            ("\u{200b}", Err(Error(("\u{200b}", ErrorKind::TakeWhile1)))),
            ("\u{1680}\u{200a}\u{2028}\u{202f}\u{205f}\u{3000}\u{200b}", Err(Error(("\u{1680}\u{200a}\u{2028}\u{202f}\u{205f}\u{3000}\u{200b}", ErrorKind::TakeWhile1)))),
            ("\x00\x01\x02\u{80}", Ok(("\u{80}", "\x00\x01\x02"))),
            ("\u{94}\u{100}", Err(Error(("\u{94}\u{100}", ErrorKind::TakeWhile1)))),
            ("", Err(Incomplete(Size(1))))
        ]);
    }
}
