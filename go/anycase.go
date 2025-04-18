package anycase

import (
	"strings"
	"unicode"
)

// ToCamel converts a string to camelCase.
func ToCamel(s string) string {
	first := true
	writeFn := func(s *strings.Builder, word string) {
		if first {
			WriteLower(s, word)
			first = false
		} else {
			WriteTitle(s, word)
		}
	}
	return Transform(s, writeFn, nil)
}

// ToPascal converts a string to PascalCase.
func ToPascal(s string) string {
	return Transform(s, WriteTitle, nil)
}

// ToSnake converts a string to snake_case.
func ToSnake(s string) string {
	return Transform(s, WriteLower, DelimUnderscore)
}

// ToScreamingSnake converts a string to SCREAMING_SNAKE_CASE.
func ToScreamingSnake(s string) string {
	return Transform(s, WriteUpper, DelimUnderscore)
}

// ToKebab converts a string to kebab-case.
func ToKebab(s string) string {
	return Transform(s, WriteLower, DelimHyphen)
}

// ToScreamingKebab converts a string to SCREAMING-KEBAB-CASE.
func ToScreamingKebab(s string) string {
	return Transform(s, WriteUpper, DelimHyphen)
}

// ToTrain converts a string to Train-Case.
func ToTrain(s string) string {
	return Transform(s, WriteTitle, DelimHyphen)
}

// ToLower converts a string to lower case.
func ToLower(s string) string {
	return Transform(s, WriteLower, DelimSpace)
}

// ToTitle converts a string to Title Case.
func ToTitle(s string) string {
	return Transform(s, WriteTitle, DelimSpace)
}

// ToUpper converts a string to UPPER CASE.
func ToUpper(s string) string {
	return Transform(s, WriteUpper, DelimSpace)
}

type state int

const (
	stateUnknown state = 0
	stateDelims  state = 1
	stateLower   state = 2
	stateUpper   state = 3
)

type delimFn = func(s *strings.Builder)

type writeFn = func(s *strings.Builder, word string)

// Transform reconstructs the provided string using the given "word function" and
// "delimiter function".
//
// The word function is called for each word in the string, and the delimiter
// function is called for each delimiter between words.
func Transform(s string, wordFn writeFn, delimFn delimFn) string {
	out := strings.Builder{}
	out.Grow(len(s))

	runes := []rune(s)

	// when we are on the first word
	first := true
	// the byte index of the start of the current word
	w0 := 0
	// the byte index of the end of the current word
	w1 := -1
	// the current state of the word boundary machine
	state := stateUnknown

	write := func(w0, w1 int) {
		if w1-w0 > 0 {
			if first {
				first = false
			} else if delimFn != nil {
				delimFn(&out)
			}
			wordFn(&out, string(runes[w0:w1]))
		}
	}

	for i := 0; i < len(runes); i++ {
		r := runes[i]
		if !unicode.IsLetter(r) && !unicode.IsNumber(r) {
			state = stateDelims
			if w1 == -1 {
				w1 = i // store the end of the previous word
			}
			continue
		}

		isLower := unicode.IsLower(r)
		isUpper := unicode.IsUpper(r)

		switch {
		case state == stateDelims:
			if w1 != -1 {
				write(w0, w1)
			}
			w0 = i
			w1 = -1
		case state == stateLower && isUpper:
			write(w0, i)
			w0 = i
		case state == stateUpper && isUpper && i+1 < len(runes) && unicode.IsLower(runes[i+1]):
			write(w0, i)
			w0 = i
		}

		if isLower {
			state = stateLower
		} else if isUpper {
			state = stateUpper
		} else if state == stateDelims {
			state = stateUnknown
		}
	}

	switch state {
	case stateDelims:
		if w1 != -1 {
			write(w0, w1)
		}
	default:
		write(w0, len(runes))
	}

	return out.String()
}

// DelimUnderscore is a delimiter function that inserts an underscore.
func DelimUnderscore(s *strings.Builder) {
	s.WriteRune('_')
}

// DelimHyphen is a delimiter function that inserts a hyphen.
func DelimHyphen(s *strings.Builder) {
	s.WriteRune('-')
}

// DelimSpace is a delimiter function that inserts a space.
func DelimSpace(s *strings.Builder) {
	s.WriteRune(' ')
}

// WriteUpper writes the word in uppercase.
func WriteUpper(s *strings.Builder, word string) {
	s.WriteString(strings.ToUpper(word))
}

// WriteLower writes the word in lowercase.
func WriteLower(s *strings.Builder, word string) {
	s.WriteString(strings.ToLower(word))
}

// WriteTitle writes the word in title case.
func WriteTitle(s *strings.Builder, word string) {
	for i, r := range word {
		if i == 0 {
			s.WriteRune(unicode.ToUpper(r))
		} else {
			s.WriteRune(unicode.ToLower(r))
		}
	}
}
