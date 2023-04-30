package main

import (
	"strings"
	"testing"
)

func TestProblems(t *testing.T) {
	t.Run("Problem one", func(t *testing.T) {
		input := `
123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> a
`
		ans := ProblemOne(strings.TrimSpace(input))
		var expected uint16 = 65079

		if ans != expected {
			t.Errorf("Problem one result incorrect: got %d, expected %d", ans, expected)
		}
	})
}
