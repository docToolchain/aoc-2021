package main

import (
	"bufio"
	"io"
	"os"
	"strconv"
	"strings"
)

// Sum computes the sum of all values in an int slice. This is a possible reduction function for
// SlidingWindow.
func Sum(vals []int) int {
	result := 0
	for _, val := range vals {
		result += val
	}
	return result
}

// SlidingWindow converts an int slice into one that results from applying a reduction function to
// each sliding window of size `size`.
func SlidingWindow(sli []int, size int, reductionFn func([]int) int) []int {
	result := make([]int, 0, len(sli)-size+1)
	for startIdx := range sli[:len(sli)-size+1] {
		window := sli[startIdx : startIdx+size]
		windowSum := reductionFn(window)
		result = append(result, windowSum)
	}
	return result
}

var reader = bufio.NewReader(os.Stdin)

// Function readLine reads one line from stdin via a global reader instance.
func readLine() (string, error) {
	return reader.ReadString('\n')
}

// ReadLinesAsInts reads all lines from stdin, assuming one int per line.
func ReadLinesAsInts() ([]int, error) {
	var sli []int
	for {
		line, err := readLine()
		if err == io.EOF {
			// Success case, no more input to read.
			return sli, nil
		}
		if err != nil {
			return []int{}, err
		}
		line = strings.TrimSpace(line)
		val, err := strconv.Atoi(line)
		if err != nil {
			return []int{}, err
		}
		sli = append(sli, val)
	}
}

// CountIncrements counts how often an int in an int slice is larger than its predecessor.
func CountIncrements(sli []int) int {
	var increments int
	if len(sli) == 0 {
		return 0
	}
	last := sli[0]
	for _, val := range sli[1:] {
		if val > last {
			increments++
		}
		last = val
	}
	return increments
}
