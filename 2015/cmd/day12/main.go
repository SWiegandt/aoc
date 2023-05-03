package main

import (
	"encoding/json"
	"fmt"
	"github.com/SWiegandt/aoc/2015/pkg/util"
)

func sumJsonObject(obj map[string]interface{}, ignoreRed bool) float64 {
	var sum float64

	for _, value := range obj {
		if parsed, ok := value.(map[string]interface{}); ok {
			sum += sumJsonObject(parsed, ignoreRed)
		} else if parsed, ok := value.([]interface{}); ok {
			sum += sumJsonArray(parsed, ignoreRed)
		} else if parsed, ok := value.(float64); ok {
			sum += parsed
		} else if parsed, ok := value.(string); ok && parsed == "red" && ignoreRed {
			return 0
		}
	}

	return sum
}

func sumJsonArray(arr []interface{}, ignoreRed bool) float64 {
	var sum float64

	for _, value := range arr {
		if parsed, ok := value.(map[string]interface{}); ok {
			sum += sumJsonObject(parsed, ignoreRed)
		} else if parsed, ok := value.([]interface{}); ok {
			sum += sumJsonArray(parsed, ignoreRed)
		} else if parsed, ok := value.(float64); ok {
			sum += parsed
		}
	}

	return sum
}

func sumJson(obj interface{}, ignoreRed bool) float64 {
	if parsed, ok := obj.(map[string]interface{}); ok {
		return sumJsonObject(parsed, ignoreRed)
	} else if parsed, ok := obj.([]interface{}); ok {
		return sumJsonArray(parsed, ignoreRed)
	}

	return obj.(float64)
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
