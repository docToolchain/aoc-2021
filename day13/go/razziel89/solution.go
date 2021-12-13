// Package main is the main executable for razziel89's solution for this day's advent.
package main

import (
	"fmt"
	"log"
	"os"
	"strings"
)

const (
	prettyPrintEnvVar = "PRETTY_PRINT"
)

// tag::solution[]

func foldPoint(point Vec, ins Instruction) Vec {
	// Assume the point is unaffected.
	newPoint := point
	if ins.Dir == "x" && point.x > ins.Val {
		// Affected by fold.
		newPoint = Vec{
			x: ins.Val - (point.x - ins.Val),
			y: point.y,
		}
	} else if ins.Dir == "y" && point.y > ins.Val {
		// Affected by fold.
		newPoint = Vec{
			x: point.x,
			y: ins.Val - (point.y - ins.Val),
		}
	} else if !strings.Contains("xy", ins.Dir) {
		// Cannot reach here, so fatalling is OK if we do.
		log.Fatal("internal error while folding")
	}
	return newPoint
}

func fold(grid Grid, ins Instruction) (Grid, error) {
	result := Grid{}
	if !strings.Contains("xy", ins.Dir) {
		return Grid{}, fmt.Errorf("internal error")
	}
	// Check that no point is on the fold line.
	for p := range grid.Points() {
		if (ins.Dir == "x" && p.x == ins.Val) || (ins.Dir == "y" && p.y == ins.Val) {
			return Grid{}, fmt.Errorf("point on fold line")
		}
	}
	// Fold the points.
	for p := range grid.Points() {
		newPoint := foldPoint(p, ins)
		err := result.Mark(newPoint, 1)
		if err != nil {
			return Grid{}, err
		}
	}
	return result, nil
}

//nolint: funlen
func main() {
	prettyPrint := os.Getenv(prettyPrintEnvVar) == "1"
	grid, ins, err := ReadLinesAsGridAndInstructions()
	if err != nil {
		log.Fatal(err.Error())
	}
	if prettyPrint {
		fmt.Println(grid.Pretty(1))
	}
	for _, instruct := range ins {
		folded, err := fold(grid, instruct)
		if err != nil {
			log.Fatal(err.Error())
		}
		fmt.Printf("There are %d marked points.\n", len(folded))
		if prettyPrint {
			fmt.Println(folded.Pretty(1))
		}
		// err = folded.ReduceMarked(1)
		// if err != nil {
		// 	log.Fatal(err.Error())
		// }
		grid = folded
	}
	fmt.Println()
	// We need this in the end even though generating it might take long.
	fmt.Println(grid.Pretty(1)) //nolint:gomnd
}

// end::solution[]
