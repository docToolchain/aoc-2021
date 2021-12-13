package main

import (
	"fmt"
	"strings"
)

const (
	vecSep         = ","
	tokensPerPoint = 2
)

// tag::grid[]

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

// Grid is a lazily evaluated grid that supports marking points on it. Most of it has been taken
// from a previous solution.
type Grid map[Vec]int

// Mark marks a point on the grid n times. Don't accept numbers <0.
func (g *Grid) Mark(entry Vec, n int) error {
	if n < 0 {
		return fmt.Errorf("can only mark non-negative times")
	}
	// We don't have to handle non-existing values here since Go returns the zero value (0 for
	// integers) for such entries.
	(*g)[entry] = (*g)[entry] + n
	return nil
}

// Count determines how often a point has been marked.
func (g *Grid) Count(entry Vec) int {
	return (*g)[entry]
}

// Has determines whether a point is on the grid.
func (g *Grid) Has(entry Vec) bool {
	_, ok := (*g)[entry]
	return ok
}

// RemoveAll removes all markings for a specific point.
func (g *Grid) RemoveAll(entry Vec) {
	delete(*g, entry)
}

// Points returns an iterator over all points on the grid.
func (g *Grid) Points() <-chan Vec {
	channel := make(chan Vec)
	go func() {
		for point := range *g {
			channel <- point
		}
		close(channel)
	}()
	return channel
}

// ReduceMarked sets the marking value of all points that are marked to the given value.
func (g *Grid) ReduceMarked(val int) error {
	if val < 0 {
		return fmt.Errorf("can only mark non-negative times")
	}
	for p := range g.Points() {
		(*g)[p] = val
	}
	return nil
}

func (g *Grid) topLeft() (int, int) {
	foundX, foundY := false, false
	minX, minY := 0, 0
	for p := range g.Points() {
		if !foundX || p.x < minX {
			minX = p.x
			foundX = true
		}
		if !foundY || p.y < minY {
			minY = p.y
			foundY = true
		}
	}
	return minX, minY
}

func (g *Grid) bottomRight() (int, int) {
	foundX, foundY := false, false
	maxX, maxY := 0, 0
	for p := range g.Points() {
		if !foundX || p.x > maxX {
			maxX = p.x
			foundX = true
		}
		if !foundY || p.y > maxY {
			maxY = p.y
			foundY = true
		}
	}
	return maxX, maxY
}

// Pretty creates a pretty string representation of this grid.
func (g *Grid) Pretty(digits int) string {
	result := ""
	empty := " "
	maxVal := 10
	for count := 1; count < digits; count++ {
		empty += " "
		maxVal *= 10
	}
	formatStr := fmt.Sprintf("%%-%dd", digits)
	minX, minY := g.topLeft()
	maxX, maxY := g.bottomRight()
	for y := minY; y <= maxY; y++ {
		for x := minX; x <= maxX; x++ {
			val := g.Count(Vec{x: x, y: y})
			if val > 0 {
				result += fmt.Sprintf(formatStr, val)
				if val >= maxVal {
					// Return the string up to the offending point so that the caller sees what the
					// offending number was. This is a hack at best but good enough for me right
					// now.
					return result + "\nINCOMPLETE"
				}
			} else {
				result += empty
			}
		}
		result += "\n"
	}
	return result
}

// end::grid[]
