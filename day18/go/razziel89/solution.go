// Package main is the main executable for razziel89's solution for this day's advent.
package main

import (
	"log"
)

// tag::solution[]

//nolint: funlen
func main() {
	lines, err := ReadLinesAsNumbers()
	if err != nil {
		log.Fatal(err.Error())
	}
}

// end::solution[]
