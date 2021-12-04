// Package main is the main executable for razziel89's solution for this day's advent.
package main

import (
	"fmt"
	"log"
)

// tag::solution[]

func findFirstWinner(picks []int, boards []Board) (Board, error) {
	for _, pick := range picks {
		for boardIdx := range boards {
			board := &boards[boardIdx]
			if board.Mark(pick) && board.Score() >= 0 {
				return *board, nil
			}
		}
	}
	return Board{}, fmt.Errorf("no winner found")
}

//nolint: funlen
func main() {
	// -1 means no score assigned yet.
	firstWinningScore, lastWinningScore := -1, -1
	// Read input.
	picks, boards, err := ReadLinesAsPicksOrBoards()
	if err != nil {
		log.Fatalf("cannot read in: %s", err.Error())
	}
	// Both parts.
	for len(boards) > 0 {
		winner, err := findFirstWinner(picks, boards)
		if err != nil {
			if len(boards) > 0 {
				log.Fatal(err.Error())
			} else {
				fmt.Println("All done")
				return
			}
		}
		score := winner.Score()
		if firstWinningScore < 0 {
			firstWinningScore = score
		} else {
			lastWinningScore = score
		}
		fmt.Printf("Next winner follows, winning score is %d\n", score)
		fmt.Println(winner.Pretty())
		// Remove all winners, find next winner, and repeat.
		newBoards := make([]Board, 0, len(boards))
		for _, board := range boards {
			if board.Score() < 0 {
				newBoards = append(newBoards, board)
			}
		}
		boards = newBoards
	}
	fmt.Printf(
		"All done, first (last) winning score is %d (%d).\n",
		firstWinningScore, lastWinningScore,
	)
}

// end::solution[]
