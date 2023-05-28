package main

import (
	"fmt"
	"strconv"
	"strings"

	"github.com/SWiegandt/aoc/2015/pkg/util"
)

func combinations(containers []int, amount int) [][]int {
	var combs [][]int

	if amount == 0 {
		return [][]int{{}}
	}

	for i, container := range containers {
		if container <= amount {
			for _, comb := range combinations(containers[i+1:], amount-container) {
				combs = append(combs, append(comb, container))
			}
		}
	}

	return combs
}

func parseContainers(input string) []int {

	var containers []int

	for _, container := range strings.Split(input, "\n") {
		c, _ := strconv.Atoi(container)
		containers = append(containers, c)
	}

	return containers
}

func ProblemOne(input string, liters int) int {
	return len(combinations(parseContainers(input), liters))
}

func ProblemTwo(input string, liters int) int {
	sizes := make([]int, liters+1)
	combinations := combinations(parseContainers(input), liters)
	minSize := liters

	for _, c := range combinations {
		sizes[len(c)] += 1
		minSize = util.Min(minSize, len(c))
	}

	return sizes[minSize]
}

func main() {
	input := util.ReadInput(17)

	fmt.Printf("Problem 1: %d\n", ProblemOne(input, 150))
	fmt.Printf("Problem 2: %d\n", ProblemTwo(input, 150))
}
