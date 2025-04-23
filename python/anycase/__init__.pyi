"""
A case conversion library with Unicode support, implemented in Rust.

This implementation divides the input string into words and applies a "word
function" to each word and calls a "delimiter function" for each word boundary
(the space between words).

Word boundaries are defined as follows:
- A set of consecutive non-letter/number/symbol e.g. `foo _bar` is two words
  `foo` and `bar`.
- A transition from a lowercase letter to an uppercase letter e.g. `fooBar` is
  two words `foo` and `Bar`.
- The second last uppercase letter in a word with multiple uppercase letters
  e.g. `FOOBar` is two words `FOO` and `Bar`.
"""

from typing import Optional

def to_camel(s: str, acronyms: Optional[dict[str, str]] = None) -> str:
    """
    Convert a string to 'camelCase'.

    The first word will be converted to lowercase and subsequent words to title
    case. See module documentation for how word boundaries are defined.

    For example:

        >>> anycase.to_camel("foo_bar")
        'fooBar'

    The `acronyms` argument is a mapping of lowercase words to an override
    value. This value will be used instead of the camel case conversion.

    For example:

        >>> anycase.to_camel("xml http request", acronyms={"http": "HTTP"})
        'xmlHTTPRequest'

    """
    ...

def to_pascal(s: str, acronyms: Optional[dict[str, str]] = None) -> str:
    """
    Convert a string to 'PascalCase'.

    Each word will be converted to title case. See module documentation for how
    word boundaries are defined.

    For example:

        >>> anycase.to_pascal("foo_bar")
        'FooBar'

    The `acronyms` argument is a mapping of lowercase words to an override
    value. This value will be used instead of the pascal case conversion.

    For example:

        >>> anycase.to_pascal("xml http request", acronyms={"http": "HTTP"})
        'XmlHTTPRequest'

    """
    ...

def to_snake(s: str) -> str:
    """
    Convert a string to 'snake_case'.

    Each word will be converted to lower case and separated with an underscore.
    See module documentation for how word boundaries are defined.

    For example:

        >>> anycase.to_snake("fooBar")
        'foo_bar'

    """
    ...

def to_screaming_snake(s: str) -> str:
    """
    Convert a string to 'SCREAMING_SNAKE_CASE'.

    Each word will be converted to upper case and separated with an underscore.
    See module documentation for how word boundaries are defined.

    For example:

        >>> anycase.to_screaming_snake("fooBar")
        'FOO_BAR'

    """
    ...

def to_kebab(s: str) -> str:
    """
    Convert a string to 'kebab-case'.

    Each word will be converted to lower case and separated with a hyphen. See
    module documentation for how word boundaries are defined.

    For example:

        >>> anycase.to_kebab("fooBar")
        'foo-bar'

    """
    ...

def to_screaming_kebab(s: str) -> str:
    """
    Convert a string to 'SCREAMING-KEBAB-CASE'.

    Each word will be converted to upper case and separated with a hyphen. See
    module documentation for how word boundaries are defined.

    For example:

        >>> anycase.to_screaming_kebab("fooBar")
        'FOO-BAR'

    """
    ...

def to_train(s: str, acronyms: Optional[dict[str, str]] = None) -> str:
    """
    Convert a string to 'Train-Case'.

    Each word will be converted to title case and separated with a hyphen. See
    module documentation for how word boundaries are defined.

    For example:

        >>> anycase.to_train("fooBar")
        'Foo-Bar'

    The `acronyms` argument is a mapping of lowercase words to an override
    value. This value will be used instead of the train case conversion.

    For example:

        >>> anycase.to_train("xml http request", acronyms={"http": "HTTP"})
        'Xml-HTTP-Request'

    """
    ...

def to_lower(s: str) -> str:
    """
    Convert a string to 'lower case'.

    Each word will be converted to lower case and separated with a space. See
    module documentation for how word boundaries are defined.

    For example:

        >>> anycase.to_lower("FooBar")
        'foo bar'

    """
    ...

def to_title(s: str, acronyms: Optional[dict[str, str]] = None) -> str:
    """
    Convert a string to 'Title Case'.

    Each word will be converted to title case and separated with a space. See
    module documentation for how word boundaries are defined.

    For example:

        >>> anycase.to_title("foo_bar")
        'Foo Bar'

    The `acronyms` argument is a mapping of lowercase words to an override
    value. This value will be used instead of the title case conversion.

    For example:

        >>> anycase.to_title("xml_http_request", acronyms={"http": "HTTP"})
        'Xml HTTP Request'

    """
    ...

def to_upper(s: str) -> str:
    """
    Convert a string to 'UPPER CASE'.

    Each word will be converted to upper case and separated with a space. See
    module documentation for how word boundaries are defined.

    For example:

        >>> anycase.to_upper("fooBar")
        'FOO BAR'

    """
    ...
