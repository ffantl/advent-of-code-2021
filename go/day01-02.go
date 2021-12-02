package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func day0102(file *os.File) {
	depths := make([]int64, 0, 3)
	previousdepth := int64(0)
	increments := 0

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		if len(depths) == 3 {
			depths = depths[1:]
		}

		depth, err := strconv.ParseInt(scanner.Text(), 10, 64)
		if err != nil {
			panic("trouble in paradise parsing an int")
		}

		depths = append(depths, depth)

		if len(depths) < 3 {
			continue
		} else if previousdepth != 0 && previousdepth < addDepths(depths) {
			increments += 1
		}

		previousdepth = addDepths(depths)
	}

	fmt.Printf("Increments: %d\n", increments)
}

func addDepths(depths []int64) int64 {
	return depths[0] + depths[1] + depths[2]
}
