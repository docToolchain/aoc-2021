// Package main is the main executable for razziel89's solution for this day's advent.
package main

import (
	"fmt"
	"log"
)

// tag::solution[]

const (
	dangerThreshold = 2
)

func filterDanger(num int) bool {
	return num >= dangerThreshold
}

//nolint: funlen
func main() {
	grid := make(Grid)
	lines, err := ReadLinesAsLines()
	if err != nil {
		log.Fatal(err.Error())
	}
	for _, line := range lines {
		points, err := line.Points()
		if err != nil {
			log.Fatal(err.Error())
		}
		for _, point := range points {
			grid.Mark(point)
		}
	}
	danger := grid.FilterCounts(filterDanger)
	fmt.Printf("There are %d dangerous spots.\n", len(danger))
}

// end::solution[]
