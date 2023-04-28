package main

import (
	"crypto/md5"
	"encoding/hex"
	"fmt"
	"strings"

	"github.com/SWiegandt/aoc/2015/pkg/util"
)

func hash(prefix string, num int) string {
	data := fmt.Sprintf("%s%d", prefix, num)
	bytes := md5.Sum([]byte(data))

	return hex.EncodeToString(bytes[:])
}

func findHashPrefix(input string, prefix string) int {
	for i := 0; ; i++ {
		if strings.HasPrefix(hash(input, i), prefix) {
			return i
		}
	}
}

func ProblemOne(input string) int {
	return findHashPrefix(input, "00000")
}

func ProblemTwo(input string) int {
	return findHashPrefix(input, "000000")
}

func main() {
	input := util.ReadInput(4)

	fmt.Printf("Problem 1: %d\n", ProblemOne(input))
	fmt.Printf("Problem 2: %d\n", ProblemTwo(input))
}
