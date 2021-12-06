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

// ReadLines reads all lines from stdin.
func ReadLines() ([]string, error) {
	var sli []string
	for {
		line, err := readLine()
		if err == io.EOF {
			// Success case, no more input to read.
			return sli, nil
		}
		if err != nil {
			return []string{}, err
		}
		val := strings.TrimSpace(line)
		sli = append(sli, val)
	}
}

// tag::utils[]

// CountTokens counts the tokens at each position of each string and returns one CountingSet per
// position. All strings must be of equal length.
func CountTokens(input []string) ([]CountingSet, error) {
	if len(input) == 0 {
		return []CountingSet{}, fmt.Errorf("empty string slice obtained")
	}
	length := len(input[0])
	for idx, str := range input {
		if len(str) != length {
			err := fmt.Errorf("string '%s' of unexpected length at index %d, expected %d got %d",
				str, idx+1, length, len(str),
			)
			return []CountingSet{}, err
		}
	}
	result := make([]CountingSet, length)
	for idx := range result {
		result[idx] = make(CountingSet)
	}
	for _, str := range input {
		for idx, char := range str {
			err := result[idx].Add(string(char))
			if err != nil {
				return []CountingSet{}, err
			}
		}
	}
	return result, nil
}

// Function sliceByIndex returns a slice of length-one strings that have been taken from the given
// index of each slice. An error is returned if a string in the slice is too short. This would be
// very easy in, say, NumPy, but it requires iteration in plain Go.
func sliceByIndex(sli []string, idx int) ([]string, error) {
	reqLength := idx + 1
	result := make([]string, 0, len(sli))
	for _, str := range sli {
		if len(str) < reqLength {
			err := fmt.Errorf(
				"entry '%s' too short, need at least %d but got %d",
				str, reqLength, len(str),
			)
			return []string{}, err
		}
		result = append(result, string(str[idx]))
	}
	return result, nil
}

// FilterFunc is a type needed to filter counts out. Based on values in a counting set, it
// determines which entries of a string slice to keep. For each value to keep, the resulting boolean
// array contains true, false otherwise.
type FilterFunc = func([]string, CountingSet) []bool

// FilterBy contains true in the resulting boolean array for every entry in sli that is equal to
// filter.
func FilterBy(sli []string, filter string) []bool {
	// The initial value of a boolean array is all false. Thus, only set those to true we wish to
	// keep.
	result := make([]bool, len(sli))
	for idx, val := range sli {
		if val == filter {
			result[idx] = true
		}
	}
	return result
}

// FilterCounts filters out inputs that are assigned true by filterFunc until exactly one remains.
// If none remains at the end, an error is returned.
func FilterCounts(inputs []string, filterFunc FilterFunc) (string, error) {
	allGoneErr := fmt.Errorf("everything was filtered out")
	copied := make([]string, len(inputs))
	_ = copy(copied, inputs)
	for idx := 0; len(copied) > 1; idx++ {
		counts, err := CountTokens(copied)
		if err != nil {
			return "", err
		}
		if len(counts) < idx+1 {
			return "", allGoneErr
		}
		sliced, err := sliceByIndex(copied, idx)
		if err != nil {
			return "", err
		}
		filtered := filterFunc(sliced, counts[idx])
		newCopy := make([]string, 0, len(filtered))
		for idx, include := range filtered {
			if include {
				newCopy = append(newCopy, copied[idx])
			}
		}
		copied = newCopy
	}
	if len(copied) == 0 {
		return "", allGoneErr
	}
	return copied[0], nil
}

// end::utils[]
