package main

import (
	"strings"
	"testing"
)

func TestProblems(t *testing.T) {
	input := `
""
"abc"
"aaa\"aaa"
"\x27"
`
	t.Run("Problem one", func(t *testing.T) {
		ans := ProblemOne(strings.TrimSpace(input))
		expected := 12

		if ans != expected {
			t.Errorf("Problem one result incorrect: got %d, expected %d", ans, expected)
		}
	})

	t.Run("Problem two", func(t *testing.T) {
		ans := ProblemTwo(strings.TrimSpace(input))
		expected := 19

		if ans != expected {
			t.Errorf("Problem two result incorrect: got %d, expected %d", ans, expected)
		}
	})
}
