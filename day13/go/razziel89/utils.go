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

const (
	defaultGridVal    = 1
	instructionSep    = "fold along "
	instructionFields = 2
	lineSep           = "="
)

// Instruction describes a folding instruction.
type Instruction struct {
	Val int
	Dir string
}

func instructionFromString(str string) (Instruction, error) {
	fields := strings.Split(str, lineSep)
	if len(fields) != instructionFields {
		return Instruction{}, fmt.Errorf("cannot convert %s to instruction, fields", str)
	}
	var result Instruction
	val, err := strconv.Atoi(fields[1])
	if err != nil {
		return Instruction{}, err
	}
	result = Instruction{
		Val: val,
		Dir: fields[0],
	}
	if !strings.Contains("xy", fields[0]) {
		return Instruction{}, fmt.Errorf("cannot convert %s to instruction, coord", str)
	}
	return result, nil
}

// ReadLinesAsGridAndInstructions reads all lines from stdin as a grid and instructions.
func ReadLinesAsGridAndInstructions() (Grid, []Instruction, error) {
	result := make(Grid)
	instructions := []Instruction{}
	for {
		line, err := readLine()
		if err == io.EOF {
			// Success case, no more input to read.
			return result, instructions, nil
		}
		if err != nil {
			return Grid{}, []Instruction{}, err
		}
		line = strings.TrimSpace(line)
		if len(line) == 0 {
			// Ignore empty lines.
			continue
		}
		//nolint:nestif
		if strings.Contains(line, instructionSep) {
			// Line with instruction.
			fields := strings.Split(line, instructionSep)
			if len(fields) != instructionFields {
				err := fmt.Errorf("cannot extract instruction from %s", line)
				return Grid{}, []Instruction{}, err
			}
			ins, err := instructionFromString(fields[1])
			if err != nil {
				return Grid{}, []Instruction{}, err
			}
			instructions = append(instructions, ins)
		} else {
			// Line with grid point.
			vec, err := VecFromStr(line)
			if err != nil {
				return Grid{}, []Instruction{}, err
			}
			err = result.Mark(vec, defaultGridVal)
			if err != nil {
				return Grid{}, []Instruction{}, err
			}
		}
	}
}

// end::utils[]
