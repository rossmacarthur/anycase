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
//! Alternatively, you can use the [`fmt`] module to get a [`Display`] type.
//!
//! ```
//! let s = format!("snake case: {}", anycase::fmt::snake("Hello world!"));
//! assert_eq!(s, "snake case: hello_world");
//! ```
//!
//! [`Display`]: core::fmt::Display
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
//! Additionally, the crate provides the [`raw`] module containing the raw
//! functions which can be used to implement custom case conversion functions.
//!
//! ```
//! use anycase::raw;
//!
//! let input = "Hello world!";
//! let output =  raw::to_string(input, raw::write_upper, raw::delim_fn("."));
//! assert_eq!(output, "HELLO.WORLD");
//! ```
//!
//! See the [module level documentation](crate::raw) for more details.
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
#[cfg(feature = "alloc")]
use alloc::string::ToString;

pub mod fmt;
pub mod raw;

/// Returns a string in 'camelCase'.
#[cfg(feature = "alloc")]
#[inline]
pub fn to_camel<S: AsRef<str>>(s: S) -> String {
    fmt::camel(s).to_string()
}

/// Returns a string in 'PascalCase'.
#[cfg(feature = "alloc")]
#[inline]
pub fn to_pascal<S: AsRef<str>>(s: S) -> String {
    fmt::pascal(s).to_string()
}

/// Returns a string in 'snake_case'.
#[cfg(feature = "alloc")]
#[inline]
pub fn to_snake<S: AsRef<str>>(s: S) -> String {
    fmt::snake(s).to_string()
}

/// Returns a string in 'SCREAMING_SNAKE_CASE'.
#[cfg(feature = "alloc")]
#[inline]
pub fn to_screaming_snake<S: AsRef<str>>(s: S) -> String {
    fmt::screaming_snake(s).to_string()
}

/// Returns a string in 'kebab-case'.
#[cfg(feature = "alloc")]
#[inline]
pub fn to_kebab<S: AsRef<str>>(s: S) -> String {
    fmt::kebab(s).to_string()
}

/// Returns a string in 'SCREAMING-KEBAB-CASE'.
#[cfg(feature = "alloc")]
#[inline]
pub fn to_screaming_kebab<S: AsRef<str>>(s: S) -> String {
    fmt::screaming_kebab(s).to_string()
}

/// Returns a string in 'Train-Case'.
#[cfg(feature = "alloc")]
#[inline]
pub fn to_train<S: AsRef<str>>(s: S) -> String {
    fmt::train(s).to_string()
}

/// Returns a string in 'lower case'.
#[cfg(feature = "alloc")]
#[inline]
pub fn to_lower<S: AsRef<str>>(s: S) -> String {
    fmt::lower(s).to_string()
}

/// Returns a string in 'Title Case'.
#[cfg(feature = "alloc")]
#[inline]
pub fn to_title<S: AsRef<str>>(s: S) -> String {
    fmt::title(s).to_string()
}

/// Returns a string in 'UPPER CASE'.
#[cfg(feature = "alloc")]
#[inline]
pub fn to_upper<S: AsRef<str>>(s: S) -> String {
    fmt::upper(s).to_string()
}
