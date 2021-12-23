package main

import (
	"fmt"
)

const (
	numGames = 1000000
)

//nolint:nestif,funlen
func main() {
	// // Actual riddle part 1.
	// g := newGame(
	// 	[16]rune{'D', 'C', 'A', 'A', 'D', 'A', 'B', 'B', 'B', 'B', 'C', 'C', 'A', 'C', 'D', 'D'},
	// )
	// Example part 1.
	g := newGame(
		[16]rune{'B', 'A', 'A', 'A', 'C', 'D', 'B', 'B', 'B', 'C', 'C', 'C', 'D', 'A', 'D', 'D'},
	)
	// // Actual riddle part 2.
	// g := newGame(
	// 	[16]rune{'D', 'D', 'D', 'C', 'D', 'C', 'B', 'A', 'B', 'B', 'A', 'B', 'A', 'A', 'C', 'C'},
	// )
	// // Example part 2.
	// g := newGame(
	// 	[16]rune{'B', 'D', 'D', 'A', 'C', 'C', 'B', 'D', 'B', 'B', 'A', 'C', 'D', 'A', 'C', 'A'},
	// )
	// Initialise.
	stack := make(Stack, 0, numGames)

	cheapest := 0
	found := false
	popped := false

	var moves []move

	trackedCost := 0
	path := 0

	count := -1

	for done := false; !done; {
		count++
		fmt.Println(g.pretty())

		if !popped {
			moves = g.moves()
		}
		// fmt.Println(moves)
		if path < len(moves) {
			// There are still moves available. Save the game state.
			move := moves[path]
			path++
			stack.Push(g, trackedCost, path, moves)
			// fmt.Printf("PUSH %d\n\n", count)
			path = 0
			trackedCost += move.cost
			g = g.update(move)
			popped = false
		} else {
			// There are no more moves available. Check whether this is final and pop the last
			// element and continue from there.
			if g.isFinal() {
				if !found || trackedCost < cheapest {
					found = true
					cheapest = trackedCost
					fmt.Println(cheapest)
				}
			}
			if len(stack) == 0 {
				// We have reached the end. No more moves and the stack is empty.
				break
			}
			g, trackedCost, path, moves = stack.Pop()
			popped = true
			// fmt.Printf("POP %d\n\n", count)
		}
	}
	fmt.Println(cheapest)
}
