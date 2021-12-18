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
	sum := numbers[0]
	fmt.Println(sum)
	sum = Reduce(sum)
	fmt.Println(sum)
	for _, addMe := range numbers[1:] {
		sum = Add(sum, addMe)
		fmt.Println(sum)
		sum = Reduce(sum)
		fmt.Println(sum)
	}
}

// end::solution[]
