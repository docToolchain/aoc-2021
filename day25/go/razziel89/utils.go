package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"strings"
)

const (
	kindSouth = 'v'
	kindEast  = '>'
	kindEmpty = '0'
	emptyChar = '.'
)

var reader = bufio.NewReader(os.Stdin)

// Function readLine reads one line from stdin via a global reader instance.
func readLine() (string, error) {
	return reader.ReadString('\n')
}

// tag::utils[]

// ReadLinesAsImage reads input as an image of sea cucumbers.
func ReadLinesAsImage() ([]rune, int, error) {
	var zeroVal rune
	result := []rune{}
	lineLen := -1
	for rowIdx := 0; ; rowIdx++ {
		line, err := readLine()
		if err == io.EOF {
			// Success case, no more input to read.
			return result, lineLen, nil
		}
		if err != nil {
			return []rune{}, 0, err
		}
		line = strings.TrimSpace(line)
		if lineLen < 0 {
			lineLen = len(line)
		}
		if len(line) != lineLen {
			return []rune{}, 0, fmt.Errorf("line %s has incorrect length", line)
		}
		for _, char := range line {
			if char == emptyChar {
				char = zeroVal
			}
			result = append(result, char)
		}
	}
}

// end::utils[]
