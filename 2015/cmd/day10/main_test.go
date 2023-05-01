package main

import (
	"strings"
	"testing"
)

func TestProblems(t *testing.T) {
	input := `

`
	t.Run("Problem one", func(t *testing.T) {
		ans := ProblemOne(strings.TrimSpace(input))
		expected := _

		if ans != expected {
			t.Errorf("Problem one result incorrect: got %d, expected %d", ans, expected)
		}
	})

	// 	t.Run("Problem two", func (t *testing.T) {
	// 		input := `

	// `
	// 		ans := ProblemTwo(strings.TrimSpace(input))
	// 		expected := _

	// 		if ans != expected {
	// 			t.Errorf("Problem two result incorrect: got %d, expected %d", ans, expected)
	// 		}
	// 	})
}
