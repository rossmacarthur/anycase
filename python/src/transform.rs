use std::fmt;
use std::fmt::Write;

#[derive(Copy, Clone, PartialEq)]
enum State {
    Unknown,
    Delims,
    Lower,
    Upper,
}

/// Transforms reconstructs the string into a new string using the given
/// functions.
pub fn to_string<F>(s: &str, word_fn: F, delim: &str) -> String
where
    F: FnMut(&mut String, &str) -> fmt::Result,
{
    let mut buf = String::with_capacity(s.len());
    transform(s, &mut buf, word_fn, delim).unwrap();
    buf
}

/// Transform reconstructs the string into the given buffer using the given
/// functions.
pub fn transform<B, W>(s: &str, buf: &mut B, mut word_fn: W, delim: &str) -> fmt::Result
where
    B: Write,
    W: FnMut(&mut B, &str) -> fmt::Result,
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
                buf.write_str(delim)?;
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

pub fn fmt_lower<W: Write>(buf: &mut W, s: &str) -> fmt::Result {
    for c in s.chars() {
        write!(buf, "{}", c.to_lowercase())?
    }
    Ok(())
}

pub fn fmt_upper<W: Write>(buf: &mut W, s: &str) -> fmt::Result {
    for c in s.chars() {
        write!(buf, "{}", c.to_uppercase())?
    }
    Ok(())
}

pub fn fmt_title<W: Write>(buf: &mut W, s: &str) -> fmt::Result {
    let mut iter = s.chars();
    if let Some(c) = iter.next() {
        write!(buf, "{}", c.to_uppercase())?;
        for c in iter {
            write!(buf, "{}", c.to_lowercase())?;
        }
    }
    Ok(())
}
