package main

import (
	"strings"
	"testing"
)

func TestProblems(t *testing.T) {
	input := `
Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.
`

	t.Run("Problem one", func(t *testing.T) {
		ans := ProblemOne(strings.TrimSpace(input), 1000)
		expected := 1120

		if ans != expected {
			t.Errorf("Problem one result incorrect: got %d, expected %d", ans, expected)
		}
	})

	t.Run("Problem two", func(t *testing.T) {
		ans := ProblemTwo(strings.TrimSpace(input), 1000)
		expected := 689

		if ans != expected {
			t.Errorf("Problem two result incorrect: got %d, expected %d", ans, expected)
		}
	})
}
