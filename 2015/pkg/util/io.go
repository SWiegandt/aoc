package util

import (
	"fmt"
	"log"
	"os"
	"strings"
)

func ReadInput(day int) string {
	data, err := os.ReadFile(fmt.Sprintf("input/%d.txt", day))

	if err != nil {
		log.Fatal(err)
	}

	return strings.TrimSpace(string(data))
}
