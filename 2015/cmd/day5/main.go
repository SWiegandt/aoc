package main

import (
	"fmt"
	"strings"

	"github.com/SWiegandt/aoc/2015/pkg/util"
	"golang.org/x/exp/slices"
)

func ProblemOne(input string) int {
	vowels := []rune{'a', 'e', 'i', 'o', 'u'}
	nice := 0

outer:
	for _, line := range strings.Split(input, "\n") {
		vowelCount := 0
		var prevChar rune
		hasCharDouble := false

		for _, char := range line {
			if slices.Contains(vowels, char) {
				vowelCount += 1
			}

			hasCharDouble = hasCharDouble || char == prevChar

			if slices.Contains([]string{"ab", "cd", "pq", "xy"}, string([]rune{prevChar, char})) {
				continue outer
			}

			prevChar = char
		}

		if vowelCount >= 3 && hasCharDouble {
			nice += 1
		}
	}

	return nice
}

func ProblemTwo(input string) int {
	nice := 0

	for _, line := range strings.Split(input, "\n") {
		buffer := make([]rune, 3)
		buffers := make(map[string]interface{})
		hasOffsetCharDouble := false
		hasDigraphDouble := false

		for _, char := range line {
			hasOffsetCharDouble = hasOffsetCharDouble || char == buffer[1]
			isOverlap := char == buffer[1] && char == buffer[2] && char != buffer[0]
			buffer = append(buffer[1:], char)

			if _, ok := buffers[string(buffer[1:])]; ok && !isOverlap {
				hasDigraphDouble = true
			}

			buffers[string(buffer[1:])] = nil
		}

		if hasOffsetCharDouble && hasDigraphDouble {
			nice += 1
		}
	}

	return nice
}

func main() {
	input := util.ReadInput(5)

	fmt.Printf("Problem 1: %d\n", ProblemOne(input))
	fmt.Printf("Problem 2: %d\n", ProblemTwo(input))
}
