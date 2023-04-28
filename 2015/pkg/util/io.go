package util

import (
	"fmt"
	"log"
	"os"
)

func ReadInput(day int) string {
	data, err := os.ReadFile(fmt.Sprintf("input/%d.txt", day))

	if err != nil {
		log.Fatal(err)
	}

	return string(data)
}
