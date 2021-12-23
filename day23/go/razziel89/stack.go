package main

// tag::stack[]

// StackedGame tracks a game on the stack. It also contains the total cost for all moves until that
// point.
type StackedGame struct {
	game game
	cost int
	path int
}

// Stack is a FIFO.
type Stack []StackedGame

// Push puts a value on the stack.
func (s *Stack) Push(g game, cost, path int) {
	stackElem := StackedGame{
		game: g,
		cost: cost,
		path: path,
	}
	*s = append(*s, stackElem)
}

// Pop removes the topmost value and returns it. Also returns whether the stack was non-empty.
func (s *Stack) Pop() (game, int, int) {
	l := len(*s)
	if l == 0 {
		return game{}, 0, 0
	}
	val := (*s)[l-1]
	// Don't cut the capacity. A game can be copied, so we pemit overwriting it.
	*s = (*s)[0 : len(*s)-1]
	return val.game, val.cost, val.path
}

// Peek returns the top-most entry.
func (s *Stack) Peek() *StackedGame {
	if len(*s) == 0 {
		return &StackedGame{}
	}
	return &(*s)[len(*s)-1]
}

// end::stack[]
