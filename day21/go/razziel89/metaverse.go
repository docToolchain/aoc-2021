package main

import (
	"fmt"
)

// tag::metaverse[]

// Game describes the state of one two-player game.
type Game struct {
	Pos1, Pos2     int
	Score1, Score2 int
}

const (
	diracSize     = 3
	winScoreDirac = 21
)

// Metaverse tracks how many universes there are for each game state.
type Metaverse map[Game]int

// I'm too lazy to write out all possible rolls and counts when rolling a 3-sided die 3 times in a
// row. Hence, I compute that once.
func dieRolls(diracSize, numRolls int) map[int]int {
	state := map[int]int{1: 1, 2: 1, 3: 1}
	for rollIdx := 1; rollIdx < numRolls; rollIdx++ {
		newState := map[int]int{}
		for dieRoll := 1; dieRoll <= diracSize; dieRoll++ {
			for val, count := range state {
				newState[val+dieRoll] += count
			}
		}
		state = newState
	}
	return state
}

// We always assume it's player 1's turn. But we swap players so that each has their turn after the
// other.
func updateMetaverse(meta Metaverse, dieRolls map[int]int, boardSize int) Metaverse {
	newMeta := Metaverse{}
	for game, gameCount := range meta {
		// Deconstruct the game data.
		pos1 := game.Pos1
		pos2 := game.Pos2
		score1 := game.Score1
		score2 := game.Score2
		// Update the new metaverse for each possible outcome.
		for roll, rollCount := range dieRolls {
			newPos := (pos1+roll-1)%boardSize + 1
			newGame := Game{
				Pos1:   pos2,
				Pos2:   newPos,
				Score1: score2,
				Score2: score1 + newPos,
			}
			newMeta[newGame] += gameCount * rollCount
		}
	}
	return newMeta
}

func countAndRemoveWins(meta *Metaverse) int {
	wins := 0
	for game, count := range *meta {
		// The score that has last been updated will always be score 2 due to the fact that we swap
		// players all the time.
		if game.Score2 >= winScoreDirac {
			wins += count
			// Deleting from a map while iterating over it is not a problem in Go.
			delete(*meta, game)
		}
	}
	return wins
}

func buildMetaverse(pos1, pos2 int) {
	rollUpdate := dieRolls(diracSize, numRolls)
	fmt.Println(rollUpdate)
	// Set up the starting metaverse. During each round, we will update the metaverse.
	meta := Metaverse{}
	startGame := Game{Pos1: pos1, Pos2: pos2, Score1: 0, Score2: 0}
	meta[startGame] = 1
	// Count how many victories there were for each player. Go's zero value for integers is zero, so
	// we don't need to initialise the map.
	numPlayers := 2
	wins := map[int]int{}
	// Update the metaverse.
	for playerIdx := 0; len(meta) > 0; playerIdx = (playerIdx + 1) % numPlayers {
		meta = updateMetaverse(meta, rollUpdate, boardSize)
		wins[playerIdx] += countAndRemoveWins(&meta)
	}
	fmt.Println("Wins player 1:", wins[0])
	fmt.Println("Wins player 2:", wins[1])
}

// end::metaverse[]
