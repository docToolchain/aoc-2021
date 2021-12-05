package main

import (
	"fmt"
	"strconv"
	"strings"
)

const (
	vecSep         = ","
	tokensPerPoint = 2
	lineSep        = "->"
	tokensPerLine  = 2
)

func trimStrings(sli []string) []string {
	result := make([]string, 0, len(sli))
	for _, val := range result {
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

// tag::set[]

// Vec is a 2D vector.
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
		return Vec{}, fmt.Errorf("cannot parse %v as vector, %v", str, err.Error)
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

// Normalize returns a unit vector with the same direction as the original vector. For now, this
// does not support diagonals.
func (v Vec) Normalize() (Vec, error) {
	if v.y == 0 {
		return Vec{x: 1}, nil
	} else if v.x == 0 {
		return Vec{y: 1}, nil
	} else {
		return Vec{}, fmt.Errorf("cannot normalize %v", v)
	}
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
		return []Vec{}, err
	}
	pos := l.start
	for pos != l.end {
		result = append(result, pos)
		pos = pos.Add(direction)
	}
	result = append(result, pos)
	return result, nil
}

// Grid is a lazily evaluated grid that supports marking points on it.
type Grid map[Vec]int

// Mark makrs a point on the grid once.
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

// FilterCounts allow to filter counts using a FilterFn.
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
