package main

import (
	"fmt"
	"strconv"
	"strings"
)

const (
	hexBase    = 16
	binarybase = 2
	// Numerical values, there are a lot of them this time.
	// literalType = 4
	padding = 4
)

// Package describes a package.
type Package interface {
	Version() int
	Type() int
	SubPackages() []Package
}

func hexToBinary(char rune) ([]bool, error) {
	num, err := strconv.ParseInt(string(char), hexBase, 0)
	if err != nil {
		return []bool{}, err
	}
	binaryString := strconv.FormatInt(num, binarybase)
	for len(binaryString) < padding {
		binaryString = "0" + binaryString
	}
	result := make([]bool, 0, len(binaryString))
	for _, char := range binaryString {
		result = append(result, char == '1')
	}
	return result, nil
}

// BitStream provides a stream of bits based on hex code. I am using boolean values here to
// represent bits.
type BitStream struct {
	bin  []bool
	pad  int
	read int
}

// Next provides the next num bits.
func (s *BitStream) Next(num int) []bool {
	result := s.bin[s.read : s.read+num : s.read+num]
	s.read += num
	s.pad = s.read % padding
	return result
}

// Pad reads up to the next multiple of since one hexadecimal digit is four binary digits.
func (s *BitStream) Pad() {
	_ = s.Next(s.pad)
}

// AddHex adds a hex string.
func (s *BitStream) AddHex(hex string) error {
	if len(s.bin) == 0 {
		s.bin = make([]bool, 0, len(hex))
	} else {
		newBin := make([]bool, 0, len(s.bin)+len(hex))
		_ = copy(newBin, s.bin)
		s.bin = newBin
	}
	hex = strings.ToUpper(hex)
	for _, char := range hex {
		binarySequence, err := hexToBinary(char)
		if err != nil {
			return fmt.Errorf("error in hex to binary conversion: %s", err.Error())
		}
		s.bin = append(s.bin, binarySequence...)
	}
	return nil
}

// ToString converts to a binary string.
func (s *BitStream) ToString() string {
	result := ""
	for _, val := range s.bin {
		if val {
			result += "1"
		} else {
			result += "0"
		}
	}
	return result
}

// Parse parses a string into packages and sub-packages.
func Parse(input string) ([]Package, error) {
	stream := BitStream{}
	if err := stream.AddHex(input); err != nil {
		return []Package{}, err
	}
	fmt.Println(stream.ToString())
	return []Package{}, nil
}
