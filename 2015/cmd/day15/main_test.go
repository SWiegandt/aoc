package main

import (
	"strings"
	"testing"
)

func TestProblems(t *testing.T) {
	input := `
Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3
`
	t.Run("Problem one", func(t *testing.T) {
		ans := ProblemOne(strings.TrimSpace(input))
		expected := 62842880

		if ans != expected {
			t.Errorf("Problem one result incorrect: got %d, expected %d", ans, expected)
		}
	})

	t.Run("Problem two", func(t *testing.T) {
		ans := ProblemTwo(strings.TrimSpace(input))
		expected := 57600000

		if ans != expected {
			t.Errorf("Problem two result incorrect: got %d, expected %d", ans, expected)
		}
	})
}
