package main

// tag::stack[]

// StackedGame tracks a game on the stack. It also contains the total cost for all moves until that
// point. It also tracks the available moves for that game as well as a counter called `path` that
// indicates which move index next to try. Thus, if path>=len(moves), we have exhausted all possible
// moves.
type StackedGame struct {
	game  game
	cost  int
	path  int
	moves []move
}

// Stack is a FIFO.
type Stack []StackedGame

// Push puts a value on the stack.
func (s *Stack) Push(g game, cost, path int, moves []move) {
	stackElem := StackedGame{
		game:  g,
		cost:  cost,
		path:  path,
		moves: moves,
	}
	*s = append(*s, stackElem)
}

// Pop removes the topmost value and returns it. Also returns whether the stack was non-empty.
func (s *Stack) Pop() (game, int, int, []move) {
	l := len(*s)
	if l == 0 {
		return game{}, 0, 0, []move{}
	}
	val := (*s)[l-1]
	// Prevent memory leak.
	(*s)[l-1] = StackedGame{}
	// Don't cut the capacity. A game can be copied, so we pemit overwriting it.
	*s = (*s)[0 : len(*s)-1]
	return val.game, val.cost, val.path, val.moves
}

// Peek returns the top-most entry.
func (s *Stack) Peek() *StackedGame {
	if len(*s) == 0 {
		return &StackedGame{}
	}
	return &(*s)[len(*s)-1]
}

// end::stack[]
