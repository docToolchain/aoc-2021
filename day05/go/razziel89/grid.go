package main

import (
	"fmt"
	"os"
	"strings"
)

const (
	vecSep         = ","
	tokensPerPoint = 2
	lineSep        = "->"
	tokensPerLine  = 2
)

var (
	// Set PART to "1" to select only part 1.
	partSelect = os.Getenv("PART")
)

// tag::set[]

// Vec is a 2D vector. Most of it has been taken from a previous solution.
type Vec struct {
	x, y int
}

// VecFromStr converts a sring into a vector.
func VecFromStr(str string) (Vec, error) {
	fields := trimStrings(strings.Split(str, vecSep))
	if len(fields) != tokensPerPoint {
		return Vec{}, fmt.Errorf("cannot parse %v as vector, wrong number of fields", str)
	}
	ints, err := strSliceToIntSlice(fields)
	if err != nil {
		return Vec{}, fmt.Errorf("cannot parse %s as vector, %s", str, err.Error())
	}
	result := Vec{
		x: ints[0],
		y: ints[1],
	}
	return result, nil
}

// Add adds one vector to another one.
func (v Vec) Add(delta Vec) Vec {
	result := Vec{
		x: v.x + delta.x,
		y: v.y + delta.y,
	}
	return result
}

// Mul multiplies each component of a vector with a number.
func (v Vec) Mul(factor int) Vec {
	result := Vec{
		x: v.x * factor,
		y: v.y * factor,
	}
	return result
}

// Inv inverts a vector.
func (v Vec) Inv() Vec {
	return v.Mul(-1)
}

// Sub subtracts a vector'v data from another one'v.
func (v Vec) Sub(delta Vec) Vec {
	return v.Add(delta.Inv())
}

func abs(num int) int {
	if num < 0 {
		return -num
	}
	return num
}

func max(i1, i2 int) int {
	if i1 > i2 {
		return i1
	}
	return i2
}

// Normalize returns a unit vector with the same direction as the original vector. For now, this
// does not support diagonals.
func (v Vec) Normalize() (Vec, error) {
	if partSelect == "1" {
		if v.x != 0 && v.y != 0 {
			return Vec{}, fmt.Errorf("cannot normalize %v", v)
		}
	} else {
		// Default to part 2.
		if v.x != 0 && v.y != 0 && abs(v.x) != abs(v.y) {
			return Vec{}, fmt.Errorf("cannot normalize %v", v)
		}
	}
	length := max(abs(v.x), abs(v.y))
	norm := Vec{
		x: v.x / length,
		y: v.y / length,
	}
	return norm, nil
}

// Line is a line in 2D with a start and an end.
type Line struct {
	start, end Vec
}

// LineFromStr converts a sring into a line.
func LineFromStr(str string) (Line, error) {
	fields := trimStrings(strings.Split(str, lineSep))
	if len(fields) != tokensPerLine {
		return Line{}, fmt.Errorf("cannot parse %v as line, wrong number of fields", str)
	}
	start, err := VecFromStr(fields[0])
	if err != nil {
		return Line{}, fmt.Errorf("cannot parse %v as line, %v", str, err.Error())
	}
	end, err := VecFromStr(fields[1])
	if err != nil {
		return Line{}, fmt.Errorf("cannot parse %v as line, %v", str, err.Error())
	}
	result := Line{
		start: start,
		end:   end,
	}
	return result, nil
}

// Points determines all points on this line.
func (l Line) Points() ([]Vec, error) {
	result := []Vec{}
	direction, err := l.end.Sub(l.start).Normalize()
	if err != nil {
		// We ignore lines whose direction we cannot determine.
		return []Vec{}, nil
	}
	pos := l.start
	for pos != l.end {
		result = append(result, pos)
		pos = pos.Add(direction)
	}
	result = append(result, pos)
	return result, nil
}

// Grid is a lazily evaluated grid that supports marking points on it. Most of it has been taken
// from a previous solution.
type Grid map[Vec]int

// Mark marks a point on the grid once.
func (g *Grid) Mark(entry Vec) {
	// We don't have to handle non-existing values here since Go returns the zero value (0 for
	// integers) for such entries.
	(*g)[entry] = (*g)[entry] + 1
}

// Count determines how often a point has been marked.
func (g *Grid) Count(entry Vec) int {
	return (*g)[entry]
}

// RemoveAll removes all markings for a specific point.
func (g *Grid) RemoveAll(entry Vec) {
	delete(*g, entry)
}

// FilterFn is a type that can be used for FilterCounts to filter counts that fulfil a predicate.
type FilterFn = func(int) bool

// FilterCounts allow to filter points based counts using a FilterFn.
func (g *Grid) FilterCounts(filterFn FilterFn) []Vec {
	result := []Vec{}
	for point, count := range *g {
		if filterFn(count) {
			result = append(result, point)
		}
	}
	return result
}

// end::set[]
