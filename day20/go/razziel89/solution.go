package main

import (
	"fmt"
	"log"
)

func main() {
	algo, grid, err := ReadLinesAsPixelGrid()
	if err != nil {
		log.Fatal(err.Error())
	}
	fmt.Println(algo)
	fmt.Println()
	fmt.Println(grid.Pretty())
	fmt.Println(grid)
}
