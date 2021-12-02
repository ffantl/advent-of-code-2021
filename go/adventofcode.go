package main

import (
	"flag"
	"fmt"
	"os"
)

var filenameFlag string
var adventCode string

func main() {
	file, err := os.Open(filenameFlag)
	if err != nil {
		fmt.Fprintf(os.Stderr, "put a good filename you jerk %s", filenameFlag)
		return
	}
	defer file.Close()

	// For new solutions, add the solution code here, and write the function in a different file
	// The function should be of format file *os.File, otherwise we need to figure out how to get more input into these.
	switch adventCode {
	case "11":
		day0101(file)
	case "12":
		day0102(file)
	default:
		fmt.Printf("Pick a good code, use -code parameter to select an existing function")
	}
}

func init() {
	flag.StringVar(&filenameFlag, "filename", "", "filepath to input file")
	flag.StringVar(&adventCode, "code", "", "code for advent function")
	flag.Parse()
}
