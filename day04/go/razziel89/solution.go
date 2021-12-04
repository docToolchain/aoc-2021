// Package main is the main executable for razziel89's solution for this day's advent.
package main

import (
	"fmt"
	"log"
)

// tag::solution[]

func findWinner(picks []int, boards []Board) (Board, error) {
	for _, pick := range picks {
		for boardIdx := range boards {
			board := &boards[boardIdx]
			if board.Mark(pick) && board.Score() > 0 {
				return *board, nil
			}
		}
	}
	return Board{}, fmt.Errorf("no winner found")
}

//nolint: funlen
func main() {
	// Read input.
	picks, boards, err := ReadLinesAsPicksOrBoards()
	if err != nil {
		log.Fatalf("cannot read in: %s", err.Error())
	}
	winner, err := findWinner(picks, boards)
	if err != nil {
		log.Fatal(err.Error())
	}
	fmt.Printf("Winner follows, winning score is %d\n", winner.Score())
	fmt.Println(winner.Pretty())
}

// end::solution[]
