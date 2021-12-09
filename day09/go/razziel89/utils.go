package main

import (
	"bufio"
	"fmt"
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

// ReadLinesAsGrid reads all lines from stdin as a grid. That is, each point on the grid has a
// height and a location.
func ReadLinesAsGrid() (Grid, error) {
	lineIdx := 0
	result := make(Grid)
	for {
		line, err := readLine()
		if err == io.EOF {
			// Success case, no more input to read.
			return result, nil
		}
		if err != nil {
			return Grid{}, err
		}
		line = strings.TrimSpace(line)
		ints, err := strSliceToIntSlice(strings.Split(line, ""))
		if err != nil {
			return Grid{}, err
		}
		for rowIdx, val := range ints {
			// This is lazy but I wanted to re-use old code.
			point, err := VecFromStr(fmt.Sprintf("%d%s%d", lineIdx, vecSep, rowIdx))
			if err != nil {
				return Grid{}, err
			}
			err = result.Mark(point, val)
			if err != nil {
				return Grid{}, err
			}
		}
		if err != nil {
			return Grid{}, err
		}
		lineIdx++
	}
}

// end::utils[]
