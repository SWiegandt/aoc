package main

import (
	"strings"
	"testing"
)

func TestProblems(t *testing.T) {
	t.Run("Problem one", func(t *testing.T) {
		ans := ProblemOne(strings.TrimSpace(`{"a":{"b":4},"c":-1}`))
		expected := 3.0

		if ans != expected {
			t.Errorf("Problem one result incorrect: got %f, expected %f", ans, expected)
		}
	})

	t.Run("Problem two", func(t *testing.T) {
		ans := ProblemTwo(strings.TrimSpace(`[1,{"c":"red","b":2},3]`))
		expected := 4.0

		if ans != expected {
			t.Errorf("Problem two result incorrect: got %f, expected %f", ans, expected)
		}
	})
}
