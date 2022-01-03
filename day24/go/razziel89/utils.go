package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"strconv"
	"strings"
)

// tag::utils[]
const (
	// Registers.
	allRegs = "wxyz"
)

var reader = bufio.NewReader(os.Stdin)

func readLine() (string, error) {
	return reader.ReadString('\n')
}

// ReadLinesAsOps reads lines as ops that are the input to the submarine's ACL.
//nolint:gomnd,funlen
func ReadLinesAsOps() ([]op, error) {
	result := []op{}
	for {
		line, err := readLine()
		if err == io.EOF {
			// Success case, no more input to read.
			return result, nil
		}
		if err != nil {
			return []op{}, err
		}
		line = strings.TrimSpace(line)
		fields := strings.Fields(line)
		if len(fields[1]) != 1 {
			err := fmt.Errorf("cannot extract register")
			return []op{}, err
		}
		if !strings.Contains(allRegs, fields[1]) {
			err := fmt.Errorf("unknown register")
			return []op{}, err
		}
		reg := rune(fields[1][0])
		var data interface{}
		if len(fields) == 3 {
			if strings.Contains(allRegs, fields[2]) {
				// Is a register.
				data = rune(fields[2][0])
			} else {
				// Is a number.
				num, err := strconv.Atoi(fields[2])
				if err != nil {
					return []op{}, err
				}
				data = num
			}
		}
		var o op
		opStr := fields[0]
		switch opStr {
		case "inp":
			if data != nil {
				err := fmt.Errorf("non-nil interface")
				return []op{}, err
			}
			o = op{act: opInp, reg: reg}
		case "add", "mul", "div", "mod", "eql":
			if data == nil {
				err := fmt.Errorf("nil interface")
				return []op{}, err
			}
			var opType rune
			switch opStr {
			case "add":
				opType = opAdd
			case "mul":
				opType = opMul
			case "div":
				opType = opDiv
			case "mod":
				opType = opMod
			case "eql":
				opType = opEql
			default:
				err := fmt.Errorf("internal error")
				return []op{}, err
			}
			o = op{act: opType, reg: reg, dat: data}
		default:
			err := fmt.Errorf("unknown op")
			return []op{}, err
		}
		result = append(result, o)
	}
}

// end::utils[]
