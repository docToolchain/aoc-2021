package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"strings"
)

const (
	empty             = '.'
	full              = '#'
	enhancementLength = 512
)

var reader = bufio.NewReader(os.Stdin)

// Function readLine reads one line from stdin via a global reader instance.
func readLine() (string, error) {
	return reader.ReadString('\n')
}

// tag::utils[]

// ReadLinesAsPixelGrid reads lines in as a pixel grid.
func ReadLinesAsPixelGrid() ([]bool, Grid, error) {
	result := Grid{background: false, data: map[Vec]bool{}}
	var algo []bool
	rowIdx := 0
	for {
		line, err := readLine()
		if err == io.EOF {
			if len(algo) != enhancementLength {
				err := fmt.Errorf("enhancement algorithm not of correct length")
				return []bool{}, Grid{}, err
			}
			// Success case, no more input to read.
			return algo, result, nil
		}
		if err != nil {
			return []bool{}, Grid{}, err
		}
		line = strings.TrimSpace(line)
		//nolint:nestif
		if len(line) == enhancementLength {
			algo = make([]bool, 0, len(line))
			for _, char := range line {
				algo = append(algo, char == full)
				if char != full && char != empty {
					err := fmt.Errorf("unknown character %c", char)
					return []bool{}, Grid{}, err
				}
			}
		} else if len(line) == 0 {
			continue
		} else {
			for colIdx, char := range line {
				pos := Vec{x: colIdx, y: rowIdx}
				result.Mark(pos, char == full)
				if char != full && char != empty {
					err := fmt.Errorf("unknown character %c", char)
					return []bool{}, Grid{}, err
				}
			}
			rowIdx++
		}
	}
}

// end::utils[]
