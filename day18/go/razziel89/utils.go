package main

import (
	"bufio"
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

// ReadLinesAsNumbers reads all lines from stdin as replacement instructions or input.
func ReadLinesAsNumbers() ([]Number, error) {
	var result []Number
	for {
		line, err := readLine()
		if err == io.EOF {
			// Success case, no more input to read.
			return result, nil
		}
		if err != nil {
			return []Number{}, err
		}
		line = strings.TrimSpace(line)
		if len(line) == 0 {
			// Skip empty lines.
			continue
		}
		parsed, err := NumberFromString(line)
		if err != nil {
			return []Number{}, err
		}
		result = append(result, parsed)
	}
}

// end::utils[]
