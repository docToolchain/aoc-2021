package main

import (
	"fmt"
)

func main() {
	// Actual riddle.
	// g := newGame([8]rune{'D', 'C', 'D', 'A', 'B', 'B', 'A', 'C'})
	g := newGame([8]rune{'B', 'A', 'C', 'D', 'B', 'C', 'D', 'A'})
	// g := newGame([8]rune{'','','','','','','',''})
	fmt.Println(g.pretty())
	fmt.Println(g.moves())
}
