package main

import (
	"bufio"
	"io"
	"os"
	"strings"
)

const (
	setSep = ","
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

// ReadLinesAsSets reads all lines from stdin as counting sets.
func ReadLinesAsSets() ([]CountingSet, error) {
	var result []CountingSet
	for {
		line, err := readLine()
		if err == io.EOF {
			// Success case, no more input to read.
			return result, nil
		}
		if err != nil {
			return []CountingSet{}, err
		}
		fields := trimStrings(strings.Split(line, setSep))
		set := CountingSet{}
		for _, field := range fields {
			err := set.Add(field)
			if err != nil {
				return []CountingSet{}, err
			}
		}
		result = append(result, set)
	}
}

// end::utils[]
