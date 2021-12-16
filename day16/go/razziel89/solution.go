// Package main is the main executable for razziel89's solution for this day's advent.
package main

import (
	"fmt"
	"log"
)

// tag::solution[]

//nolint: funlen
func main() {
	lines, err := ReadLinesAsLines()
	if err != nil {
		log.Fatal(err.Error())
	}
	if len(lines) != 1 {
		log.Fatal("wrong number of lines")
	}

	parsed, err := Parse(lines[0])
	if err != nil {
		log.Fatal(err.Error())
	}

	fmt.Println(parsed)
}

// end::solution[]
