// Package main is the main executable for razziel89's solution for this day's advent.
package main

import (
	"fmt"
	"log"
)

// tag::solution[]

//nolint: funlen
func main() {
	grid, err := ReadLinesAsGrid()
	if err != nil {
		log.Fatal(err.Error())
	}
	fmt.Println(grid)
	risk := 0
	for point := range grid.Points() {
		if grid.IsLocalMin(point) {
			risk += 1 + grid.Count(point)
			fmt.Println(point, grid.Count(point))
		}
	}
	fmt.Printf("There is %d risk.\n", risk)
}

// end::solution[]
