// Package main is the main executable for razziel89's solution for this day's advent.
package main

import (
	"fmt"
	"log"
)

// tag::solution[]

//nolint: funlen
func main() {
	// Read input.
	picks, boards, err := ReadLinesAsPicksOrBoards()
	if err != nil {
		log.Fatalf("cannot read in: %s", err.Error())
	}
	var winner Board
	for _, pick := range picks {
		for _, board := range boards {
			fmt.Println(board.Pretty())
			if board.Mark(pick) && board.Score() > 0 {
				winner = board
			}
		}
		fmt.Println()
	}
	if score := winner.Score(); score > 0 {
		fmt.Printf("Winner follows, winning score is %d\n", score)
		fmt.Println(winner.Pretty())
	} else {
		log.Fatal("no winner found")
	}
}

// end::solution[]
