# anycase

[![PyPI version](https://badgers.space/pypi/version/anycase)](https://pypi.org/project/anycase)
[![License](https://badgers.space/github/license/rossmacarthur/anycase)](https://github.com/rossmacarthur/anycase#license)
[![Build Status](https://badgers.space/github/checks/rossmacarthur/anycase/trunk?label=build)](https://github.com/rossmacarthur/anycase/actions/workflows/python.yaml)

A case conversion library for Python.

## Features

- Automatic case detection, no need to specify the input case
- Extremely fast, written in Rust ✨
- Support for Unicode characters
- Support for providing acronyms in title case

**Supported cases**

| Function                        | Output                 |
| :------------------------------ | :--------------------- |
| `anycase.to_camel(s)`           | `camelCase`            |
| `anycase.to_pascal(s)`          | `PascalCase`           |
| `anycase.to_snake(s)`           | `snake_case`           |
| `anycase.to_screaming_snake(s)` | `SCREAMING_SNAKE_CASE` |
| `anycase.to_kebab(s)`           | `kebab-case`           |
| `anycase.to_screaming_kebab(s)` | `SCREAMING-KEBAB-CASE` |
| `anycase.to_train(s)`           | `Train-Case`           |
| `anycase.to_lower(s)`           | `lower case`           |
| `anycase.to_title(s)`           | `Title Case`           |
| `anycase.to_upper(s)`           | `UPPER CASE`           |


## Getting started

Install using

```sh
pip install py-anycase
```

Now convert a string using the relevant function.

```python
import anycase

anycase.to_snake("XMLHttpRequest") # returns "xml_http_request"
```

## Details

Each of the provided functions using the same underlying implementation which
does the following:
- Divide the input string into words
- Convert each word as required
- Join the words back together optionally with a separator

Word boundaries are defined as follows:

- A set of consecutive Unicode non-letter and non-number characters.

  For example: 'foo _bar' is two words (foo and bar)

- A transition from a lowercase letter to an uppercase letter.

  For example: fooBar is two words (foo and Bar)

- A transition from multiple uppercase letters to a single uppercase letter
  followed by lowercase letters.

  For example: FOOBar is two words (FOO and Bar)

Functions where the transform is "title" accept an optional `acronyms` argument,
which is a mapping of lowercase words to their output. For example:

```python
>>> anycase.to_pascal("xml_http_request", acronyms={"xml": "XML"})
'XMLHttpRequest'
>>> anycase.to_pascal("xml_http_request", acronyms={"xml": "XML", "http": "HTTP"})
'XMLHTTPRequest'
```

## Benchmarks

A simple benchmark against various other libraries is provided in
[./benches](./benches). The following table shows the results when run on my
Macbook M2 Max.

| Library                   |  Min (µs) |  Max (µs) |     Mean (µs) |
| :------------------------ | --------: | --------: | ------------: |
| py-anycase                |    26.666 |   176.834 |    **30.909** |
| pyheck                    |    51.000 |   131.416 |    **53.565** |
| pure python               |    63.583 |   108.125 |    **65.075** |
| re                        |    81.916 |   171.000 |    **87.856** |
| stringcase                |    99.250 |   222.292 |   **102.197** |
| pydantic.alias_generators |   182.000 |   304.458 |   **189.063** |
| inflection                |   229.750 |   360.792 |   **239.153** |
| caseconversion            | 1,430.042 | 1,838.375 | **1,559.019** |

## License

This project is licensed under the terms of the MIT license. See
[LICENSE](LICENSE) for more details.
