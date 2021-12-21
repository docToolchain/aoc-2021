package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"strings"
)

const (
	inputFormat = "Player %d starting position: %d"
	inputNums   = 2
)

var reader = bufio.NewReader(os.Stdin)

// Function readLine reads one line from stdin via a global reader instance.
func readLine() (string, error) {
	return reader.ReadString('\n')
}

// tag::utils[]

// ReadLinesAsPlayers reads lines in as players.
func ReadLinesAsPlayers() ([]Player, error) {
	result := []Player{}
	for {
		line, err := readLine()
		if err == io.EOF {
			// Success case, no more input to read.
			return result, nil
		}
		if err != nil {
			return []Player{}, err
		}
		line = strings.TrimSpace(line)
		//nolint:nestif
		var player Player
		count, err := fmt.Sscanf(line, inputFormat, &player.ID, &player.Pos)
		player.BoardSize = boardSize
		if err != nil {
			return []Player{}, err
		}
		if count != inputNums {
			return []Player{}, fmt.Errorf("cannot parse %s as player", line)
		}
		result = append(result, player)
	}
}

// end::utils[]
