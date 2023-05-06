package main

import (
	"fmt"
	"regexp"
	"strconv"
	"strings"

	"github.com/SWiegandt/aoc/2015/pkg/util"
	"golang.org/x/exp/slices"
)

type Aunt map[string]int

func parseAunt(s string) Aunt {
	aunt := make(Aunt)
	auntSplit := strings.SplitN(s, " ", 3)
	re := regexp.MustCompile(`(\w+): (\d+)(, )?`)

	for _, matches := range re.FindAllStringSubmatch(auntSplit[2], -1) {
		count, _ := strconv.Atoi(matches[2])
		aunt[matches[1]] = count
	}

	return aunt
}

var testResult Aunt = Aunt{
	"children":    3,
	"cats":        7,
	"samoyeds":    2,
	"pomeranians": 3,
	"akitas":      0,
	"vizslas":     0,
	"goldfish":    5,
	"trees":       3,
	"cars":        2,
	"perfumes":    1,
}

func ProblemOne(input string) int {
outer:
	for i, line := range strings.Split(input, "\n") {
		for key, value := range parseAunt(line) {
			if value != testResult[key] {
				continue outer
			}
		}

		return i + 1
	}

	return -1
}

func ProblemTwo(input string) int {
	gtKeys := []string{"cats", "trees"}
	ltKeys := []string{"pomeranians", "goldfish"}

outer:
	for i, line := range strings.Split(input, "\n") {
		for key, value := range parseAunt(line) {
			resultValue := testResult[key]

			if slices.Contains(gtKeys, key) && value <= resultValue {
				continue outer
			} else if slices.Contains(ltKeys, key) && value >= resultValue {
				continue outer
			} else if !slices.Contains(gtKeys, key) && !slices.Contains(ltKeys, key) && value != resultValue {
				continue outer
			}
		}

		return i + 1
	}

	return -1
}

func main() {
	input := util.ReadInput(16)

	fmt.Printf("Problem 1: %d\n", ProblemOne(input))
	fmt.Printf("Problem 2: %d\n", ProblemTwo(input))
}
