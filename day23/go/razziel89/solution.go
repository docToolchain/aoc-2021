package main

import "fmt"

func main() {
	g := newGame([8]rune{'B', 'A', 'C', 'D', 'B', 'C', 'D', 'A'})
	// g := newGame([8]rune{'','','','','','','',''})
	fmt.Println(g.pretty())
}
