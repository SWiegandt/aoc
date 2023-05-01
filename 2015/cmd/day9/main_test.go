package main

import (
	"strings"
	"testing"
)

func TestProblems(t *testing.T) {
	input := `
London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141
`
	t.Run("Problem one", func(t *testing.T) {
		ans := ProblemOne(strings.TrimSpace(input))
		expected := 605

		if ans != expected {
			t.Errorf("Problem one result incorrect: got %d, expected %d", ans, expected)
		}
	})

	t.Run("Problem two", func(t *testing.T) {
		ans := ProblemTwo(strings.TrimSpace(input))
		expected := 982

		if ans != expected {
			t.Errorf("Problem two result incorrect: got %d, expected %d", ans, expected)
		}
	})
}
