package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"strings"
)

var reader = bufio.NewReader(os.Stdin)

// Function readLine reads one line from stdin via a global reader instance.
func readLine() (string, error) {
	return reader.ReadString('\n')
}

// tag::utils[]

// An example game looks like this:
// #############
// #...........#
// ###B#C#B#D###
//   #A#D#C#A#
//   #########

const (
	frontWallFormat = "#############"
	hallFormat      = "#...........#"
	entryFormat     = "###%c#%c#%c#%c###"
	roomFormat      = "#%c#%c#%c#%c#"
	backWallFormat  = "#########"
	numHalls        = 4
	numPods         = 4
	firstInputLine  = 2
)

var hallVals = [numHalls]rune{a, b, c, d}

// ReadLinesAsGame reads lines as a game for the amphipods.
//nolint:funlen
func ReadLinesAsGame() ([16]rune, error) {
	result := [16]rune{}
	// Initialise with the assumption all rooms are correctly filled.
	for hallIdx, hallVal := range hallVals {
		for podIdx := 0; podIdx < numPods; podIdx++ {
			result[hallIdx*numPods+podIdx] = hallVal
		}
	}
	for lineNr := 0; ; lineNr++ {
		line, err := readLine()
		if err == io.EOF {
			// Success case, no more input to read.
			return result, nil
		}
		if err != nil {
			return [16]rune{}, err
		}
		line = strings.TrimSpace(line)
		//nolint:gomnd
		switch lineNr {
		case 0:
			if line != frontWallFormat {
				err := fmt.Errorf("incorrect line %d", lineNr)
				return [16]rune{}, err
			}
		case 1:
			if line != hallFormat {
				err := fmt.Errorf("incorrect line %d", lineNr)
				return [16]rune{}, err
			}
		case firstInputLine:
			count, err := fmt.Sscanf(
				line, entryFormat,
				&result[0], &result[4], &result[8], &result[12],
			)
			if count != numHalls {
				err := fmt.Errorf("incorrect line %d", lineNr)
				return [16]rune{}, err
			}
			if err != nil {
				return [16]rune{}, err
			}
		}
		//nolint:gomnd
		if lineNr > firstInputLine && line != backWallFormat {
			// If we have reached the back wall, this is not a problem.
			memberIdx := lineNr - firstInputLine
			count, err := fmt.Sscanf(
				line, roomFormat,
				&result[memberIdx+0],
				&result[memberIdx+4],
				&result[memberIdx+8],
				&result[memberIdx+12],
			)
			if count != numHalls {
				err := fmt.Errorf("incorrect line %d", lineNr)
				return [16]rune{}, err
			}
			if err != nil {
				return [16]rune{}, err
			}
		}
	}
}

// end::utils[]
