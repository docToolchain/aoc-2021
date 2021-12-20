package main

import (
	"fmt"
	"log"
)

const (
	numConvPart1     = 2
	numConvPart2     = 50
	startBorderPart1 = 4
	startBorderPart2 = 52
)

func main() {
	algo, grid, err := ReadLinesAsPixelGrid()
	if err != nil {
		log.Fatal(err.Error())
	}
	// fmt.Println(grid)

	gridP1 := grid
	gridP2 := grid

	// Convert image twice for part 1. Take care of the background!
	for count := 0; count < numConvPart1; count++ {
		// fmt.Println(gridP1.Pretty(startBorderPart1 - count))
		gridP1 = gridP1.Convert(algo)
	}

	// Convert image twice for part 1. Take care of the background!
	for count := 0; count < numConvPart2; count++ {
		// fmt.Println(gridP2.Pretty(startBorderPart2 - count))
		gridP2 = gridP2.Convert(algo)
	}
	fmt.Println("Part 2, image")
	fmt.Println(gridP2.Pretty(startBorderPart2 - numConvPart2))
	fmt.Println("Part 1, image")
	fmt.Println(gridP1.Pretty(startBorderPart1 - numConvPart1))

	fmt.Println("Part 2, solution")
	fmt.Println(len(gridP2.data))
	fmt.Println("Part 1, solution")
	fmt.Println(len(gridP1.data))
}
