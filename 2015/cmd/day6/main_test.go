package main

import (
	"strings"
	"testing"
)

func TestProblems(t *testing.T) {
		input := `
turn on 0,0 through 999,999
toggle 0,0 through 999,0
turn off 499,499 through 500,500
`

	t.Run("Problem one", func (t *testing.T) {
		ans := ProblemOne(strings.TrimSpace(input))
		expected := 1000000 - 1000 - 4

		if ans != expected {
			t.Errorf("Problem one result incorrect: got %d, expected %d", ans, expected)
		}
	})

	t.Run("Problem two", func (t *testing.T) {
		ans := ProblemTwo(strings.TrimSpace(input))
		expected := 1000000 + 2*1000 - 4

		if ans != expected {
			t.Errorf("Problem two result incorrect: got %d, expected %d", ans, expected)
		}
	})
}
