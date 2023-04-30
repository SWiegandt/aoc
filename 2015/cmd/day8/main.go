package main

import (
	"fmt"
	"strconv"
	"strings"

	"github.com/SWiegandt/aoc/2015/pkg/util"
)

func ProblemOne(input string) int {
	codeLength := 0
	stringLength := 0

	for _, line := range strings.Split(input, "\n") {
		codeLength += len(line)
		unquoted, _ := strconv.Unquote(line)
		stringLength += len(unquoted)
	}

	return codeLength - stringLength
}

func ProblemTwo(input string) int {
	codeLength := 0
	quotedLength := 0

	for _, line := range strings.Split(input, "\n") {
		quotedLength += len(strconv.Quote(line))
		codeLength += len(line)
	}

	return quotedLength - codeLength
}

func main() {
	input := util.ReadInput(8)

	fmt.Printf("Problem 1: %d\n", ProblemOne(input))
	fmt.Printf("Problem 2: %d\n", ProblemTwo(input))
}
