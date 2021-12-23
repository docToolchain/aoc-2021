package main

import (
	"fmt"
)

const (
	numGames = 1000000
)

// This solution really isn't that efficient, but it can find the solution for each example in under
// 10min and for each actual puzzle in way under an hour. That's fine by me.

//nolint:nestif,funlen
func main() {
	// Part 1 stuff has been augmented to fit into the part 2 board by adding pieces that will never
	// leave their rooms to the very bottom of their destination rooms. Uncomment each array to run.
	// // Actual riddle part 1.
	// g := newGame(
	// 	[16]rune{'D', 'C', 'A', 'A', 'D', 'A', 'B', 'B', 'B', 'B', 'C', 'C', 'A', 'C', 'D', 'D'},
	// )
	// // Example part 1.
	// g := newGame(
	// 	[16]rune{'B', 'A', 'A', 'A', 'C', 'D', 'B', 'B', 'B', 'C', 'C', 'C', 'D', 'A', 'D', 'D'},
	// )
	// Actual riddle part 2.
	g := newGame(
		[16]rune{'D', 'D', 'D', 'C', 'D', 'C', 'B', 'A', 'B', 'B', 'A', 'B', 'A', 'A', 'C', 'C'},
	)
	// Example part 2.
	// fmt.Println("Example 2")
	// g := newGame(
	// 	[16]rune{'B', 'D', 'D', 'A', 'C', 'C', 'B', 'D', 'B', 'B', 'A', 'C', 'D', 'A', 'C', 'A'},
	// )
	// Initialise. Allocate much since we don't know how deep we need to iterate. This uses about
	// 1.5GB of RAM.
	stack := make(Stack, 0, numGames)
	fmt.Println(g.pretty())

	cheapest := 0
	found := false
	popped := false

	var moves []move

	trackedCost := 0
	path := 0

	count := -1

	for done := false; !done; {
		count++

		if !popped {
			moves = g.moves()
		}
		// fmt.Println(moves)
		if path < len(moves) {
			// There are still moves available. Save the game state.
			move := moves[path]
			path++
			stack.Push(g, trackedCost, path, moves)
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
					// Print out the entire solution.
					fmt.Println(cheapest, "==============================")
					for _, gam := range stack {
						fmt.Println(gam.game.pretty())
						fmt.Println(gam.cost)
						fmt.Println(gam.moves[gam.path-1])
					}
					fmt.Println(g.pretty())
					fmt.Println(cheapest)
				}
			}
			if len(stack) == 0 {
				// We have reached the end. No more moves and the stack is empty.
				break
			}
			g, trackedCost, path, moves = stack.Pop()
			popped = true
		}
	}
	fmt.Println(cheapest)
}
