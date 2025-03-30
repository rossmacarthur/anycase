//! 💼 A case conversion library for Rust.
//!
//! # 🚀 Getting started
//!
//! First, add the `anycase` crate to your Cargo manifest.
//!
//! ```sh
//! cargo add anycase
//! ```
//!
//! Then, you can use the provided conversion functions in your code:
//!
//! ```
//! let s = anycase::to_snake("Hello world!");
//! assert_eq!(s, "hello_world");
//! ```
//!
//! # 🤸 Usage
//!
//! The `anycase` crate provides a set of functions to convert strings between
//! different case styles. The following functions are available.
//!
//! Given an input of `Hello world!`:
//!
//! - [`to_camel`]           outputs `helloWorld`
//! - [`to_pascal`]          outputs `HelloWorld`
//! - [`to_snake`]           outputs `hello_world`
//! - [`to_screaming_snake`] outputs `HELLO_WORLD`
//! - [`to_kebab`]           outputs `hello-world`
//! - [`to_screaming_kebab`] outputs `HELLO_WORLD`
//! - [`to_train`]           outputs `Hello-World`
//! - [`to_lower`]           outputs `hello world`
//! - [`to_title`]           outputs `Hello World`
//! - [`to_upper`]           outputs `HELLO WORLD`
//!
//! Additionally, the crate provides the [`fmt`] module containing the raw
//! functions which can be used to implement custom case conversion functions.
//!
//! ```
//! use anycase::fmt;
//!
//! let input = "Hello world!";
//! let output =  fmt::to_string(input, fmt::write_upper, fmt::delim_fn("."));
//! assert_eq!(output, "HELLO.WORLD");
//! ```
//!
//! See the [module level documentation](crate::fmt) for more details.
//!
//! # How does it work?
//!
//! This implementation divides the input string into words and applies a "word
//! function" to each word and calls a "delimiter function" for each word
//! boundary (the space between words).
//!
//! Word boundaries are defined as follows:
//! - A set of consecutive non-letter/number/symbol e.g. `foo _bar` is two words
//!   `foo` and `bar`.
//! - A transition from a lowercase letter to an uppercase letter e.g. `fooBar`
//!   is two words `foo` and `Bar`.
//! - The second last uppercase letter in a word with multiple uppercase letters
//!   e.g. `FOOBar` is two words `FOO` and `Bar`.
//!
//! The following `char` methods are used in the above conditions:
//!
//! - [`char::is_alphanumeric`] is used to determine if a character is a
//!   letter/number/symbol
//! - [`char::is_lowercase`] is used to determine if a character is a lowercase
//!   letter
//! - [`char::is_uppercase`] is used to determine if a character is an uppercase
//!   letter
//!
//! # MSRV
//!
//! The minimum supported Rust version (MSRV) is 1.56.0.  The policy of this
//! crate is to only increase the MSRV in a breaking release.

#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::string::String;

pub mod fmt;

/// Convert a string to 'camelCase'.
#[cfg(feature = "alloc")]
pub fn to_camel(s: &str) -> String {
    let mut first = true;
    let word_fn = |buf: &mut String, s: &str| -> fmt::Result {
        if first {
            first = false;
            fmt::write_lower(buf, s)?;
        } else {
            fmt::write_title(buf, s)?;
        }
        Ok(())
    };
    fmt::to_string(s, word_fn, |_| Ok(()))
}

/// Convert a string to 'PascalCase'.
#[cfg(feature = "alloc")]
pub fn to_pascal(s: &str) -> String {
    fmt::to_string(s, fmt::write_title, fmt::delim_none)
}

/// Convert a string to 'snake_case'.
#[cfg(feature = "alloc")]
pub fn to_snake(s: &str) -> String {
    fmt::to_string(s, fmt::write_lower, fmt::delim_fn("_"))
}

/// Convert a string to 'SCREAMING_SNAKE_CASE'.
#[cfg(feature = "alloc")]
pub fn to_screaming_snake(s: &str) -> String {
    fmt::to_string(s, fmt::write_upper, fmt::delim_fn("_"))
}

/// Convert a string to 'kebab-case'.
#[cfg(feature = "alloc")]
pub fn to_kebab(s: &str) -> String {
    fmt::to_string(s, fmt::write_lower, fmt::delim_fn("-"))
}

/// Convert a string to 'SCREAMING-KEBAB-CASE'.
#[cfg(feature = "alloc")]
pub fn to_screaming_kebab(s: &str) -> String {
    fmt::to_string(s, fmt::write_upper, fmt::delim_fn("-"))
}

/// Convert a string to 'Train-Case'.
#[cfg(feature = "alloc")]
pub fn to_train(s: &str) -> String {
    fmt::to_string(s, fmt::write_title, fmt::delim_fn("-"))
}

/// Convert a string to 'lower case'.
#[cfg(feature = "alloc")]
pub fn to_lower(s: &str) -> String {
    fmt::to_string(s, fmt::write_lower, fmt::delim_fn(" "))
}

/// Convert a string to 'Title Case'.
#[cfg(feature = "alloc")]
pub fn to_title(s: &str) -> String {
    fmt::to_string(s, fmt::write_title, fmt::delim_fn(" "))
}

/// Convert a string to 'UPPER CASE'.
#[cfg(feature = "alloc")]
pub fn to_upper(s: &str) -> String {
    fmt::to_string(s, fmt::write_upper, fmt::delim_fn(" "))
}
