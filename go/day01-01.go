package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func day0101(file *os.File) {
	previous := int64(0)
	increments := 0

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		depth, err := strconv.ParseInt(scanner.Text(), 10, 64)
		if err != nil {
			panic("trouble in paradise parsing an int")
		}

		if previous != 0 && previous < depth {
			increments += 1
		}

		previous = depth
	}

	fmt.Printf("Increments: %d\n", increments)
}
