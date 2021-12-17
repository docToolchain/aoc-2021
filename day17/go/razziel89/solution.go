// Package main is the main executable for razziel89's solution for this day's advent.
package main

import (
	"fmt"
	"log"
)

// tag::solution[]

//nolint: funlen
func main() {
	areas, err := ReadLinesAsAreas()
	if err != nil {
		log.Fatal(err.Error())
	}
	if len(areas) != 1 {
		log.Fatal("only one area expected")
	}
	area := areas[0]
	fmt.Println(area)
}

// end::solution[]
