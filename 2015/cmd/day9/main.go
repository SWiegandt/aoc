package main

import (
	"fmt"
	"math"
	"regexp"
	"strconv"
	"strings"

	"github.com/SWiegandt/aoc/2015/pkg/util"
)

type Path struct {
	from string
	to   string
}

func toPathDistance(s string) (Path, int) {
	re := regexp.MustCompile(`(\w+) to (\w+) = (\d+)`)
	matches := re.FindStringSubmatch(s)
	distance, _ := strconv.Atoi(matches[3])

	return Path{
		from: matches[1],
		to:   matches[2],
	}, distance
}

func minDistance(paths map[Path]int, from string, toVisit util.Set[string], comparison func(int, int) bool, start int) int {
	if len(toVisit) == 0 {
		return 0
	}

	result := start

	for to := range toVisit {
		if distance := paths[Path{from, to}] + minDistance(paths, to, toVisit.Without(to), comparison, start); comparison(distance, result) {
			result = distance
		}
	}

	return result
}

func findPath(input string, comparison func(int, int) bool, start int) int {
	paths := make(map[Path]int)
	locations := make(util.Set[string])
	result := start

	for _, line := range strings.Split(input, "\n") {
		path, distance := toPathDistance(line)
		paths[path] = distance
		paths[Path{path.to, path.from}] = distance
		locations.Add(path.from)
		locations.Add(path.to)
	}

	for from := range locations {
		if distance := minDistance(paths, from, locations.Without(from), comparison, start); comparison(distance, result) {
			result = distance
		}
	}

	return result
}

func ProblemOne(input string) int {
	return findPath(input, func(a, b int) bool { return a < b }, math.MaxInt)
}

func ProblemTwo(input string) int {
	return findPath(input, func(a, b int) bool { return a > b }, 0)
}

func main() {
	input := util.ReadInput(9)

	fmt.Printf("Problem 1: %d\n", ProblemOne(input))
	fmt.Printf("Problem 2: %d\n", ProblemTwo(input))
}
