use core::fmt;

use crate::raw;

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
pub fn camel<S: AsRef<str>>(s: S) -> impl fmt::Display {
    as_case! { s, raw::write_camel_fn(), raw::delim_none }
}

/// Display a string as 'PascalCase'.
pub fn pascal<S: AsRef<str>>(s: S) -> impl fmt::Display {
    as_case! { s, raw::write_title, raw::delim_none }
}

/// Display a string as 'snake_case'.
pub fn snake<S: AsRef<str>>(s: S) -> impl fmt::Display {
    as_case! { s, raw::write_lower, raw::delim_fn("_") }
}

/// Display a string as 'SCREAMING_SNAKE_CASE'.
pub fn screaming_snake<S: AsRef<str>>(s: S) -> impl fmt::Display {
    as_case! { s, raw::write_upper, raw::delim_fn("_") }
}

/// Display a string as 'kebab-case'.
pub fn kebab<S: AsRef<str>>(s: S) -> impl fmt::Display {
    as_case! { s, raw::write_lower, raw::delim_fn("-") }
}

/// Display a string as 'SCREAMING-KEBAB-CASE'.
pub fn screaming_kebab<S: AsRef<str>>(s: S) -> impl fmt::Display {
    as_case! { s, raw::write_upper, raw::delim_fn("-") }
}

/// Display a string as 'Train-Case'.
pub fn train<S: AsRef<str>>(s: S) -> impl fmt::Display {
    as_case! { s, raw::write_title, raw::delim_fn("-") }
}

/// Display a string as 'lower case'.
pub fn lower<S: AsRef<str>>(s: S) -> impl fmt::Display {
    as_case! { s, raw::write_lower, raw::delim_fn(" ") }
}

/// Display a string as 'Title Case'.
pub fn title<S: AsRef<str>>(s: S) -> impl fmt::Display {
    as_case! { s, raw::write_title, raw::delim_fn(" ") }
}

/// Display a string as 'UPPER CASE'.
pub fn upper<S: AsRef<str>>(s: S) -> impl fmt::Display {
    as_case! { s, raw::write_upper, raw::delim_fn(" ") }
}
