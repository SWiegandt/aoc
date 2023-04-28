package main

import (
	"fmt"
	"strconv"
	"strings"

	"github.com/SWiegandt/aoc/2015/pkg/util"
)

func Problems(input string) (int, int) {
	rows := strings.Split(input, "\n")
	area := 0
	length := 0

	for _, row := range rows {
		dims := strings.SplitN(row, "x", 3)
		x, _ := strconv.Atoi(dims[0])
		y, _ := strconv.Atoi(dims[1])
		z, _ := strconv.Atoi(dims[2])

		area += 2*(x*y+x*z+y*z) + util.Min(x*y, x*z, y*z)
		length += x*y*z + 2*(x+y+z-util.Max(x, y, z))
	}

	return area, length
}

func main() {
	input := util.ReadInput(2)
	one, two := Problems(input)

	fmt.Printf("Problem 1: %d\n", one)
	fmt.Printf("Problem 2: %d\n", two)
}
