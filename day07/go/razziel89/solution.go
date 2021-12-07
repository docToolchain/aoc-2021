// Package main is the main executable for razziel89's solution for this day's advent.
package main

import (
	"fmt"
	"log"
)

// tag::solution[]

// For each entry in the set, compute the cost based on some cost function. The target will be every
// possible number between the smallest and largest entry in the set. Multiples of set entries are
// accounted for. The lowest cost and associated target are returned.
func overallCost(set CountingSet, costFn func(int, int) int) (int, int) {
	bestCost := 0
	bestPos := 0
	found := false
	for checkPos := set.Min(); checkPos <= set.Max(); checkPos++ {
		checkCost := 0
		for _, startPos := range set.Keys() {
			additionalCost := costFn(startPos, checkPos) * set.Count(startPos)
			checkCost += additionalCost
		}
		if !found || checkCost < bestCost {
			bestPos = checkPos
			bestCost = checkCost
			found = true
		}
	}
	return bestCost, bestPos
}

//nolint: funlen
func main() {
	sets, err := ReadLinesAsSets()
	if err != nil {
		log.Fatal(err.Error())
	}
	if len(sets) != 1 {
		log.Fatal("exactly 1 input set supported")
	}
	set := sets[0]
	// Part 1.
	fuelConsumptionFnPart1 := func(start, end int) int {
		return abs(start - end)
	}
	bestCost, bestPos := overallCost(set, fuelConsumptionFnPart1)
	fmt.Printf("Best position is %d with cost %d\n", bestPos, bestCost)
	// Part 2. Same code, different cost formula.
	fuelConsumptionFnPart2 := func(start, end int) int {
		diff := abs(start - end)
		// This is the sum of the first diff natural numbers. This also nicely works for diff==0.
		return (diff * (diff + 1)) / 2 //nolint: gomnd
	}
	bestCost, bestPos = overallCost(set, fuelConsumptionFnPart2)
	fmt.Printf("Best position is %d with cost %d\n", bestPos, bestCost)
}

// end::solution[]
