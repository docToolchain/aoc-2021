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

// ReadLinesAsStates reads all lines from stdin, assuming one instruction describing a movement vector
// per line.
func ReadLinesAsStates() ([]State, error) {
	var sli []State
	for {
		line, err := readLine()
		if err == io.EOF {
			// Success case, no more input to read.
			return sli, nil
		}
		if err != nil {
			return []State{}, err
		}
		line = strings.TrimSpace(line)
		val, err := StateFromString(line)
		if err != nil {
			return []State{}, err
		}
		sli = append(sli, val)
	}
}

// tag::utils[]

// TokensInInstruction specifies how many token are needed to describe one movement.
const TokensInInstruction = 2

// StateFromString converts an instructions string to a movement.
func StateFromString(str string) (State, error) {
	splitStr := strings.Fields(str)
	if len(splitStr) != TokensInInstruction {
		return State{}, fmt.Errorf("wrong number %d of tokens in '%s'", len(splitStr), str)
	}
	unit, ok := Units[splitStr[0]]
	if !ok {
		return State{}, fmt.Errorf("cannot understand %s as unit vector name", splitStr[0])
	}
	repeats, err := strconv.Atoi(splitStr[1])
	if err != nil {
		return State{}, fmt.Errorf("cannot convert repeats in %s to int: %s", str, err.Error())
	}
	return unit.Mul(repeats), nil
}

// end::utils[]
