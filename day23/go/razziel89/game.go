package main

import (
	"fmt"
	"log"
)

const (
	a     = 'A'
	b     = 'B'
	c     = 'C'
	d     = 'D'
	costA = 1
	costB = 10
	costC = 100
	costD = 1000
)

// I'll try to keep this structure as copyable as possible. That way, I can simply put it on a stack
// to remember the old state, instead of reverting to an old state.
type game struct {
	pods   [8]pod
	spaces [19]space
}

func getChar(pods [8]pod, pos int) rune {
	char := '.'
	for _, p := range pods {
		if p.pos == pos {
			switch p.cost {
			case costA:
				return a
			case costB:
				return b
			case costC:
				return b
			case costD:
				return d
			default:
				log.Fatal("internal error")
			}
		}
	}
	return char
}

func (g *game) pretty() string {
	str := "#############\n"
	str += "#"
	for pos := 0; pos < 11; pos++ {
		str += string(getChar(g.pods, pos))
	}
	str += "#\n"
	roomChars := [8]rune{}
	for pos := 11; pos < 19; pos++ {
		char := getChar(g.pods, pos)
		roomChars[pos-11] = char
	}
	str += fmt.Sprintf(
		"###%c#%c#%c#%c###\n",
		roomChars[0], roomChars[2], roomChars[4], roomChars[6],
	)
	str += fmt.Sprintf("  #%c#%c#%c#%c#\n", roomChars[1], roomChars[3], roomChars[5], roomChars[7])

	str += "  #########"
	return str
}

//nolint:funlen
func newGame(inPods [8]rune) game {

	// Build the game board first.

	// The game board consists of all rooms and the hall, 19 spaces in total.
	spaces := [19]space{}

	allowed := []rune{a, b, c, d}
	// These are the spaces we can move to in the hall.
	hallIndces := []int{0, 1, 3, 5, 7, 9, 10}
	// These are the spaces we cannot move to in the hall, but they are still in the hall. We use
	// them to identify which positions are to the top left or right of a room.
	aboveSpaces := []int{2, 4, 6, 8}

	// Everyone is allowed in the hall.
	allAllowed := map[rune]struct{}{
		a: struct{}{},
		b: struct{}{},
		c: struct{}{},
		d: struct{}{},
	}
	for _, hallIdx := range hallIndces {
		spaces[hallIdx].allowed = allAllowed
	}

	// Connect everything together. All rooms are connected to the hall and vice versa.
	// Iterate over those spaces that are at the beginning of a room.
	for roomIdx, roomStart := range []int{11, 13, 15, 17} {
		roomAllowed := map[rune]struct{}{allowed[roomIdx]: struct{}{}}
		aboveRoom := aboveSpaces[roomIdx]
		// Build the room. Make sure to connect to the entire hall.
		for _, count := range []int{0, 1} {
			spaceIdx := roomStart + count
			roomSpace := &spaces[spaceIdx]
			roomSpace.allowed = roomAllowed
			roomSpace.above = aboveRoom
			roomSpace.room = true
			// Build the connections to the hall.
			for _, hallIdx := range hallIndces {
				roomSpace.moveIndices = append(roomSpace.moveIndices, hallIdx)
				hallSpace := &spaces[hallIdx]
				hallSpace.moveIndices = append(hallSpace.moveIndices, spaceIdx)
			}
		}
	}

	// Build the pods.
	pods := [8]pod{}
	costs := map[rune]int{a: costA, b: costB, c: costC, d: costD}
	podIdx := 0
	for _, roomStart := range []int{11, 13, 15, 17} {
		for _, count := range []int{0, 1} {
			spaceIdx := roomStart + count
			pods[podIdx].pos = spaceIdx
			pods[podIdx].cost = costs[inPods[podIdx]]
			podIdx++
		}
	}

	return game{
		spaces: spaces,
		pods:   pods,
	}
}

type pod struct {
	pos  int
	cost int
}

type space struct {
	moveIndices []int
	above       int
	room        bool
	allowed     map[rune]struct{}
}
