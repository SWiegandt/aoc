package main

import (
	"fmt"
	"regexp"
	"strconv"
	"strings"

	"github.com/SWiegandt/aoc/2015/pkg/util"
)

func partitions(total int, numBuckets int) [][]int {
	if numBuckets == 1 {
		return [][]int{{total}}
	}

	var buckets [][]int

	for i := 0; i <= total; i++ {
		for _, bucket := range partitions(total-i, numBuckets-1) {
			buckets = append(buckets, append(bucket, i))
		}
	}

	return buckets
}

type Ingredient [5]int

func parseIngredient(line string) Ingredient {
	re := regexp.MustCompile(`.*?(-?\d+).*?(-?\d+).*?(-?\d+).*?(-?\d+).*?(-?\d+)`)
	matches := re.FindStringSubmatch(line)
	capacity, _ := strconv.Atoi(matches[1])
	durability, _ := strconv.Atoi(matches[2])
	flavor, _ := strconv.Atoi(matches[3])
	texture, _ := strconv.Atoi(matches[4])
	calories, _ := strconv.Atoi(matches[5])

	return Ingredient{capacity, durability, flavor, texture, calories}
}

func calculateScore(input string, calorieRestriction bool) int {
	var ingredients []Ingredient
	var maxProduct int

	for _, line := range strings.Split(input, "\n") {
		ingredients = append(ingredients, parseIngredient(line))
	}

outer:
	for _, partition := range partitions(100, len(ingredients)) {
		var totals Ingredient
		var product int = 1

		for i, amount := range partition {
			for j, attribute := range ingredients[i] {
				totals[j] += amount * attribute
			}
		}

		if calorieRestriction && totals[4] != 500 {
			continue outer
		}

		for i := 0; i < len(totals)-1; i++ {
			if totals[i] <= 0 {
				continue outer
			} else {
				product *= totals[i]
			}
		}

		maxProduct = util.Max(maxProduct, product)
	}

	return maxProduct
}

func ProblemOne(input string) int {
	return calculateScore(input, false)
}

func ProblemTwo(input string) int {
	return calculateScore(input, true)
}

func main() {
	input := util.ReadInput(15)

	fmt.Printf("Problem 1: %d\n", ProblemOne(input))
	fmt.Printf("Problem 2: %d\n", ProblemTwo(input))
}
