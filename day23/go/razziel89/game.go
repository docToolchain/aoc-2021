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
	kindToRoom   = 4
	// Assume there are at least 2 possible moves per situation. This number influences performance
	// a bit.
	buffer = 16
)

// I'll try to keep this structure as copyable as possible. That way, I can simply put it on a stack
// to remember the old state, instead of reverting to an old state.
type game struct {
	pods   [16]pod
	spaces [27]space
}

func getChar(pods [16]pod, pos int) rune {
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
	roomChars := [16]rune{}
	for pos := 11; pos < 27; pos++ {
		char := getChar(g.pods, pos)
		roomChars[pos-firstRoomIdx] = char
	}
	str += fmt.Sprintf(
		"###%c#%c#%c#%c###\n",
		roomChars[0], roomChars[4], roomChars[8], roomChars[12],
	)
	str += fmt.Sprintf(
		"  #%c#%c#%c#%c#\n",
		roomChars[1], roomChars[5], roomChars[9], roomChars[13],
	)
	str += fmt.Sprintf(
		"  #%c#%c#%c#%c#\n",
		roomChars[2], roomChars[6], roomChars[10], roomChars[14],
	)
	str += fmt.Sprintf(
		"  #%c#%c#%c#%c#\n",
		roomChars[3], roomChars[7], roomChars[11], roomChars[15],
	)

	str += "  #########\n"
	return str
}

