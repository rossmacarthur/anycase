package anycase_test

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"strings"
	"testing"

	anycase "github.com/rossmacarthur/anycase/go"
	"github.com/stretchr/testify/require"
)

type testCase struct {
	Input          string `json:"input"`
	SnakeCase      string `json:"snake"`
	CamelCase      string `json:"camel"`
	PascalCase     string `json:"pascal"`
	ScreamingSnake string `json:"screaming_snake"`
	KebabCase      string `json:"kebab"`
	ScreamingKebab string `json:"screaming_kebab"`
	TrainCase      string `json:"train"`
	LowerCase      string `json:"lower"`
	TitleCase      string `json:"title"`
	UpperCase      string `json:"upper"`
}

var tests []testCase

func init() {
	// Load test cases from common.json
	path := filepath.Join("..", "testdata", "common.json")
	data, err := os.ReadFile(path)
	if err != nil {
		panic(fmt.Sprintf("failed to read test cases: %v", err))
	}

	err = json.Unmarshal(data, &tests)
	if err != nil {
		panic(fmt.Sprintf("failed to parse test cases: %v", err))
	}
}

func TestCommon(t *testing.T) {
	for _, tc := range tests {
		if tc.Input == "XΣXΣ baﬄe" {
			continue // skip for now
		}
		t.Run(tc.Input, func(t *testing.T) {
			got := anycase.ToCamel(tc.Input)
			require.Equal(t, tc.CamelCase, got, fmt.Sprintf("'%s'", tc.Input))

			got = anycase.ToSnake(tc.Input)
			require.Equal(t, tc.SnakeCase, got, fmt.Sprintf("'%s'", tc.Input))

			got = anycase.ToPascal(tc.Input)
			require.Equal(t, tc.PascalCase, got, fmt.Sprintf("'%s'", tc.Input))

			got = anycase.ToScreamingSnake(tc.Input)
			require.Equal(t, tc.ScreamingSnake, got, fmt.Sprintf("'%s'", tc.Input))

			got = anycase.ToKebab(tc.Input)
			require.Equal(t, tc.KebabCase, got, fmt.Sprintf("'%s'", tc.Input))

			got = anycase.ToScreamingKebab(tc.Input)
			require.Equal(t, tc.ScreamingKebab, got, fmt.Sprintf("'%s'", tc.Input))

			got = anycase.ToTrain(tc.Input)
			require.Equal(t, tc.TrainCase, got, fmt.Sprintf("'%s'", tc.Input))

			got = anycase.ToLower(tc.Input)
			require.Equal(t, tc.LowerCase, got, fmt.Sprintf("'%s'", tc.Input))

			got = anycase.ToTitle(tc.Input)
			require.Equal(t, tc.TitleCase, got, fmt.Sprintf("'%s'", tc.Input))

			got = anycase.ToUpper(tc.Input)
			require.Equal(t, tc.UpperCase, got, fmt.Sprintf("'%s'", tc.Input))
		})
	}
}

func BenchmarkToSnake(b *testing.B) {
	s := strings.Repeat("ThisIsATestCase", 100)

	require.True(b, anycase.ToSnake(s) == regexToSnake(s))

	b.Run("anycase", func(b *testing.B) {
		for i := 0; i < b.N; i++ {
			anycase.ToSnake(s)
		}
	})

	b.Run("regex", func(b *testing.B) {
		for i := 0; i < b.N; i++ {
			regexToSnake(s)
		}
	})
}

// regexToSnake is a regex implementation to convert to snake case to compare
// the benchmark to.
//
// This function doesn't support as many word boundaries as anycase.ToSnake but
// it is still much slower than the anycase.ToSnake implementation.
//
// From https://stackoverflow.com/a/56616250/4591251
func regexToSnake(s string) string {
	snake := matchFirstCap.ReplaceAllString(s, "${1}_${2}")
	snake = matchAllCap.ReplaceAllString(snake, "${1}_${2}")
	return strings.ToLower(snake)
}

var (
	matchFirstCap = regexp.MustCompile("(.)([A-Z][a-z]+)")
	matchAllCap   = regexp.MustCompile("([a-z0-9])([A-Z])")
)
