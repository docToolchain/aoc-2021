package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"sort"
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

// Pow computes an integer power of an integer base.
func Pow(base, exponent int) int {
	result := 1
	for exp := 0; exp < exponent; exp++ {
		result *= base
	}
	return result
}

// ReadLinesAsInputsAndOutputs reads all lines from stdin, splitting tokens, and determining how
// many outputs there are. If not all lines have the same number of outputs, an error is returned.
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

// AllPermutations returns all possible permutations of a string via a channel. It uses Heap's
// algorithm. See https://en.wikipedia.org/wiki/Heap%27s_algorithm for details.
func AllPermutations(str string) <-chan string {
	channel := make(chan string)

	sli := strings.Split(str, "")

	var generate func([]string, int)
	generate = func(sli []string, permLen int) {
		if permLen == 1 {
			// This is a solution, emit it.
			emittedStr := strings.Join(sli, "")
			channel <- emittedStr
		} else {
			for idx := 0; idx < permLen; idx++ {
				generate(sli, permLen-1)
				if permLen%2 == 1 {
					sli[idx], sli[permLen-1] = sli[permLen-1], sli[idx]
				} else {
					sli[0], sli[permLen-1] = sli[permLen-1], sli[0]
				}
			}
		}
		// This is the outer-most call to generate. Make sure to close the channel in the end.
		if permLen == len(sli) {
			close(channel)
		}
	}
	go generate(sli, len(sli))

	return channel
}

// SortString sorts a string character-wise. This is particularly useful to determine whether two
// strings contain the same characters.
func SortString(str string) string {
	split := strings.Split(str, "")
	sort.Strings(split)
	combined := strings.Join(split, "")
	return combined
}

// end::utils[]
