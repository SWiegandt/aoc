package main

import (
	"fmt"

	"github.com/SWiegandt/aoc/2015/pkg/util"
)

func presents(input string, units int) int {
	positions := make([]util.Point, units)

	for i := 0; i < units; i++ {
		positions[i] = util.Point{X: 0, Y: 0}
	}

	visited := map[util.Point]bool{positions[0]: true}

	for i, chr := range input {
		pos := &positions[i%units]

		switch chr {
		case '^':
			pos.Y += 1
		case '<':
			pos.X -= 1
		case '>':
			pos.X += 1
		case 'v':
			pos.Y -= 1
		}

		visited[*pos] = true
	}

	return len(visited)
}

func ProblemOne(input string) int {
	return presents(input, 1)
}

func ProblemTwo(input string) int {
	return presents(input, 2)
}

func main() {
	input := util.ReadInput(3)

	fmt.Printf("Problem 1: %d\n", ProblemOne(input))
	fmt.Printf("Problem 2: %d\n", ProblemTwo(input))
}
