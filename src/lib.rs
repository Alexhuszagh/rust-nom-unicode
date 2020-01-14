//! Adaptors to add Unicode-aware parsing to Nom.

#![allow(unused)]

use std::char as stdchar;

// HELPERS
// -------

/// Calculates the number of UTF-8 continuation bytes.
const UTF8_BYTES: [u8; 256] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,3,3,3,3,3,3,3,3,4,4,4,4,5,5,5,5];
/// Offset to adjust codepoint back based on number of bytes.
const UTF8_OFFSETS: [u32; 6] = [0x00000000,0x00003080,0x000E2080,0x03C82080,0xFA082080,0x82082080];

/// Determine if a single character is ASCII.
#[inline]
fn is_ascii_byte(byte: u8) -> bool {
    byte <= 0x7f
}

/// Determine if byte is a start byte.
fn is_start_byte(byte: u8) -> bool {
    byte > 0xbf
}

/// Determine if byte is a continuation byte.
fn is_continuation_byte(byte: u8) -> bool {
    byte >= 0x80 && byte <= 0xbf
}

/// Helper to add a start byte to the result.
macro_rules! add_start_byte {
    ($result:ident, $byte:expr) => {
        let byte = $byte;
        if !is_start_byte(byte) {
            return None
        }
        $result += byte as u32;
    };
    (@shr $result:ident, $byte:expr) => {
        add_start_byte!($result, $byte);
        $result <<= 6;
    };
}

/// Helper to add a continuation byte to the result.
macro_rules! add_continuation_byte {
    ($result:ident, $byte:expr) => {
        let byte = $byte;
        if !is_continuation_byte(byte) {
            return None
        }
        $result += byte as u32;
    };
    (@shr $result:ident, $byte:expr) => {
        add_continuation_byte!($result, $byte);
        $result <<= 6;
    };
}

/// Extract single Unicode character from byte slice.
///
/// Validates UTF-8 input, and returns None if the input
/// is invalid.
fn extract_character<'a>(bytes: &'a [u8])
    -> Option<(char, &'a [u8])>
{
    // Need to find the number of bytes, validate the UTF-8.
    let mut iter = bytes.iter();
    let first = *iter.next()?;
    let count = UTF8_BYTES[first as usize] as usize;

    // Create our character
    let code = match (count) {
        // Valid UTF-8.
        3 => {
            let mut result: u32 = 0;
            add_start_byte!(@shr result, first);
            add_continuation_byte!(@shr result, *iter.next()?);
            add_continuation_byte!(@shr result, *iter.next()?);
            add_continuation_byte!(result, *iter.next()?);
            result
        },
        2 => {
            let mut result: u32 = 0;
            add_start_byte!(@shr result, first);
            add_continuation_byte!(@shr result, *iter.next()?);
            add_continuation_byte!(result, *iter.next()?);
            result
        },
        1 => {
            let mut result: u32 = 0;
            add_start_byte!(@shr result, first);
            add_continuation_byte!(result, *iter.next()?);
            result
        },
        0 => {
            first as u32
        }
        // Invalid UTF-8, unapproved extensions.
        _ => return None,
    };

    // Create character and code point.
    unsafe {
        let character = stdchar::from_u32_unchecked(code - UTF8_OFFSETS[count]);
        let slc = iter.as_slice();
        Some((character, slc))
    }
}

// API

/// Determine if a character is alphabetical.
///
/// Returns if the first code point is an alphabetical
/// character and the number of bytes contributing
/// to the code point. If the data is invalid or empty,
/// will return None.
#[inline]
fn is_alphabetic<'a>(bytes: &'a [u8])
    -> Option<(bool, &'a [u8])>
{
    let (c, bytes) = extract_character(bytes)?;
    let is_alpha = c.is_alphabetic();
    Some((is_alpha, bytes))
}

// COMPLETE

pub mod complete {
    use super::is_alphabetic;
    use core::ops::{Range, RangeFrom, RangeTo};
    //use nom;

//    pub fn take_while<F, Input, Error: ParseError<Input>>(cond: F) -> impl Fn(Input) -> IResult<Input, Input, Error>
//    where
//      Input: InputTakeAtPosition,
//      F: Fn(<Input as InputTakeAtPosition>::Item) -> bool,
//    {
//      move |i: Input| i.split_at_position_complete(|c| !cond(c))
//    }

    pub fn alpha<'a, Error>(input: &'a [u8]) -> nom::IResult<&'a [u8], &'a [u8], Error>
    where
        Error: nom::error::ParseError<&'a [u8]>
    {
        // Continually process bytes.
        let mut bytes = input;
        while (bytes.len() > 0) {
            let result = match is_alphabetic(bytes) {
                None => panic!(""),
                //None => return Err(nom::internal::Err(Error::from_error_kind(input, nom::error::ErrorKind::Alpha))),
                Some(r) => r
            };
            if (result.0) {
                bytes = result.1;
            } else {
                break;
            }
        }

        // Now need to split into result.
        panic!("");
        // TODO(ahuszagh) Here...
    }
    // TODO(ahuszagh) here...
}

// INCOMPLETE

pub mod incomplete {
    // TODO(ahuszagh) here...
}

//fn take_while()

//#[inline]
//fn alpha(s: &[u8]) -> IResult<&[u8], &[u8]> {
//  take_while(is_alphabetic)(s)
//}

#[cfg(test)]
mod tests {
    use super::*;

    /// Wrap data for simplified testing.
    fn wrap<'a, F>(input: &'a [u8], f: F)
        -> Option<(bool, usize)>
        where F: Fn(&'a [u8]) -> Option<(bool, &'a [u8])>
    {
        let result = f(input)?;
        Some((result.0, input.len() - result.1.len()))
    }

    #[test]
    fn is_alphabetic_test() {
        // Test ASCII
        assert_eq!('a'.is_alphabetic(), true);
        assert_eq!(wrap(b"a", is_alphabetic), Some((true, 1)));

        // Test Hangul.
        assert_eq!('조'.is_alphabetic(), true);
        assert_eq!(wrap("조".as_bytes(), is_alphabetic), Some((true, 3)));
    }
}
