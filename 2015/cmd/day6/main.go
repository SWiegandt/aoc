package main

import (
	"fmt"
	"strconv"
	"strings"

	"github.com/SWiegandt/aoc/2015/pkg/util"
)

func coords(line string, fromStart, toStart int) (int, int, int, int) {
	from := strings.Split(strings.Split(line, " ")[fromStart], ",")
	to := strings.Split(strings.Split(line, " ")[toStart], ",")
	fromX, _ := strconv.Atoi(from[0])
	fromY, _ := strconv.Atoi(from[1])
	toX, _ := strconv.Atoi(to[0])
	toY, _ := strconv.Atoi(to[1])

	return fromX, fromY, toX, toY
}

func ProblemOne(input string) int {
	var lights [1000][1000]bool

	for _, line := range strings.Split(input, "\n") {
		var lightModifier func(bool) bool
		var toX, toY, fromX, fromY int

		if strings.HasPrefix(line, "turn on") {
			fromX, fromY, toX, toY = coords(line, 2, 4)
			lightModifier = func(_ bool) bool { return true }
		} else if strings.HasPrefix(line, "toggle") {
			fromX, fromY, toX, toY = coords(line, 1, 3)
			lightModifier = func(b bool) bool { return !b }
		} else if strings.HasPrefix(line, "turn off") {
			fromX, fromY, toX, toY = coords(line, 2, 4)
			lightModifier = func(_ bool) bool { return false }
		}

		for x := fromX; x <= toX; x++ {
			for y := fromY; y <= toY; y++ {
				lights[x][y] = lightModifier(lights[x][y])
			}
		}
	}

	turnedOn := 0

	for _, line := range lights {
		for _, light := range line {
			if light {
				turnedOn += 1
			}
		}
	}

	return turnedOn
}

func ProblemTwo(input string) int {
	var lights [1000][1000]int

	for _, line := range strings.Split(input, "\n") {
		var lightModifier func(int) int
		var toX, toY, fromX, fromY int

		if strings.HasPrefix(line, "turn on") {
			fromX, fromY, toX, toY = coords(line, 2, 4)
			lightModifier = func(i int) int { return i + 1 }
		} else if strings.HasPrefix(line, "toggle") {
			fromX, fromY, toX, toY = coords(line, 1, 3)
			lightModifier = func(i int) int { return i + 2 }
		} else if strings.HasPrefix(line, "turn off") {
			fromX, fromY, toX, toY = coords(line, 2, 4)
			lightModifier = func(i int) int { return util.Max(0, i-1) }
		}

		for x := fromX; x <= toX; x++ {
			for y := fromY; y <= toY; y++ {
				lights[x][y] = lightModifier(lights[x][y])
			}
		}
	}

	turnedOn := 0

	for _, line := range lights {
		for _, light := range line {
			turnedOn += light
		}
	}

	return turnedOn
}

func main() {
	input := util.ReadInput(6)

	fmt.Printf("Problem 1: %d\n", ProblemOne(input))
	fmt.Printf("Problem 2: %d\n", ProblemTwo(input))
}
