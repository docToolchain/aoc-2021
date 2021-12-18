package main

import (
	"fmt"
	"strconv"
	"strings"
)

// Number describes properties of a number for snailfish.
type Number interface {
}

// NumberFromString converts a string to a number.
func NumberFromString(str string) (Number, error) {
	split := strings.Split(str, "")
	idx := 0
	result, err := numFromStrSlice(split, &idx)
	if err != nil {
		return nil, err
	}
	return result, nil
}

// Recursively parse numbers.
func numFromStrSlice(sli []string, idx *int) (Number, error) {
	if len(sli) == 0 {
		return nil, fmt.Errorf("empty input")
	}
	switch sli[*idx] {
	case "[":
		*idx++
		left, err := numFromStrSlice(sli, idx)
		if err != nil {
			return nil, err
		}
		if sli[*idx] != "," {
			return nil, fmt.Errorf("cannot find comma")
		}
		*idx++
		right, err := numFromStrSlice(sli, idx)
		if err != nil {
			return nil, err
		}
		if sli[*idx] != "]" {
			return nil, fmt.Errorf("cannot find closing bracket")
		}
		*idx++
		return Pair{left: left, right: right}, nil
	default:
		// An actual digit reached! We only get single digits as input. Thus, process only one
		// character.
		num, err := strconv.Atoi(sli[*idx])
		if err != nil {
			return nil, err
		}
		*idx++
		return Digit{val: num}, nil
	}
}

// Pair is an actual number containung two numbers.
type Pair struct {
	left  Number
	right Number
}

// Digit is an actual number containung two numbers.
type Digit struct {
	val int
}
