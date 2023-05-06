package main

import (
	"fmt"
	"regexp"
	"strconv"
	"strings"

	"github.com/SWiegandt/aoc/2015/pkg/util"
)

type Movement struct {
	speed    int
	duration int
	rest     int
}

func parseMovement(s string) Movement {
	re := regexp.MustCompile(`.*?(\d+).*?(\d+).*?(\d+)`)
	matches := re.FindStringSubmatch(s)
	speed, _ := strconv.Atoi(matches[1])
	duration, _ := strconv.Atoi(matches[2])
	rest, _ := strconv.Atoi(matches[3])

	return Movement{speed, duration, rest}
}

func calculateDistance(movement Movement, time int) int {
	duration := movement.duration + movement.rest

	return movement.speed * (movement.duration*(time/duration) + util.Min(time%duration, movement.duration))
}

func ProblemOne(input string, time int) int {
	var max int

	for _, line := range strings.Split(input, "\n") {
		movement := parseMovement(line)
		distance := calculateDistance(movement, time)
		max = util.Max(max, distance)
	}

	return max
}

func ProblemTwo(input string, time int) int {
	movements := make([]Movement, 0)
	points := make([]int, 0)

	for _, line := range strings.Split(input, "\n") {
		movements = append(movements, parseMovement(line))
		points = append(points, 0)
	}

	for t := 1; t <= time; t++ {
		var max int
		var distances []int

		for _, movement := range movements {
			distance := calculateDistance(movement, t)
			distances = append(distances, distance)
			max = util.Max(max, distance)
		}

		for idx, dist := range distances {
			if dist == max {
				points[idx]++
			}
		}
	}

	return util.Max(points...)
}

func main() {
	input := util.ReadInput(14)

	fmt.Printf("Problem 1: %d\n", ProblemOne(input, 2503))
	fmt.Printf("Problem 2: %d\n", ProblemTwo(input, 2503))
}