//nolint:funlen
func newGame(inPods [16]rune) game {

	// Build the game board first.

	// The game board consists of all rooms and the hall, 27 spaces in total.
	spaces := [27]space{}

	// allowed := []rune{a, b, c, d}
	// These are the spaces we can move to in the hall.
	// hallIndces := []int{0, 1, 3, 5, 7, 9, 10}
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
	for roomIdx, roomStart := range []int{11, 15, 19, 23} {
		// roomAllowed := map[rune]struct{}{allowed[roomIdx]: struct{}{}}
		aboveRoom := aboveSpaces[roomIdx]
		// Build the room. Make sure to connect to the entire hall.
		for _, count := range []int{0, 1, 2, 3} {
			spaceIdx := roomStart + count
			roomSpace := &spaces[spaceIdx]
			// roomSpace.allowed = roomAllowed
			roomSpace.above = aboveRoom
			roomSpace.room = true
			// // Build the connections to the hall.
			// for _, hallIdx := range hallIndces {
			// 	// roomSpace.moveIndices = append(roomSpace.moveIndices, hallIdx)
			// 	hallSpace := &spaces[hallIdx]
			// 	// hallSpace.moveIndices = append(hallSpace.moveIndices, spaceIdx)
			// }
		}
	}

	// Build the pods.
	pods := [16]pod{}
	costs := map[rune]int{a: costA, b: costB, c: costC, d: costD}
	kinds := map[rune]int{a: kindA, b: kindB, c: kindC, d: kindD}
	podIdx := 0
	for _, roomStart := range []int{11, 15, 19, 23} {
		for _, count := range []int{0, 1, 2, 3} {
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
	// moveIndices []int
	above int
	room  bool
	// allowed     map[rune]struct{}
}

type move struct {
	start, end, cost int
}

func (m move) String() string {
	return fmt.Sprintf("[s: %d, e: %d, c: %d]", m.start, m.end, m.cost)
}

func abs(i int) int {
	if i < 0 {
		return -i
	}
	return i
}

// Determine possible moves. This is the crux of the entire thing.
//nolint:nestif,funlen
func (g game) moves() []move {
	moves := make([]move, 0, buffer)

	occupied := [27]int{}
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
			diffToRoomStart := p.pos - firstRoomIdx
			//nolint:gomnd
			if diffToRoomStart/kindToRoom == p.kind {
				// If we already are in our room, we don't want to move anymore, but only if the
				// other ones in our room are also of our kind.
				// If we are at the bottom, we are good to go.
				if diffToRoomStart%4 == 3 {
					continue PODLOOP
				}
				// If we are somewhere at the top, we still have to move if the ones below us are of
				// a different kind. But if the ones below us is of the same kind, we don't want to
				// move anymore.
				if diffToRoomStart%4 == 2 {
					if occupied[p.pos+1] == p.kind {
						continue PODLOOP
					}
				}
				if diffToRoomStart%4 == 1 {
					if occupied[p.pos+1] == p.kind && occupied[p.pos+2] == p.kind {
						continue PODLOOP
					}
				}
				if diffToRoomStart%4 == 0 {
					if occupied[p.pos+1] == p.kind && occupied[p.pos+2] == p.kind && occupied[p.pos+3] == p.kind {
						continue PODLOOP
					}
				}
			}
			if diffToRoomStart%kindToRoom > 0 {
				// These are the bottom spaces of a room. If there is someone above us, we cannot
				// move.
				topSpace := kindToRoom * ((p.pos - firstRoomIdx) / kindToRoom)
				// Only check spaces above us.
				for checkSpace := firstRoomIdx + topSpace; checkSpace < p.pos; checkSpace++ {
					if occupied[checkSpace] != kindFree {
						// If any of the spaces above us are occupied, we don't have any moves.
						continue PODLOOP
					}
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
					// The number of spacs moved is the distance between the target position and the
					// above space, plus distanceToAbove, which is hte distance to the space
					// directly above our room.
					distanceToAbove := diffToRoomStart%kindToRoom + 1
					spacesMoved := abs(pos-mySpace.above) + distanceToAbove
					m := move{
						start: p.pos,
						end:   pos,
						cost:  p.cost * spacesMoved,
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
			// Check whether any space already in the room is occupied. We may only move to the
			// lowest unoccupied space. But only if there are only the same kind of pods there like
			// us.
			diffToTop := kindToRoom
			bottomSpace := ourRoom + kindToRoom - 1
			// This block uses the implicit knowledge that, if we find a free space, all spaces
			// above it must be free, too, since all rooms start out full.
			for space := bottomSpace; space >= ourRoom; space-- {
				if occupied[space] == kindFree {
					spacesMoved := abs(p.pos-aboveOurRoom) + diffToTop //nolint:gomnd
					m := move{
						start: p.pos,
						end:   space,
						cost:  p.cost * spacesMoved,
					}
					moves = append(moves, m)
					continue PODLOOP
				}
				if occupied[space] != p.kind {
					// If we find anyone else here, we must not enter the room. Since we've already
					// taken care of the empty case, this one means we cannot move into the room.
					continue PODLOOP
				}
				diffToTop--
			}
		}
	}

	return moves
}

// func (g *game) getPod(pos int) *pod {
// 	for posIdx := range g.pods {
// 		if g.pods[posIdx].pos == pos {
// 			return &g.pods[posIdx]
// 		}
// 	}
// 	return nil
// }

func (g game) update(m move) game {
	mover := -1
	for moverIdx, p := range g.pods {
		if p.pos == m.start {
			mover = moverIdx
			break
		}
	}
	if mover < 0 {
		fmt.Println(g.pretty())
		fmt.Println(m)
		fmt.Println(g.pods)
		log.Fatal("invalid move, no mover")
	}
	// Sanity check, try to find something at the target position.
	for _, p := range g.pods {
		if p.pos == m.end {
			log.Fatal("invalid move, target occupied")
		}
	}
	g.pods[mover].pos = m.end
	return g
}

func (g *game) isFinal() bool {
	occupied := [27]int{}
	for idx := range occupied {
		occupied[idx] = kindFree
	}
	for _, p := range g.pods {
		occupied[p.pos] = p.kind
	}
	occ := occupied[11] == kindA &&
		occupied[12] == kindA &&
		occupied[13] == kindA &&
		occupied[14] == kindA &&
		occupied[15] == kindB &&
		occupied[16] == kindB &&
		occupied[17] == kindB &&
		occupied[18] == kindB &&
		occupied[19] == kindC &&
		occupied[20] == kindC &&
		occupied[21] == kindC &&
		occupied[22] == kindC &&
		occupied[23] == kindD &&
		occupied[24] == kindD &&
		occupied[25] == kindD &&
		occupied[26] == kindD
	return occ
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
