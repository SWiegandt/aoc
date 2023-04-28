package main

import "testing"

func TestProblems(t *testing.T) {
	input := "^>v<"

	{
		ans := ProblemOne(input)
		expected := 4

		if ans != expected {
			t.Errorf("Problem one result incorrect: got %d, expected %d", ans, expected)
		}
	}

	{
		ans := ProblemTwo(input)
		expected := 3

		if ans != expected {
			t.Errorf("Problem two result incorrect: got %d, expected %d", ans, expected)
		}
	}
}
