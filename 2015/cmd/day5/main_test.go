package main

import (
	"strings"
	"testing"
)

func TestProblems(t *testing.T) {
	t.Run("Problem one", func (t *testing.T) {
		input := `
ugknbfddgicrmopn
aaa
jchzalrnumimnmhp
haegwjzuvuyypxyu
dvszwmarrgswjxmb
`
		ans := ProblemOne(strings.TrimSpace(input))
		expected := 2

		if ans != expected {
			t.Errorf("Problem one result incorrect: got %d, expected %d", ans, expected)
		}

	})

	t.Run("Problem two", func (t *testing.T) {
		input := `
qjhvhtzxzqqjkmpb
xxyxx
uurcxstgmygtbstg
ieodomkazucvgmuy
`
		ans := ProblemTwo(strings.TrimSpace(input))
		expected := 2

		if ans != expected {
			t.Errorf("Problem two result incorrect: got %d, expected %d", ans, expected)
		}
	})
}
