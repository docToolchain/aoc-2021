package main

import (
	"fmt"
	"log"
	"strconv"
	"strings"
)

const (
	hexBase    = 16
	binarybase = 2
	padding    = 4
)

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
	if s.read+num > len(s.bin) {
		log.Fatalf("trying to read too much, remaining %s, reading %d", s.ToString(true), num)
	}
	if num <= 0 {
		return []bool{}
	}
	result := s.bin[s.read : s.read+num : s.read+num]
	s.read += num
	s.pad = (s.pad + num) % padding
	return result
}

// Done determines whether any characters are left. If only zeroes are left, we also exit.
func (s *BitStream) Done() bool {
	if s.read >= len(s.bin) {
		return true
	}
	for _, bit := range s.bin[s.read:] {
		if bit {
			return false
		}
	}
	return true
}

// Pad reads up to the next multiple of since one hexadecimal digit is four binary digits.
func (s *BitStream) Pad() {
	if s.pad > 0 {
		_ = s.Next(padding - s.pad)
	}
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

// ToString converts to a binary string. Specify whether you want truncated (true) or full (false)
func (s *BitStream) ToString(truncated bool) string {
	result := ""
	start := 0
	if truncated {
		start = s.read
	}
	for _, val := range s.bin[start:] {
		if val {
			result += "1"
		} else {
			result += "0"
		}
	}
	fmtStr := fmt.Sprintf("%%%ds", len(s.bin))
	return fmt.Sprintf(fmtStr, result)
}

// LimitedStream creates a limited bit stream based on the next limit bits of this one.
func (s *BitStream) LimitedStream(limit int) BitStream {
	stream := BitStream{
		bin:  s.Next(limit),
		pad:  0,
		read: 0,
	}
	return stream
}

// BitsToInt converts a stream of bits into an integer.
func BitsToInt(bits []bool) int {
	result := 0
	for _, bit := range bits {
		result *= 2
		if bit {
			result++
		}
	}
	return result
}

// Parse parses a string into packages and sub-packages.
func Parse(input string) ([]Package, error) {
	result := []Package{}

	stream := BitStream{}
	if err := stream.AddHex(input); err != nil {
		return []Package{}, err
	}

	for !stream.Done() {
		pkg := FromBitStream(&stream)
		result = append(result, pkg)
		stream.Pad()
	}

	return result, nil
}
