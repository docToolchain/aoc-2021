// Package main is the main executable for razziel89's solution for this day's advent.
package main

import (
	"fmt"
	"log"
)

// tag::solution[]

//nolint: funlen
func main() {
	numbers, err := ReadLinesAsNumbers()
	if err != nil {
		log.Fatal(err.Error())
	}
	if len(numbers) < 2 { //nolint:gomnd
		log.Fatal("we were promised some numbers")
	}
	fmt.Println("Part 1")
	sum := numbers[0].Copy()
	sum = Reduce(sum)
	for _, addMe := range numbers[1:] {
		sum = Add(sum, addMe)
		sum = Reduce(sum)
	}
	fmt.Println(sum)
	fmt.Println(sum.Magnitude())

	fmt.Println("Part 2")
	maxMagnitude := 0
	for leftIdx, left := range numbers {
		for rightIdx, right := range numbers {
			if leftIdx == rightIdx {
				// We cannot add a number to itself here.
				continue
			}
			sum := Add(left, right)
			sum = Reduce(sum)
			magnitude := sum.Magnitude()
			if magnitude > maxMagnitude {
				maxMagnitude = magnitude
			}
			sum = Add(right, left)
			sum = Reduce(sum)
			magnitude = sum.Magnitude()
			if magnitude > maxMagnitude {
				maxMagnitude = magnitude
			}
		}
	}
	fmt.Println(maxMagnitude)
}

// end::solution[]
