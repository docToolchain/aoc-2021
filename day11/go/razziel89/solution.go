// Package main is the main executable for razziel89's solution for this day's advent.
package main

import (
	"fmt"
	"log"
)

// tag::solution[]

const (
	flashThreshold = 9
)

func solve(grid Grid, threshold int) int {
	flashes := 0
	return flashes
}

//nolint: funlen
func main() {
	grid, err := ReadLinesAsGrid()
	if err != nil {
		log.Fatal(err.Error())
	}
	flashes := solve(grid, flashThreshold)
	fmt.Printf("Total number of flashes is %d.\n", flashes)
}

// end::solution[]
