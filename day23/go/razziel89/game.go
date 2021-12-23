package main

import (
	"fmt"
	"log"
)

const (
	a            = 'A'
	b            = 'B'
	c            = 'C'
	d            = 'D'
	costA        = 1
	costB        = 10
	costC        = 100
	costD        = 1000
	kindA        = 0
	kindB        = 1
	kindC        = 2
	kindD        = 3
	firstHallIdx = -1
	lastHallIdx  = 11
	firstRoomIdx = 11
	// Assume there are at least 2 possible moves per pod. This number influences performance a bit.
	buffer = 16
)

// I'll try to keep this structure as copyable as possible. That way, I can simply put it on a stack
// to remember the old state, instead of reverting to an old state.
type game struct {
	pods   [8]pod
	spaces [19]space
}

func getChar(pods [8]pod, pos int) rune {
	for _, p := range pods {
		if p.pos == pos {
			switch p.cost {
			case costA:
				return a
			case costB:
				return b
			case costC:
				return c
			case costD:
				return d
			default:
				log.Fatal("internal error")
			}
		}
	}
	return '.'
}

func (g game) pretty() string {
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
	str += fmt.Sprintf(
		"  #%c#%c#%c#%c#\n",
		roomChars[1], roomChars[3], roomChars[5], roomChars[7],
	)

	str += "  #########"
	return str
}

//nolint:funlen
func newGame(inPods [8]rune) game {

	// Build the game board first.

	// The game board consists of all rooms and the hall, 19 spaces in total.
	spaces := [19]space{}

	// allowed := []rune{a, b, c, d}
	// These are the spaces we can move to in the hall.
	hallIndces := []int{0, 1, 3, 5, 7, 9, 10}
	// These are the spaces we cannot move to in the hall, but they are still in the hall. We use
	// them to identify which positions are to the top left or right of a room.
	aboveSpaces := []int{2, 4, 6, 8}

	// // Everyone is allowed in the hall.
	// allAllowed := map[rune]struct{}{
	// 	a: struct{}{},
	// 	b: struct{}{},
	// 	c: struct{}{},
	// 	d: struct{}{},
	// }
	// We implicitly know that anyone may move to the hall. Don't track that here.
	// for _, hallIdx := range hallIndces {
	// 	spaces[hallIdx].allowed = allAllowed
	// }

	// Connect everything together. All rooms are connected to the hall and vice versa.
	// Iterate over those spaces that are at the beginning of a room.
	for roomIdx, roomStart := range []int{11, 13, 15, 17} {
		// roomAllowed := map[rune]struct{}{allowed[roomIdx]: struct{}{}}
		aboveRoom := aboveSpaces[roomIdx]
		// Build the room. Make sure to connect to the entire hall.
		for _, count := range []int{0, 1} {
			spaceIdx := roomStart + count
			roomSpace := &spaces[spaceIdx]
			// roomSpace.allowed = roomAllowed
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
	kinds := map[rune]int{a: kindA, b: kindB, c: kindC, d: kindD}
	podIdx := 0
	for _, roomStart := range []int{11, 13, 15, 17} {
		for _, count := range []int{0, 1} {
			spaceIdx := roomStart + count
			pods[podIdx].pos = spaceIdx
			pods[podIdx].cost = costs[inPods[podIdx]]
			pods[podIdx].kind = kinds[inPods[podIdx]]
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
	kind int
}

type space struct {
	moveIndices []int
	above       int
	room        bool
	// allowed     map[rune]struct{}
}

type move struct {
	start, end, cost int
}

// Determine possible moves. This is the crux of the entire thing.
//nolint:nestif,funlen
func (g game) moves() []move {
	moves := make([]move, 0, buffer)

	occupied := [19]bool{}
	for _, p := range g.pods {
		occupied[p.pos] = true
	}

	for _, p := range g.pods {
		mySpace := g.spaces[p.pos]
		if mySpace.room {
			// If we are in a room, we can only move to the hall, but not in some cases.
			if (p.pos-firstRoomIdx)/2 == p.kind {
				// If we already are in our room, we don't want to move anymore.
				continue
			}
			if p.pos%2 == 0 {
				// This is the bottom space of a room. If there is someone above us, we cannot move.
				topSpace := p.pos - 1
				if occupied[topSpace] {
					// If the top space is occupied, we don't have any moves.
					continue
				}
			}
			// Find the largest occupied space in the hall smaller than our "above" space. We cannot
			// move past it. Do the same for the smallest one larger than our above space.
			// For now, assume the entire hall is free to the left.
			left := firstHallIdx
			for spaceIdx := firstHallIdx + 1; spaceIdx < mySpace.above; spaceIdx++ {
				// Find the largest occupied space to the left of our above space.
				if occupied[spaceIdx] {
					left = spaceIdx
				}
			}
			// For now, assume the entire hall is free to the right.
			right := lastHallIdx
			for spaceIdx := lastHallIdx - 1; spaceIdx > mySpace.above; spaceIdx-- {
				// Find the smallest occupied space to the right of our above space.
				if occupied[spaceIdx] {
					right = spaceIdx
				}
			}
			// Add all moves between left (exclusive) and right (exclusive) that are not also an
			// above space. That is, iterate over the hall and check whether we are at least left or
			// at most right.
			for _, pos := range []int{0, 1, 3, 5, 7, 9, 10} {
				if pos > left && pos < right {
					m := move{
						start: p.pos,
						end:   pos,
						cost:  p.cost,
					}
					// Anyone is allowed in the hall. Don't check if we are allowed.
					moves = append(moves, m)
				}
			}
		} else {
			// If we are in the hall, we can only move to a room. We can only move to our room,
			// though.
			for roomIdx, roomStart := range []int{11, 13, 15, 17} {
				if p.kind == roomIdx {
					// In this case, we are allowed in the room.
					if occupied[roomStart] {
						// In this case, the first spot in the room is occupied. We are not allowed
						// in at all.
						continue
					}
					// The fiirst space is free, we are allowed in.
					m := move{
						start: p.pos,
						end:   roomStart,
						cost:  p.cost,
					}
					moves = append(moves, m)
					// If the second spaceis free, we are also allowed in.
					if !occupied[roomStart+1] {
						m := move{
							start: p.pos,
							end:   roomStart + 1,
							cost:  p.cost,
						}
						moves = append(moves, m)
					}
				}
			}
		}
	}

	return moves
}
