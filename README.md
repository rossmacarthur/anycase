# anycase

[![Go Reference](https://pkg.go.dev/badge/rossmacarthur/anycase/format.svg)](https://pkg.go.dev/github.com/rossmacarthur/anycase)
[![Build Status](https://img.shields.io/github/actions/workflow/status/rossmacarthur/anycase/build.yaml?branch=trunk)](https://github.com/rossmacarthur/anycase/actions/workflows/build.yaml)

A case conversion library for Go.

![anycase](https://github.com/rossmacarthur/anycase/assets/17109887/ab3a4c71-6090-49f3-a67f-b1ab395999c6)

The currently supported anycase are:

| Function                                | Output                 |
| :-------------------------------------- | :--------------------- |
| `anycase.ToCamel(s)`                    | `camelCase`            |
| `anycase.ToPascal(s)`                   | `PascalCase`           |
| `anycase.ToSnake(s)`                    | `snake_case`           |
| `anycase.ToScreamingSnake(s)`           | `SCREAMING_SNAKE_CASE` |
| `anycase.ToKebab(s)`                    | `kebab-case`           |
| `anycase.ToScreamingKebab(s)`           | `SCREAMING-KEBAB-CASE` |
| `anycase.ToTrain(s)`                    | `Train-Case`           |
| `anycase.ToLower(s)`                    | `lower case`           |
| `anycase.ToTitle(s)`                    | `Title Case`           |
| `anycase.ToUpper(s)`                    | `UPPER CASE`           |
| `anycase.Transform(s, wordFn, delimFn)` | *your own case here*   |

Word boundaries are defined as follows:
- A set of consecutive Unicode non-letter/number/symbol e.g. `foo _bar` is two
  words (`foo` and `bar`)
- A transition from a lowercase letter to an uppercase letter e.g. `fooBar` is
  two words (`foo` and `Bar`)
- The second last uppercase letter in a word with multiple uppercase letters
  e.g. `FOOBar` is two words (`FOO` and `Bar`)

## Getting started

Install using

```sh
go get -u github.com/rossmacarthur/anycase
```

Now convert a string using the relevant function.

```go
import "github.com/rossmacarthur/anycase"

anycase.ToSnake("XMLHttpRequest") // returns "xml_http_request"
```

## Customizing

This library also exposes a `Transform` function which allows flexible
customization of the output.

For example if you wanted `dotted.snake.case` you could do the following.

```go
import (
    "strings"
    "github.com/rossmacarthur/anycase"
)

func delimDot(s *strings.Builder) {
    s.WriteRune('.')
}

anycase.Transform("XmlHttpRequest", anycase.ToLower, delimDot) // returns xml.http.request
```

Here is a more involved example in order to handle acronyms in `PascalCase`.

```go
import (
    "strings"
    "github.com/rossmacarthur/anycase"
)

// The default ToPascal function has no understanding of acronyms
anycase.ToPascal("xml_http_request") // returns "XmlHttpRequest"

// We can instead use Transform directly
writeFn := func(s *strings.Builder, word string) {
    w := strings.ToUpper(asLower)
    if w == "XML" || w == "HTTP" {
        s.WriteString(w)
    } else {
        // fallback to default
        anycase.WriteTitle(s, word)
    }
}
anycase.Transform("xml_http_request", writeFn, nil) // returns "XMLHTTPRequest"
```

## License

This project is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.
