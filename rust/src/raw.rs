//! This module provides raw functions for transforming strings into different
//! cases. It is used by the functions in the root of this crate.
//!
//! The main function is [`transform`], which takes a string and a buffer and
//! transforms the string into the buffer using the provided "word function" and
//! "delimiter function". The word function is called for each word in the
//! string, and the delimiter function is called for each delimiter between
//! words. For convenience [`to_string`] is provided, which is a thin wrapper
//! around [`transform`] that returns a new `String` instead of writing to a
//! buffer. Additionally, there are several pre-defined word functions and
//! delimiter functions that can be used with [`transform`] and [`to_string`].
//!
//! **Word functions**
//!
//! - [`write_lower`]: converts the word to lowercase
//! - [`write_upper`]: converts the word to uppercase
//! - [`write_title`]: converts the first character (unicode code point) of the
//!   word to uppercase and the rest to lowercase
//!
//! **Delimiter functions**
//!
//! - [`delim_none`]: does nothing (no delimiter)
//!
//! - [`delim_fn`]: returns a "delimiter function" that writes the given
//!   delimiter to the buffer
//!
//!
//! # Examples
//!
//! In this example we convert a string to `SCREAMING.DOT.CASE` a custom
//! conversion that is not provided by this crate.
//!
//! ```
//! use anycase::raw;
//!
//! let input = "Hello world!";
//! let output =  raw::to_string(input, raw::write_upper, raw::delim_fn("."));
//! assert_eq!(output, "HELLO.WORLD");
//! ```

use core::fmt;
use core::fmt::Write;

#[cfg(feature = "alloc")]
use alloc::string::String;

/// Reconstructs the provided string, `s` as a new string using the given "word
/// function" and "delimiter function".
///
/// See the [module level documentation](crate::raw) for more details.
#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub fn to_string<WF, DF>(s: &str, word_fn: WF, delim_fn: DF) -> String
where
    WF: FnMut(&mut String, &str) -> fmt::Result,
    DF: FnMut(&mut String) -> fmt::Result,
{
    let mut buf = String::with_capacity(s.len());
    transform(s, &mut buf, word_fn, delim_fn).expect("fmt error");
    buf
}

#[derive(Copy, Clone, PartialEq)]
enum State {
    Unknown,
    Delims,
    Lower,
    Upper,
}

/// Reconstructs the provided string, `s`, into the given buffer, `buf`, using
/// the given "word function" and "delimiter function".
///
/// See the [module level documentation](crate::raw) for more details.
pub fn transform<B, WF, DF>(s: &str, buf: &mut B, mut word_fn: WF, mut delim_fn: DF) -> fmt::Result
where
    B: Write,
    WF: FnMut(&mut B, &str) -> fmt::Result,
    DF: FnMut(&mut B) -> fmt::Result,
{
    // when we are on the first word
    let mut first = true;
    // the byte index of the start of the current word
    let mut w0 = 0;
    // the byte index of the end of the current word
    let mut w1 = None;
    // the current state of the word boundary machine
    let mut state = State::Unknown;

    let mut write = |w0: usize, w1: usize| -> fmt::Result {
        if w1 - w0 > 0 {
            if first {
                first = false;
            } else {
                delim_fn(buf)?;
            }
            word_fn(buf, &s[w0..w1])?;
        }
        Ok(())
    };

    let mut iter = s.char_indices().peekable();

    while let Some((i, c)) = iter.next() {
        if !c.is_alphanumeric() {
            state = State::Delims;
            w1 = w1.or(Some(i));
            continue;
        }

        let is_lower = c.is_lowercase();
        let is_upper = c.is_uppercase();

        match state {
            State::Delims => {
                if let Some(w1) = w1 {
                    write(w0, w1)?;
                }
                w0 = i;
                w1 = None;
            }
            State::Lower if is_upper => {
                write(w0, i)?;
                w0 = i;
            }
            State::Upper
                if is_upper && matches!(iter.peek(), Some((_, c2)) if c2.is_lowercase()) =>
            {
                write(w0, i)?;
                w0 = i;
            }
            _ => {}
        }

        if is_lower {
            state = State::Lower;
        } else if is_upper {
            state = State::Upper;
        } else if state == State::Delims {
            state = State::Unknown;
        }
    }

    match state {
        State::Delims => {
            if let Some(w1) = w1 {
                write(w0, w1)?;
            }
        }
        _ => write(w0, s.len())?,
    }

    Ok(())
}

/// A "word function" that converts the word to lowercase.
pub fn write_lower<W: Write>(buf: &mut W, s: &str) -> fmt::Result {
    for c in s.chars() {
        write!(buf, "{}", c.to_lowercase())?
    }
    Ok(())
}

/// A "word function" that converts the word to uppercase.
pub fn write_upper<W: Write>(buf: &mut W, s: &str) -> fmt::Result {
    for c in s.chars() {
        write!(buf, "{}", c.to_uppercase())?
    }
    Ok(())
}

/// A "word function" that converts the first character of the word to uppercase
/// and the rest to lowercase.
pub fn write_title<W: Write>(buf: &mut W, s: &str) -> fmt::Result {
    let mut iter = s.chars();
    if let Some(c) = iter.next() {
        write!(buf, "{}", c.to_uppercase())?;
        for c in iter {
            write!(buf, "{}", c.to_lowercase())?;
        }
    }
    Ok(())
}

/// Returns a new stateful "word function" that writes the first word as
/// lowercase and the rest as title case.
pub fn write_camel_fn<W: Write>() -> impl FnMut(&mut W, &str) -> fmt::Result {
    let mut first = true;
    move |buf: &mut W, s: &str| -> fmt::Result {
        if first {
            first = false;
            write_lower(buf, s)?;
        } else {
            write_title(buf, s)?;
        }
        Ok(())
    }
}

/// A "delimiter function" that writes nothing to the buffer.
pub fn delim_none<B>(_: &mut B) -> fmt::Result
where
    B: Write,
{
    Ok(())
}

/// Returns a "delimiter function" that writes the given delimiter to the buffer.
pub fn delim_fn<B>(delim: &str) -> impl Fn(&mut B) -> fmt::Result + '_
where
    B: Write,
{
    move |buf| buf.write_str(delim)
}
