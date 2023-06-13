package main

import "testing"

func TestProblems(t *testing.T) {
	t.Run("Problem one", func(t *testing.T) {
		ans := ProblemOne("(()(()(")
		expected := 3

		if ans != expected {
			t.Errorf("Problem one result incorrect: got %d, expected %d", ans, expected)
		}
	})

	t.Run("Problem two", func(t *testing.T) {
		ans := ProblemTwo("()())")
		expected := 5

		if ans != expected {
			t.Errorf("Problem two result incorrect: got %d, expected %d", ans, expected)
		}
	})
}
