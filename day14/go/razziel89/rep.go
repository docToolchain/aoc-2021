package main

import (
	"fmt"
	"strings"
)

const (
	repSep       = "->"
	tokensPerRep = 2
)

// tag::rep[]

// Match described one match of two characters.
type Match struct {
	Left      rune
	Right     rune
	LeftEdge  bool
	RightEdge bool
}

func b2s(b bool) rune {
	if b {
		return '1'
	}
	return '0'
}

// ToString provides a nice string representation for a match.
func (m Match) ToString() string {
	return fmt.Sprintf(
		"{%c%c [%c|%c]}",
		m.Left, m.Right, b2s(m.LeftEdge), b2s(m.RightEdge),
	)
}

// Replacement describes one replacement instruction by insertion.
type Replacement struct {
	Match  Match
	Insert rune
}

// RepFromString converts a sring into a replacement instruction.
func RepFromString(str string) (Replacement, error) {
	fields := trimStrings(strings.Split(str, repSep))
	if len(fields) != tokensPerRep {
		err := fmt.Errorf("cannot parse %v as rep, wrong number of fields", str)
		return Replacement{}, err
	}
	if len(fields[0]) != tokensPerRep {
		err := fmt.Errorf("cannot parse %v as rep, wrong number of characters", str)
		return Replacement{}, err
	}
	if len(fields[1]) != 1 {
		err := fmt.Errorf("cannot parse %v as rep, wrong number of insertions", str)
		return Replacement{}, err
	}
	match := Match{
		Left:  rune(fields[0][0]),
		Right: rune(fields[0][1]),
	}
	rep := Replacement{
		Match:  match,
		Insert: rune(fields[1][0]),
	}
	return rep, nil
}

// end::rep[]
