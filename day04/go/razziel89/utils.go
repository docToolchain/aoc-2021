package main

import (
	"bufio"
	"io"
	"os"
	"strconv"
	"strings"
)

var reader = bufio.NewReader(os.Stdin)

// Function readLine reads one line from stdin via a global reader instance.
func readLine() (string, error) {
	return reader.ReadString('\n')
}

func strSliceToIntSlice(sli []string) ([]int, error) {
	result := make([]int, 0, len(sli))
	for _, val := range sli {
		conv, err := strconv.Atoi(val)
		if err != nil {
			return []int{}, err
		}
		result = append(result, conv)
	}
	return result, nil
}

const pickSep = ","

// ReadLinesAsPicksOrBoards reads all lines from stdin as picks or boards.
func ReadLinesAsPicksOrBoards() ([]int, []Board, error) {
	var picks []int
	var boards []Board
	var board Board
	for {
		line, err := readLine()
		if err == io.EOF {
			// Success case, no more input to read.
			return picks, boards, nil
		}
		if err != nil {
			return []int{}, []Board{}, err
		}
		line = strings.TrimSpace(line)
		if strings.Contains(line, pickSep) {
			// This is a line with picks.
			fields := strings.Split(line, pickSep)
			newPicks, err := strSliceToIntSlice(fields)
			if err != nil {
				return []int{}, []Board{}, err
			}
			picks = append(picks, newPicks...)
		} else {
			// This is a line with boards.
			row, err := strSliceToIntSlice(strings.Fields(line))
			if err != nil {
				return []int{}, []Board{}, err
			}
			err = board.AddRow(row)
			if err != nil {
				return []int{}, []Board{}, err
			}
		}
		if board.IsComplete() {
			boards = append(boards, board)
			board = Board{}
		}
	}
}

// tag::utils[]

// end::utils[]
