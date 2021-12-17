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

	// Declare here to allow recursion.
	var addVersionsUp func([]Package) int

	addVersionsUp = func(packages []Package) int {
		result := 0
		for _, pkg := range packages {
			result += pkg.Version()
			result += addVersionsUp(pkg.SubPackages())
		}
		return result
	}

	totalVersion := addVersionsUp(parsed)

	fmt.Println("Sum of all version numbers is", totalVersion)
}

// end::solution[]
