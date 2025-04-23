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
//! Then you can use the `as_<case>` function to get a [`Display`] type.
//! ```
//! let s = format!("snake case: {}", anycase::as_snake("Hello world!"));
//! assert_eq!(s, "snake case: hello_world");
//! ```
//!
//! Alternatively, you can use the `to_<case>` function to get a `String`.
//! ```
//! let s = anycase::to_snake("Hello world!");
//! assert_eq!(s, "hello_world");
//! ```
//!
//! # 🤸 Usage
//!
//! The `anycase` crate provides a set of functions to convert strings between
//! different case styles. The following cases are available.
//!
//! Given an input of `Hello world!`:
//!
//! - [`as_camel`]           displays `helloWorld`
//! - [`as_pascal`]          displays `HelloWorld`
//! - [`as_snake`]           displays `hello_world`
//! - [`as_screaming_snake`] displays `HELLO_WORLD`
//! - [`as_kebab`]           displays `hello-world`
//! - [`as_screaming_kebab`] displays `HELLO_WORLD`
//! - [`as_train`]           displays `Hello-World`
//! - [`as_lower`]           displays `hello world`
//! - [`as_title`]           displays `Hello World`
//! - [`as_upper`]           displays `HELLO WORLD`
//!
//! For all of the above functions, you can use the `to_<case>` variant to get a
//! `String` instead of a [`Display`] type.
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
//! # Features
//!
//! This crate is designed to be `no_std` compatible. This is made possible by
//! disabling all default features. The following features are available:
//!
//! - **`std`** _(enabled by default)_ — Currently only enables the `alloc`
//!   feature but is here to allow for forward compatibility with any
//!   [`std`]-only features.
//!
//! - **`alloc`** — Links the [`alloc`] crate and enables the use of `String`
//!   functions.
//!
//! # MSRV
//!
//! The minimum supported Rust version (MSRV) is 1.56.0.  The policy of this
//! crate is to only increase the MSRV in a breaking release.
//!
//! [`Display`]: core::fmt::Display
//! [`alloc`]: http://doc.rust-lang.org/alloc/
//! [`std`]: http://doc.rust-lang.org/std/

#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod raw;

use core::fmt;

#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::string::ToString;

macro_rules! as_case {
    { $s:ident, $wf:expr, $df:expr } => {
        struct AsCase<S>(S);

        impl<S: AsRef<str>> fmt::Display for AsCase<S> {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                raw::transform(self.0.as_ref(), f, $wf, $df)
            }
        }

        AsCase($s)
    };
}

/// Display a string as 'camelCase'.
pub fn as_camel<S: AsRef<str>>(s: S) -> impl fmt::Display {
    as_case! { s, raw::write_camel_fn(), raw::delim_none }
}
/// Transforms a string to 'camelCase'.
#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub fn to_camel<S: AsRef<str>>(s: S) -> String {
    as_camel(s).to_string()
}

/// Display a string as 'PascalCase'.
pub fn as_pascal<S: AsRef<str>>(s: S) -> impl fmt::Display {
    as_case! { s, raw::write_title, raw::delim_none }
}
/// Transforms a string to 'PascalCase'.
#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub fn to_pascal<S: AsRef<str>>(s: S) -> String {
    as_pascal(s).to_string()
}

/// Display a string as 'snake_case'.
pub fn as_snake<S: AsRef<str>>(s: S) -> impl fmt::Display {
    as_case! { s, raw::write_lower, raw::delim_fn("_") }
}
/// Transforms a string to 'snake_case'.
#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub fn to_snake<S: AsRef<str>>(s: S) -> String {
    as_snake(s).to_string()
}

/// Display a string as 'SCREAMING_SNAKE_CASE'.
pub fn as_screaming_snake<S: AsRef<str>>(s: S) -> impl fmt::Display {
    as_case! { s, raw::write_upper, raw::delim_fn("_") }
}
/// Transforms a string to 'SCREAMING_SNAKE_CASE'.
#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub fn to_screaming_snake<S: AsRef<str>>(s: S) -> String {
    as_screaming_snake(s).to_string()
}

/// Display a string as 'kebab-case'.
pub fn as_kebab<S: AsRef<str>>(s: S) -> impl fmt::Display {
    as_case! { s, raw::write_lower, raw::delim_fn("-") }
}
/// Transforms a string to 'kebab-case'.
#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub fn to_kebab<S: AsRef<str>>(s: S) -> String {
    as_kebab(s).to_string()
}

/// Display a string as 'SCREAMING-KEBAB-CASE'.
pub fn as_screaming_kebab<S: AsRef<str>>(s: S) -> impl fmt::Display {
    as_case! { s, raw::write_upper, raw::delim_fn("-") }
}
/// Transforms a string to 'SCREAMING-KEBAB-CASE'.
#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub fn to_screaming_kebab<S: AsRef<str>>(s: S) -> String {
    as_screaming_kebab(s).to_string()
}

/// Display a string as 'Train-Case'.
pub fn as_train<S: AsRef<str>>(s: S) -> impl fmt::Display {
    as_case! { s, raw::write_title, raw::delim_fn("-") }
}
/// Transforms a string to 'Train-Case'.
#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub fn to_train<S: AsRef<str>>(s: S) -> String {
    as_train(s).to_string()
}

/// Display a string as 'lower case'.
pub fn as_lower<S: AsRef<str>>(s: S) -> impl fmt::Display {
    as_case! { s, raw::write_lower, raw::delim_fn(" ") }
}
/// Transforms a string to 'lower case'.
#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub fn to_lower<S: AsRef<str>>(s: S) -> String {
    as_lower(s).to_string()
}

/// Display a string as 'Title Case'.
pub fn as_title<S: AsRef<str>>(s: S) -> impl fmt::Display {
    as_case! { s, raw::write_title, raw::delim_fn(" ") }
}
/// Transforms a string to 'Title Case'.
#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub fn to_title<S: AsRef<str>>(s: S) -> String {
    as_title(s).to_string()
}

/// Display a string as 'UPPER CASE'.
pub fn as_upper<S: AsRef<str>>(s: S) -> impl fmt::Display {
    as_case! { s, raw::write_upper, raw::delim_fn(" ") }
}
/// Transforms a string to 'UPPER CASE'.
#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub fn to_upper<S: AsRef<str>>(s: S) -> String {
    as_upper(s).to_string()
}
