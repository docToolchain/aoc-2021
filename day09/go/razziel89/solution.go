// Package main is the main executable for razziel89's solution for this day's advent.
package main

import (
	"fmt"
	"log"
	"sort"
)

// tag::solution[]

const (
	noMoreBasinHeight = 9
	numLargestBasins  = 3
)

//nolint: funlen
func main() {
	grid, err := ReadLinesAsGrid()
	if err != nil {
		log.Fatal(err.Error())
	}
	minima := []Vec{}
	risk := 0
	for point := range grid.Points() {
		if grid.IsLocalMin(point) {
			risk += 1 + grid.Count(point)
			minima = append(minima, point)
		}
	}
	// minima = []Vec{Vec{x: 0, y: 1}}
	fmt.Printf("There is %d risk.\n", risk)
	sizes := []int{}
	for _, min := range minima {
		basin, err := grid.Basin(min, noMoreBasinHeight-1)
		if err != nil {
			log.Fatal(err.Error())
		}
		sizes = append(sizes, len(basin))
	}
	// Multiply the sizes of the three largest basins.
	sort.Ints(sizes)
	sizeMult := 1
	for _, size := range sizes[len(sizes)-numLargestBasins:] {
		sizeMult *= size
	}
	fmt.Printf("Multiplied basin size is %d.\n", sizeMult)
}

// end::solution[]
