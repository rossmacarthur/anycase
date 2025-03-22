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

// Transform reconstructs the string using the given functions.
//
// wordFn is called for each word and delimFn is called for each word boundary.
func Transform(s string, wf writeFn, df delimFn) string {
	out := strings.Builder{}
	out.Grow(len(s))

	runes := []rune(s)

	// when we are on the first word
	first := true
	// the byte index of the start of the current word
	start := 0
	// the byte index of the end of the current word
	end := -1
	// the current state of the word boundary machine
	state := stateUnknown

	emit := func(end int) {
		if end-start > 0 {
			if first {
				first = false
			} else if df != nil {
				df(&out)
			}
			wf(&out, string(runes[start:end]))
		}
	}

	for i, r := range runes {
		if !(unicode.IsLetter(r) || unicode.IsNumber(r) || unicode.IsSymbol(r)) {
			state = stateDelims
			if end == -1 {
				end = i // store the end of the previous word
			}
			continue
		}

		isLower := unicode.IsLower(r)
		isUpper := unicode.IsUpper(r)

		if state == stateDelims {
			emit(end)
			start = i
			end = -1

		} else if state == stateLower && isUpper {
			emit(i)
			start = i

		} else if state == stateUpper && isUpper &&
			i+1 < len(s) && unicode.IsLower(runes[i+1]) {
			emit(i)
			start = i
		}

		if isLower {
			state = stateLower
		} else if isUpper {
			state = stateUpper
		}
	}

	if state == stateDelims {
		emit(end)
	} else {
		emit(len(runes))
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
