package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"strings"
)

const (
	inputOutputSep = "|"
	expectedSplits = 1
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

// ReadLinesAsInputsAndOutputs reads all lines from stdin, splitting tokens, and separating into
// inputs and outputs by determining how many outputs there are.
func ReadLinesAsInputsAndOutputs() ([][]string, int, error) {
	var result [][]string
	var numOutputs int
	for {
		line, err := readLine()
		if err == io.EOF {
			// Success case, no more input to read.
			return result, numOutputs, nil
		}
		if err != nil {
			return [][]string{}, 0, err
		}
		line = strings.TrimSpace(line)
		separated := trimStrings(strings.Split(line, inputOutputSep))
		if len(separated) != expectedSplits+1 {
			err := fmt.Errorf("cannot split into inputs and outputs")
			return [][]string{}, 0, err
		}
		if len(result) == 0 {
			numOutputs = len(strings.Fields(separated[1]))
		} else if len(strings.Fields(separated[1])) != numOutputs {
			err := fmt.Errorf("unexpected number of outputs in %s, wanted %d", line, numOutputs)
			return [][]string{}, 0, err
		}
		localResult := []string{}
		localResult = append(localResult, strings.Fields(separated[0])...)
		localResult = append(localResult, strings.Fields(separated[1])...)
		result = append(result, localResult)
	}
}

// end::utils[]
