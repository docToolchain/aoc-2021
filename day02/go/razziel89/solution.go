// Package main is the main executable for razziel89's solution for this day's advent.
package main

import (
	"fmt"
	"log"
)

// tag::main[]

func main() {
	movements, err := ReadLinesAsStates()
	if err != nil {
		log.Fatalf("cannot read movement values from stdin due to %v", err.Error())
	}
	pos := State{Disp: 0, Depth: 0}
	for _, mov := range movements {
		pos = pos.Displace(mov)
	}
	fmt.Printf("Area under total movement is %d\n", pos.Area())
}

// end::main[]
