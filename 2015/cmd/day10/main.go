package main

import (
	"fmt"
	"strconv"
	"unicode/utf8"

	"github.com/SWiegandt/aoc/2015/pkg/util"
)

func lookAndSay(input string) string {
	var prev rune
	var count int64
	var output []byte

	for _, curr := range input {
		if prev != 0 && curr != prev {
			output = strconv.AppendInt(output, count, 10)
			output = utf8.AppendRune(output, prev)
			count = 0
		}

		count++
		prev = curr
	}

	output = strconv.AppendInt(output, count, 10)
	output = utf8.AppendRune(output, prev)

	return string(output)
}

func iterate(input string, count int) string {
	for i := 0; i < count; i++ {
		input = lookAndSay(input)
	}

	return input
}

func ProblemOne(input string) int {
	return len(iterate(input, 40))
}

func ProblemTwo(input string) int {
	return len(iterate(input, 50))
}

func main() {
	input := util.ReadInput(10)

	fmt.Printf("Problem 1: %d\n", ProblemOne(input))
	fmt.Printf("Problem 2: %d\n", ProblemTwo(input))
}
