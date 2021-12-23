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
	kindFree     = 4
	firstHallIdx = -1
	lastHallIdx  = 11
	firstRoomIdx = 11
	kindToRoom   = 2
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

	str += "  #########\n"
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

	occupied := [19]int{}
	for idx := range occupied {
		occupied[idx] = kindFree
	}
	for _, p := range g.pods {
		occupied[p.pos] = p.kind
	}
PODLOOP:
	for _, p := range g.pods {
		mySpace := g.spaces[p.pos]
		if mySpace.room {
			// If we are in a room, we can only move to the hall, but not in some cases.
			if (p.pos-firstRoomIdx)/kindToRoom == p.kind {
				// If we already are in our room, we don't want to move anymore.
				continue PODLOOP
			}
			if p.pos%2 == 0 {
				// This is the bottom space of a room. If there is someone above us, we cannot move.
				topSpace := p.pos - 1
				if occupied[topSpace] != kindFree {
					// If the top space is occupied, we don't have any moves.
					continue PODLOOP
				}
			}
			// Find the largest occupied space in the hall smaller than our "above" space. We cannot
			// move past it. Do the same for the smallest one larger than our above space.
			// For now, assume the entire hall is free to the left.
			left := firstHallIdx
			for spaceIdx := firstHallIdx + 1; spaceIdx < mySpace.above; spaceIdx++ {
				// Find the largest occupied space to the left of our above space.
				if occupied[spaceIdx] != kindFree {
					left = spaceIdx
				}
			}
			// For now, assume the entire hall is free to the right.
			right := lastHallIdx
			for spaceIdx := lastHallIdx - 1; spaceIdx > mySpace.above; spaceIdx-- {
				// Find the smallest occupied space to the right of our above space.
				if occupied[spaceIdx] != kindFree {
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
			ourRoom := firstRoomIdx + kindToRoom*p.kind
			aboveOurRoom := g.spaces[ourRoom].above
			// If a space on our side of the hall of the above space is occupied, we cannot move to
			// our room.
			if p.pos < aboveOurRoom {
				// We are to the left of the above position.
				for checkPos := p.pos + 1; checkPos < aboveOurRoom; checkPos++ {
					if occupied[checkPos] != kindFree {
						// There is a space occupied to our right, blocking our room. We cannot move
						// into our room.
						continue PODLOOP
					}
				}
			} else {
				// We cannot be in an above position. Hence, we are to the right of the above spot.
				for checkPos := p.pos - 1; checkPos > aboveOurRoom; checkPos-- {
					if occupied[checkPos] != kindFree {
						// There is a space occupied to our left, blocking our room. We cannot move
						// into our room.
						continue PODLOOP
					}
				}
			}
			if occupied[ourRoom] != kindFree {
				// In this case, the first spot in the room is occupied. We are not allowed
				// in at all.
				continue PODLOOP
			}
			// The first space is free, we are allowed in, but only if the last space is
			// free or occupied by one of the same kind.
			if occupied[ourRoom+1] == kindFree {
				// If the second space is free, we are only allowed in there since we don't want to
				// block the room for our comrade by standing on the first spot.
				m := move{
					start: p.pos,
					end:   ourRoom + 1,
					cost:  p.cost,
				}
				moves = append(moves, m)
			} else if occupied[ourRoom+1] == p.kind {
				// If the last space is occupied by one of our kind, we are allowed in.
				m := move{
					start: p.pos,
					end:   ourRoom,
					cost:  p.cost,
				}
				moves = append(moves, m)
			}
		}
	}

	return moves
}

func (g *game) getPod(pos int) *pod {
	for posIdx := range g.pods {
		if g.pods[posIdx].pos == pos {
			return &g.pods[posIdx]
		}
	}
	return nil
}

func (g *game) isFinal(int) bool {
	occupied := [19]int{}
	for idx := range occupied {
		occupied[idx] = kindFree
	}
	for _, p := range g.pods {
		occupied[p.pos] = p.kind
	}
	return occupied[11] == kindA &&
		occupied[12] == kindA &&
		occupied[13] == kindB &&
		occupied[14] == kindB &&
		occupied[15] == kindC &&
		occupied[16] == kindC &&
		occupied[17] == kindD &&
		occupied[18] == kindD
}

func (p pod) String() string {
	var name rune
	switch p.kind {
	case kindA:
		name = a
	case kindB:
		name = b
	case kindC:
		name = c
	case kindD:
		name = d
	default:
		log.Fatal("internal error")
	}
	return fmt.Sprintf("{k: %c, p: %d, c: %d}", name, p.pos, p.cost)
}
