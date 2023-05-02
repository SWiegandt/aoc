package main

import (
	"strings"
	"testing"
)

func TestProblems(t *testing.T) {
	input := "abcdefgh"

	t.Run("Problem one", func(t *testing.T) {
		one, _ := Problems(strings.TrimSpace(input))
		expected := "abcdffaa"

		if one != expected {
			t.Errorf("Problem one result incorrect: got %s, expected %s", one, expected)
		}
	})
}
