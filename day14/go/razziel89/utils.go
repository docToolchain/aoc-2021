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

func trimStrings(sli []string) []string {
	result := make([]string, 0, len(sli))
	for _, val := range sli {
		result = append(result, strings.TrimSpace(val))
	}
	return result
}

// tag::utils[]

// ReadLinesAsReplacementsOrInput reads all lines from stdin as replacement instructions or input.
func ReadLinesAsReplacementsOrInput() (string, []Replacement, error) {
	var result []Replacement
	var input string
	for {
		line, err := readLine()
		if err == io.EOF {
			// Success case, no more input to read.
			return input, result, nil
		}
		if err != nil {
			return "", []Replacement{}, err
		}
		line = strings.TrimSpace(line)
		if len(line) == 0 {
			// Skip empty lines.
			continue
		}
		if strings.Contains(line, repSep) {
			// A line with a replacement.
			parsed, err := RepFromString(line)
			if err != nil {
				return "", []Replacement{}, err
			}
			result = append(result, parsed)
		} else {
			// A line with the input string.
			if len(input) != 0 {
				err := fmt.Errorf("already processed input")
				return "", []Replacement{}, err
			}
			input = line
		}
	}
}

// end::utils[]
