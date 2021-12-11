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
	maxGenerationsPart2 = 1000
)

// SolutionOutput is the output of `solve`'s channel.
type SolutionOutput struct {
	value int
	err   error
}

func solve(grid Grid, threshold int) <-chan SolutionOutput {
	channel := make(chan SolutionOutput)

	go func() {
		for {
			flashesThisGen := 0
			if err := grid.MarkAll(1); err != nil {
				channel <- SolutionOutput{0, err}
				close(channel)
				return
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
						channel <- SolutionOutput{0, err}
						close(channel)
						return
					}
				}
				// Rinse repeat until no more points exceed the threshold.
			}
			// Add those points back that were removed. Their initial energy level will be zero.
			for _, addBack := range addUsBack {
				if err := grid.Mark(addBack, 0); err != nil {
					channel <- SolutionOutput{0, err}
					close(channel)
					return
				}
			}
			channel <- SolutionOutput{flashesThisGen, nil}
		}
	}()

	return channel
}

//nolint: funlen
func main() {
	grid, err := ReadLinesAsGrid()
	if err != nil {
		log.Fatal(err.Error())
	}
	numOctopodes := len(grid)
	flashes := 0
	solutionGenerator := solve(grid, flashThreshold)
	for gen := 1; gen <= generationsPart1; gen++ {
		sol := <-solutionGenerator
		if err := sol.err; err != nil {
			log.Fatal(err.Error())
		}
		flashes += sol.value
	}
	fmt.Printf("The octopodes flashed %d times in %d generations.\n", flashes, generationsPart1)
	for gen := generationsPart1; gen <= maxGenerationsPart2; gen++ {
		sol := <-solutionGenerator
		if err := sol.err; err != nil {
			log.Fatal(err.Error())
		}
		if sol.value == numOctopodes {
			fmt.Printf(
				"The octopodes all flashed together the 1st time the %d'th generation.\n", gen,
			)
			return
		}
	}
	log.Fatal("the octopodes never flashed all together")
}

// end::solution[]
