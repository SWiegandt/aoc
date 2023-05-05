package main

import (
	"strings"
	"testing"
)

func TestProblems(t *testing.T) {
	input := `
Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.
`
	t.Run("Problem one", func(t *testing.T) {
		ans := ProblemOne(strings.TrimSpace(input))
		expected := 330

		if ans != expected {
			t.Errorf("Problem one result incorrect: got %d, expected %d", ans, expected)
		}
	})

	// 	t.Run("Problem two", func (t *testing.T) {
	// 		input := `

	// `
	// 		ans := ProblemTwo(strings.TrimSpace(input))
	// 		expected := _

	// 		if ans != expected {
	// 			t.Errorf("Problem two result incorrect: got %d, expected %d", ans, expected)
	// 		}
	// 	})
}
