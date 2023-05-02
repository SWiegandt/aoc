package main

import (
	"fmt"

	"golang.org/x/exp/slices"
)

type Password struct {
	chars []int32
}

func (p *Password) inc() {
	for pos, placeValue := range p.chars {
		if placeValue < 26 {
			p.chars[pos] = placeValue + 1
			break
		} else {
			p.chars[pos] = 1
		}
	}
}

func (p *Password) toString() string {
	length := len(p.chars)
	runes := make([]rune, length)

	for i := length; i > 0; i-- {
		runes[length-i] = rune('a') + p.chars[i-1] - 1
	}

	return string(runes)
}

func (p *Password) isValid() bool {
	for _, forbidden := range []rune{'i', 'o', 'l'} {
		if slices.Contains(p.chars, forbidden-rune('a')+1) {
			return false
		}
	}

	buf := make([]rune, 3)
	var numPairs int
	var hasTriple bool

	for _, char := range p.chars {
		if char == buf[2] && char != buf[1] {
			numPairs++
		}

		buf = append(buf[1:], char)

		if buf[0]-buf[1] == 1 && buf[1]-buf[2] == 1 {
			hasTriple = true
		}
	}

	return hasTriple && numPairs >= 2
}

func password(s string) Password {
	start := rune('a')
	chars := make([]int32, len(s))

	for pos, char := range s {
		chars[len(s)-pos-1] = (char - start + 1)
	}

	return Password{chars}
}

func Problems(input string) (string, string) {
	pwd := password(input)

	for !pwd.isValid() {
		pwd.inc()
	}

	nextPwd := password(pwd.toString())
	nextPwd.inc()

	for !nextPwd.isValid() {
		nextPwd.inc()
	}

	return pwd.toString(), nextPwd.toString()
}

func main() {
	input := "vzbxkghb"
	one, two := Problems(input)

	fmt.Printf("Problem 1: %s\n", one)
	fmt.Printf("Problem 2: %s\n", two)
}
