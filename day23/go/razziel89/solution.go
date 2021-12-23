package main

import (
	"fmt"
)

const (
	numGames = 1000000
)

//nolint:nestif
func main() {
	// Actual riddle.
	g := newGame([8]rune{'D', 'C', 'D', 'A', 'B', 'B', 'A', 'C'})
	// Example.
	//g := newGame([8]rune{'B', 'A', 'C', 'D', 'B', 'C', 'D', 'A'})
	// Initialise.
	stack := make(Stack, 0, numGames)

	cheapest := 0
	found := false

	trackedCost := 0
	path := 0

	count := -1

	for done := false; !done; {
		count++
		// fmt.Print(g.pretty())

		moves := g.moves()
		// fmt.Println(moves)
		if path < len(moves) {
			// There are still moves available. Save the game state.
			move := moves[path]
			path++
			stack.Push(g, trackedCost, path)
			// fmt.Printf("PUSH %d\n\n", count)
			path = 0
			trackedCost += move.cost
			g = g.update(move)
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
			g, trackedCost, path = stack.Pop()
			// fmt.Printf("POP %d\n\n", count)
		}
	}
	fmt.Println(cheapest)
}
