package main

import "testing"

func TestProblems(t *testing.T) {
	input := "abcdef"

	{
		ans := ProblemOne(input)
		expected := 609043

		if ans != expected {
			t.Errorf("Problem one result incorrect: got %d, expected %d", ans, expected)
		}
	}
}
