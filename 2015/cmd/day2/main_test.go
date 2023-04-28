package main

import "testing"

func TestProblems(t *testing.T) {
	{
		one, two := Problems("2x3x4")
		expected := []int{58, 34}

		if one != expected[0] {
			t.Errorf("Problem one result incorrect: got %d, expected %d", one, expected[0])
		}

		if two != expected[1] {
			t.Errorf("Problem two result incorrect: got %d, expected %d", two, expected[1])
		}
	}
}
