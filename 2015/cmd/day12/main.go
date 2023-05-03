package main

import (
	"encoding/json"
	"fmt"

	"github.com/SWiegandt/aoc/2015/pkg/util"
)

func sumJsonObject(obj map[string]interface{}, ignoreRed bool) float64 {
	var sum float64

	for _, value := range obj {
		switch parsed := value.(type) {
		case map[string]interface{}:
			sum += sumJsonObject(parsed, ignoreRed)
		case []interface{}:
			sum += sumJsonArray(parsed, ignoreRed)
		case float64:
			sum += parsed
		case string:
			if parsed == "red" && ignoreRed {
				return 0
			}
		}
	}

	return sum
}

func sumJsonArray(arr []interface{}, ignoreRed bool) float64 {
	var sum float64

	for _, value := range arr {
		switch parsed := value.(type) {
		case map[string]interface{}:
			sum += sumJsonObject(parsed, ignoreRed)
		case []interface{}:
			sum += sumJsonArray(parsed, ignoreRed)
		case float64:
			sum += parsed
		}
	}

	return sum
}

func sumJson(obj interface{}, ignoreRed bool) float64 {
	switch parsed := obj.(type) {
	case map[string]interface{}:
		return sumJsonObject(parsed, ignoreRed)
	case []interface{}:
		return sumJsonArray(parsed, ignoreRed)
	default:
		return obj.(float64)
	}
}

func ProblemOne(input string) float64 {
	var obj interface{}
	json.Unmarshal([]byte(input), &obj)

	return sumJson(obj, false)
}

func ProblemTwo(input string) float64 {
	var obj interface{}
	json.Unmarshal([]byte(input), &obj)

	return sumJson(obj, true)
}

func main() {
	input := util.ReadInput(12)

	fmt.Printf("Problem 1: %d\n", int64(ProblemOne(input)))
	fmt.Printf("Problem 2: %d\n", int64(ProblemTwo(input)))
}
