package main

import (
	"fmt"
	"log"
	"strconv"
	"strings"
)

// Number describes properties of a number for snailfish.
type Number interface {
	IsPair() bool
	SetLeft(Number)
	SetRight(Number)
	SetVal(int)
	Left() Number
	Right() Number
	Val() *int
	Pointer() Number
	String() string
}

// NumberFromString converts a string to a number. Numbers are not reduced.
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
		return &Pair{left: left, right: right}, nil
	default:
		// An actual digit reached! We only get single digits as input. Thus, process only one
		// character.
		num, err := strconv.Atoi(sli[*idx])
		if err != nil {
			return nil, err
		}
		*idx++
		return &Digit{val: num}, nil
	}
}

// Pair is an actual number containung two numbers.
type Pair struct {
	left  Number
	right Number
}

// IsPair says whether this is a pair.
func (p *Pair) IsPair() bool {
	return true
}

// SetLeft sets the left value.
func (p *Pair) SetLeft(num Number) {
	p.left = num
}

// SetRight sets the left value.
func (p *Pair) SetRight(num Number) {
	p.right = num
}

// Left sets the left value.
func (p *Pair) Left() Number {
	return p.left
}

// Right sets the left value.
func (p *Pair) Right() Number {
	return p.right
}

// Val gets a pointer to my value.
func (p *Pair) Val() *int {
	log.Fatal("not implemented for pairs")
	return nil
}

// SetVal sets the value.
func (p *Pair) SetVal(int) {
	log.Fatal("not implemented for pairs")
}

// Pointer gets a pointer to myself.
func (p *Pair) Pointer() Number {
	return p
}

// String gets a string representation.
func (p *Pair) String() string {
	return fmt.Sprintf("[ %s , %s ]", p.left.String(), p.right.String())
}

// Digit is an actual number containung two numbers.
type Digit struct {
	val int
}

// IsPair says whether this is a pair.
func (d *Digit) IsPair() bool {
	return false
}

// SetLeft sets the left value.
func (d *Digit) SetLeft(Number) {
	log.Fatal("not implemented for digits")
}

// SetRight sets the left value.
func (d *Digit) SetRight(Number) {
	log.Fatal("not implemented for digits")
}

// SetVal sets the value.
func (d *Digit) SetVal(num int) {
	d.val = num
}

// Left gets the left value.
func (d *Digit) Left() Number {
	log.Fatal("not implemented for digits")
	return nil
}

// Right gets the left value.
func (d *Digit) Right() Number {
	log.Fatal("not implemented for digits")
	return nil
}

// Val gets a pointer to my value.
func (d *Digit) Val() *int {
	return &d.val
}

// Pointer gets a pointer to myself.
func (d *Digit) Pointer() Number {
	return d
}

// String gets a string representation.
func (d *Digit) String() string {
	return fmt.Sprint(d.val)
}

// Add adds two numbers.
func Add(n1, n2 Number) Number {
	return &Pair{left: n1, right: n2}
}
