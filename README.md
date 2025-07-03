# anycase

A case conversion library for [Go](#go), [Rust](#rust), and [Python](#python).

<img src="https://github.com/rossmacarthur/anycase/assets/17109887/ab3a4c71-6090-49f3-a67f-b1ab395999c6" width="500">

Anycase provides a consistent way of converting between different case styles.
And has a similar API across languages.

## Go

Install using

```sh
go get -u github.com/rossmacarthur/anycase/go
```

Now convert a string using the relevant function.

```go
import "github.com/rossmacarthur/anycase/go"

anycase.ToSnake("XMLHttpRequest") // returns "xml_http_request"
```

[View full documentation](./go).

## Rust

Add the `anycase` crate to your Cargo manifest.

```sh
cargo add anycase
```

Now convert a string using the relevant function.

```rust
anycase::to_snake("XMLHttpRequest"); // returns "xml_http_request"
```

[View full documentation](./rust).

## Python

Install using

```sh
pip install py-anycase
```

Now convert a string using the relevant function.

```python
import anycase

anycase.to_snake("XMLHttpRequest") # returns "xml_http_request"
```

[View full documentation](./python).

## How does it work?

Each implementation divides the input string into words and applies a “word
function” to each word and calls a “delimiter function” for each word
boundary (the space between words).

Word boundaries are defined as follows:

- A set of consecutive non-letter/number/symbol e.g. `foo _bar` is two words
  `foo` and `bar`.
- A transition from a lowercase letter to an uppercase letter e.g. `fooBar`
  is two words `foo` and `Bar`.
- The second last uppercase letter in a word with multiple uppercase letters
  e.g. `FOOBar` is two words `FOO` and `Bar`.

## License

This project is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.
