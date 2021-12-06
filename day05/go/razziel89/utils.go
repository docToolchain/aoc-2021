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

func trimStrings(sli []string) []string {
	result := make([]string, 0, len(sli))
	for _, val := range sli {
		result = append(result, strings.TrimSpace(val))
	}
	return result
}

func strSliceToIntSlice(sli []string) ([]int, error) {
	// I wish Go had a map function...
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

// tag::utils[]

// ReadLinesAsLines reads all lines from stdin as Line structs.
func ReadLinesAsLines() ([]Line, error) {
	var result []Line
	for {
		line, err := readLine()
		if err == io.EOF {
			// Success case, no more input to read.
			return result, nil
		}
		if err != nil {
			return []Line{}, err
		}
		line = strings.TrimSpace(line)
		parsed, err := LineFromStr(line)
		if err != nil {
			return []Line{}, err
		}
		result = append(result, parsed)
	}
}

// end::utils[]
