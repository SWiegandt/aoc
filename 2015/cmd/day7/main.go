package main

import (
	"fmt"
	"regexp"
	"strconv"
	"strings"

	"github.com/SWiegandt/aoc/2015/pkg/util"
)

var VAL = regexp.MustCompile(`^(\w+) -> (\w+)`)
var NOT = regexp.MustCompile(`NOT (\w+) -> (\w+)`)
var AND = regexp.MustCompile(`(\w+) AND (\w+) -> (\w+)`)
var OR = regexp.MustCompile(`(\w+) OR (\w+) -> (\w+)`)
var LSHIFT = regexp.MustCompile(`(\w+) LSHIFT (\w+) -> (\w+)`)
var RSHIFT = regexp.MustCompile(`(\w+) RSHIFT (\w+) -> (\w+)`)
var valueCache map[string]uint16

func getWireValue(identifier string, wires map[string]func() uint16) uint16 {
	val, err := strconv.Atoi(identifier)

	if err != nil {
		if _, ok := valueCache[identifier]; !ok {
			valueCache[identifier] = wires[identifier]()
		}

		return valueCache[identifier]
	} else {
		return uint16(val)
	}
}

func ProblemOne(input string) uint16 {
	valueCache = make(map[string]uint16)
	wires := make(map[string]func() uint16)

	for _, line := range strings.Split(input, "\n") {
		if m := VAL.FindStringSubmatch(line); m != nil {
			wires[m[2]] = func() uint16 { return getWireValue(m[1], wires) }
		} else if m := NOT.FindStringSubmatch(line); m != nil {
			wires[m[2]] = func() uint16 { return ^getWireValue(m[1], wires) }
		} else if m := AND.FindStringSubmatch(line); m != nil {
			wires[m[3]] = func() uint16 { return getWireValue(m[1], wires) & getWireValue(m[2], wires) }
		} else if m := OR.FindStringSubmatch(line); m != nil {
			wires[m[3]] = func() uint16 { return getWireValue(m[1], wires) | getWireValue(m[2], wires) }
		} else if m := LSHIFT.FindStringSubmatch(line); m != nil {
			wires[m[3]] = func() uint16 { return getWireValue(m[1], wires) << getWireValue(m[2], wires) }
		} else if m := RSHIFT.FindStringSubmatch(line); m != nil {
			wires[m[3]] = func() uint16 { return getWireValue(m[1], wires) >> getWireValue(m[2], wires) }
		}
	}

	return wires["a"]()
}

func ProblemTwo(input string) uint16 {
	a := ProblemOne(input)
	B := regexp.MustCompile(`(?m).+ -> b$`)

	return ProblemOne(B.ReplaceAllString(input, fmt.Sprintf("%v -> b", a)))
}

func main() {
	input := util.ReadInput(7)

	fmt.Printf("Problem 1: %d\n", ProblemOne(input))
	fmt.Printf("Problem 2: %d\n", ProblemTwo(input))
}
