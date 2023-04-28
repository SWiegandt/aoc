package main

import (
	"fmt"
	"strings"

	"github.com/SWiegandt/aoc/2015/pkg/util"
)

func problem_one(input string) int {
	return strings.Count(input, "(") - strings.Count(input, ")")
}

func problem_two(input string) int {
	pos := 0

	for i, ch := range input {
		switch ch {
		case '(':
			pos += 1
		case ')':
			pos -= 1
		}

		if pos < 0 {
			return i + 1
		}
	}

	return -1
}

func main() {
	input := util.ReadInput(1)

	fmt.Printf("Problem 1: %d\n", problem_one(input))
	fmt.Printf("Problem 2: %d\n", problem_two(input))
}
