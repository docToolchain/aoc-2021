package main

import (
	"fmt"
	"log"
)

// tag::solution[]

const (
	boardSize = 10
	dieSize   = 100
	numRolls  = 3
	winScore  = 1000
)

// Player is a player of the die game.
type Player struct {
	ID        int
	Pos       int
	Score     int
	BoardSize int
}

// Update updates a player with a roll.
func (p *Player) Update(roll int) {
	// Positions will be in [1,10]
	p.Pos = (p.Pos-1+roll)%p.BoardSize + 1
	p.Score += p.Pos
}

// String provides a string representation.
func (p *Player) String() string {
	return fmt.Sprintf("{id: %d, pos: %d, s: %d}", p.ID, p.Pos, p.Score)
}

// DetDie is a deterministic die.
type DetDie struct {
	LastRoll int
	DieSize  int
	NumRools int
}

// Roll rolls the dice the required number of times.
func (d *DetDie) Roll() int {
	result := 0
	for rollIdx := 0; rollIdx < d.NumRools; rollIdx++ {
		d.LastRoll++
		result += d.LastRoll
		d.LastRoll = d.LastRoll % d.DieSize
	}
	return result
}

func maxScore(players []Player) int {
	result := 0
	found := false
	for _, player := range players {
		if !found || player.Score > result {
			found = true
			result = player.Score
		}
	}
	return result
}

func main() {
	players, err := ReadLinesAsPlayers()
	if err != nil {
		log.Fatal(err.Error())
	}
	fmt.Println(players)

	pos1, pos2 := players[0].Pos, players[1].Pos

	die := DetDie{LastRoll: 0, DieSize: dieSize, NumRools: numRolls}

	rollCount := 0
	for playerIdx := 0; maxScore(players) < winScore; playerIdx = (playerIdx + 1) % len(players) {
		roll := die.Roll()
		rollCount += numRolls
		players[playerIdx].Update(roll)
	}

	fmt.Printf("We have a winner! Results after %d rolls are:\n", rollCount)
	for _, player := range players {
		fmt.Println(player.String(), "Relevant value is:", rollCount*player.Score)
	}

	buildMetaverse(pos1, pos2)
}

// end::solution[]
