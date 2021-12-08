// Package main is the main executable for razziel89's solution for this day's advent.
package main

import (
	"fmt"
	"log"
	"os"
)

// tag::solution[]

const (
	initialDaysToBreed = 9
	daysToBreed        = 7
)

var (
	partChoice = os.Getenv("PART")
)

//nolint: funlen
func main() {
	var days int
	if partChoice == "1" {
		days = 80
	} else {
		days = 256
	}
	sets, err := ReadLinesAsSets()
	if err != nil {
		log.Fatal(err.Error())
	}
	if len(sets) != 1 {
		log.Fatal("exactly 1 population supported")
	}
	population, err := NewPopulation(daysToBreed, initialDaysToBreed)
	if err != nil {
		log.Fatal(err.Error())
	}
	population.PopulateFromSet(sets[0])
	population.Age(days)
	fmt.Printf("There are %d fish after %d days.\n", population.Size(), days)
}

// end::solution[]
