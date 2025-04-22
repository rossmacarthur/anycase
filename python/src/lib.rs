use std::fmt;
use std::fmt::Write;

use pyo3::prelude::*;
use pyo3::types::PyDict;

use ::anycase as lib;
use pyo3::types::PyString;

/// Convert a string to 'camelCase'.
#[pyfunction]
#[pyo3(signature = (s, /, acronyms = None))]
fn to_camel(s: &str, acronyms: Option<&Bound<'_, PyDict>>) -> String {
    let mut first = true;
    let word_fn = |buf: &mut String, s: &str| -> fmt::Result {
        if first {
            first = false;
            lib::raw::write_lower(buf, s)
        } else {
            match get_acronym(s, acronyms) {
                Some(acronym) => write!(buf, "{}", acronym),
                None => lib::raw::write_title(buf, s),
            }
        }
    };

    lib::raw::to_string(s, word_fn, lib::raw::delim_none)
}

/// Convert a string to 'PascalCase'.
#[pyfunction]
#[pyo3(signature = (s, /, acronyms = None))]
fn to_pascal(s: &str, acronyms: Option<&Bound<'_, PyDict>>) -> String {
    let word_fn = |buf: &mut String, s: &str| -> fmt::Result {
        match get_acronym(s, acronyms) {
            Some(acronym) => write!(buf, "{}", acronym),
            None => lib::raw::write_title(buf, s),
        }
    };

    lib::raw::to_string(s, word_fn, lib::raw::delim_none)
}

/// Convert a string to 'snake_case'.
#[pyfunction]
fn to_snake(s: &str) -> String {
    lib::to_snake(s)
}

/// Convert a string to 'SCREAMING_SNAKE_CASE'.
#[pyfunction]
fn to_screaming_snake(s: &str) -> String {
    lib::to_screaming_snake(s)
}

/// Convert a string to 'kebab-case'.
#[pyfunction]
fn to_kebab(s: &str) -> String {
    lib::to_kebab(s)
}

/// Convert a string to 'SCREAMING-KEBAB-CASE'.
#[pyfunction]
fn to_screaming_kebab(s: &str) -> String {
    lib::to_screaming_kebab(s)
}

/// Convert a string to 'Train-Case'.
#[pyfunction]
#[pyo3(signature = (s, /, acronyms = None))]
fn to_train(s: &str, acronyms: Option<&Bound<'_, PyDict>>) -> String {
    let word_fn = |buf: &mut String, s: &str| -> fmt::Result {
        match get_acronym(s, acronyms) {
            Some(acronym) => write!(buf, "{}", acronym),
            None => lib::raw::write_title(buf, s),
        }
    };

    lib::raw::to_string(s, word_fn, lib::raw::delim_fn("-"))
}

/// Convert a string to 'lower case'.
#[pyfunction]
fn to_lower(s: &str) -> String {
    lib::to_lower(s)
}

/// Convert a string to 'Title Case'.
#[pyfunction]
#[pyo3(signature = (s, /, acronyms = None))]
fn to_title(s: &str, acronyms: Option<&Bound<'_, PyDict>>) -> String {
    let word_fn = |buf: &mut String, s: &str| -> fmt::Result {
        match get_acronym(s, acronyms) {
            Some(acronym) => write!(buf, "{}", acronym),
            None => lib::raw::write_title(buf, s),
        }
    };

    lib::raw::to_string(s, word_fn, lib::raw::delim_fn(" "))
}

/// Convert a string to 'UPPER CASE'.
#[pyfunction]
fn to_upper(s: &str) -> String {
    lib::to_upper(s)
}

fn get_acronym<'py>(
    k: &str,
    acronyms: Option<&Bound<'py, PyDict>>,
) -> Option<Bound<'py, PyString>> {
    acronyms?.get_item(k.to_lowercase()).ok()??.extract().ok()
}

/// A case conversion library with Unicode support, implemented in Rust.
#[pymodule]
fn anycase(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(to_camel, m)?)?;
    m.add_function(wrap_pyfunction!(to_pascal, m)?)?;
    m.add_function(wrap_pyfunction!(to_snake, m)?)?;
    m.add_function(wrap_pyfunction!(to_screaming_snake, m)?)?;
    m.add_function(wrap_pyfunction!(to_kebab, m)?)?;
    m.add_function(wrap_pyfunction!(to_screaming_kebab, m)?)?;
    m.add_function(wrap_pyfunction!(to_train, m)?)?;
    m.add_function(wrap_pyfunction!(to_lower, m)?)?;
    m.add_function(wrap_pyfunction!(to_title, m)?)?;
    m.add_function(wrap_pyfunction!(to_upper, m)?)?;
    Ok(())
}
