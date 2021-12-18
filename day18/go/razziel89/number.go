package main

// tag::reduction[]

import (
	"fmt"
	"log"
	"strconv"
	"strings"
)

// Number describes properties of a number for snailfish. This interface is way too large and some
// methods are only implemented for one of the types. Sadly, I fought quite a bit with pointer
// receivers and non-pointer receivers...
type Number interface {
	IsPair() bool
	SetLeft(Number)
	SetRight(Number)
	Left() Number
	Right() Number
	Val() *int
	String() string
	Magnitude() int
	Copy() Number
}

// NumberFromString converts a string to a number. Numbers are not reduced automatically.
func NumberFromString(str string) (Number, error) {
	split := strings.Split(str, "")
	idx := 0
	result, err := numFromStrSlice(split, &idx)
	if err != nil {
		return nil, err
	}
	return result, nil
}

// Recursively parse numbers. This is done by going through the input slice and advancing the index
// for every rune/char processed. The function still calls itself recursively to build up te
// recursive data structure.
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

// Pair is an "actual number" containung two numbers.
type Pair struct {
	left  Number
	right Number
}

// IsPair says whether this is a pair. This is a hack since I didn't make type switches work with
// pointer receivers in a way that allowed me to manipulate the switched-on variable.
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

// Left gets the left value.
func (p *Pair) Left() Number {
	return p.left
}

// Right gets the left value.
func (p *Pair) Right() Number {
	return p.right
}

// Val gets a pointer to my value.
func (p *Pair) Val() *int {
	log.Fatal("not implemented for pairs")
	return nil
}

// String gets a string representation.
func (p *Pair) String() string {
	return fmt.Sprintf("[%s,%s]", p.left.String(), p.right.String())
}

// Magnitude returns the magnitude.
func (p *Pair) Magnitude() int {
	return 3*p.left.Magnitude() + 2*p.right.Magnitude()
}

// Copy copies the number so that it will not be changed later on.
func (p *Pair) Copy() Number {
	return &Pair{left: p.left.Copy(), right: p.right.Copy()}
}

// Digit is an actual digit. We wrap it here so that we can implement the interface for it.
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

// String gets a string representation.
func (d *Digit) String() string {
	return fmt.Sprint(d.val)
}

// Magnitude returns the magnitude.
func (d *Digit) Magnitude() int {
	return d.val
}

// Copy copies the number so that it will not be changed later on.
func (d *Digit) Copy() Number {
	return &Digit{d.val}
}

// Add adds two numbers. This is dead easy.
func Add(n1, n2 Number) Number {
	return &Pair{left: n1.Copy(), right: n2.Copy()}
}

// end::reduction[]
