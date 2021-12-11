// Package main is the main executable for razziel89's solution for this day's advent.
package main

import (
	"fmt"
	"log"
)

// tag::solution[]

const (
	flashThreshold   = 9
	generationsPart1 = 100
	// We set a max number of generations to prevent an endless loop.
	generationsPart2 = 1000
)

func solve(grid Grid, generations, threshold int) (int, int, error) {
	flashes := 0
	togetherFlash := -1
	for gen := 1; gen <= generations; gen++ {
		flashesThisGen := 0
		if err := grid.MarkAll(1); err != nil {
			return 0, 0, err
		}
		addUsBack := []Vec{}
		for o := grid.FilterThreshold(threshold); len(o) > 0; o = grid.FilterThreshold(threshold) {
			// If we found any that will flash, handle them.
			// First, count their flashes.
			flashesThisGen += len(o)
			// Then, remove them from the grid. They cannot flash again this time. But remember to
			// add them back later.
			for _, p := range o {
				grid.RemoveAll(p)
				addUsBack = append(addUsBack, p)
			}
			// Then, their flashes will increase the energy levels of their environment. Propagate
			// that. Only points that are still on the grid will have their energy levels changed.
			for _, p := range o {
				if err := grid.MarkExistingEnv(1, p, pointEnv); err != nil {
					return 0, 0, err
				}
			}
			// Rinse repeat until no more points exceed the threshold.
		}
		// Add those points back that were removed. Their initial energy level will be zero.
		for _, addBack := range addUsBack {
			if err := grid.Mark(addBack, 0); err != nil {
				return 0, 0, err
			}
		}
		// It could be that all octopodes flashed together. If so, remember the first time this
		// happened.
		if togetherFlash < 0 && flashesThisGen == len(grid) {
			togetherFlash = gen
		}
		flashes += flashesThisGen
	}
	return flashes, togetherFlash, nil
}

//nolint: funlen
func main() {
	grid, err := ReadLinesAsGrid()
	if err != nil {
		log.Fatal(err.Error())
	}
	flashes, _, err := solve(grid, generationsPart1, flashThreshold)
	if err != nil {
		log.Fatal(err.Error())
	}
	fmt.Printf("The octopodes flashed %d times in %d generations.\n", flashes, generationsPart1)
	_, togetherFlash, err := solve(grid, generationsPart2-generationsPart1, flashThreshold)
	if err != nil {
		log.Fatal(err.Error())
	}
	if togetherFlash == 0 {
		log.Fatal("the octopodes never flashed all together")
	}
	fmt.Printf(
		"The octopodes all flashed together the 1st time the %d'th generation.\n",
		// The grid will propagate the first 100 generations and then 1000 more. Thus, we need to
		// offset togetherFlash by generationsPart1.
		togetherFlash+generationsPart1,
	)
}

// end::solution[]
