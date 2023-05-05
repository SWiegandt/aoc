package main

import (
	"fmt"
	"regexp"
	"strconv"
	"strings"

	"github.com/SWiegandt/aoc/2015/pkg/util"
)

func permutations[T any](ts []T) [][]T {
	if len(ts) <= 1 {
		return [][]T{ts}
	}

	var perms [][]T

	for idx, t := range ts {
		var rest []T
		rest = append(rest, ts[:idx]...)
		rest = append(rest, ts[idx+1:]...)

		for _, perm := range permutations(rest) {
			perms = append(perms, append(perm, t))
		}
	}

	return perms
}

type Neighbor struct {
	left, right string
}

func getNeighbors(input string) (map[Neighbor]int, util.Set[string]) {
	pattern := regexp.MustCompile(`(\w+).*(gain|lose) (\d+).*?(\w+).$`)
	people := make(util.Set[string])
	neighbors := make(map[Neighbor]int)

	for _, line := range strings.Split(input, "\n") {
		matches := pattern.FindStringSubmatch(line)
		points, _ := strconv.Atoi(matches[3])

		if matches[2] == "lose" {
			points = -points
		}

		neighbors[Neighbor{matches[1], matches[4]}] = points
		people.Add(matches[1])
	}

	return neighbors, people
}

func getMaxPoints(neighbors map[Neighbor]int, people util.Set[string]) int {
	var maxPoints int

	for _, perm := range permutations(people.Slice()) {
		var pointSum int

		for i := 0; i < len(perm); i++ {
			if i < len(perm)-1 {
				pointSum += neighbors[Neighbor{perm[i], perm[i+1]}] + neighbors[Neighbor{perm[i+1], perm[i]}]
			} else {
				pointSum += neighbors[Neighbor{perm[i], perm[0]}] + neighbors[Neighbor{perm[0], perm[i]}]
			}
		}

		maxPoints = util.Max(maxPoints, pointSum)
	}

	return maxPoints
}

func ProblemOne(input string) int {
	neighbors, people := getNeighbors(input)

	return getMaxPoints(neighbors, people)
}

func ProblemTwo(input string) int {
	neighbors, people := getNeighbors(input)

	for person := range people {
		neighbors[Neighbor{person, "Me"}] = 0
		neighbors[Neighbor{"Me", person}] = 0
	}

	people.Add("Me")

	return getMaxPoints(neighbors, people)
}

func main() {
	input := util.ReadInput(13)

	fmt.Printf("Problem 1: %d\n", ProblemOne(input))
	fmt.Printf("Problem 2: %d\n", ProblemTwo(input))
}
