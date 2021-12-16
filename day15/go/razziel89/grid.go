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

var (
	unitX         = Vec{x: 1}
	unitY         = Vec{y: 1}
	pointEnvDisps = []Vec{
		unitX,
		unitY,
		unitY.Inv(),
		unitX.Inv(),
	}
)

// Obtain an iterator over a point's environment.
func pointEnv(point Vec) <-chan Vec {
	channel := make(chan Vec)
	go func() {
		for _, disp := range pointEnvDisps {
			displaced := point.Add(disp)
			channel <- displaced
		}
		close(channel)
	}()
	return channel
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

// FilterThreshold returns all points whose value exceeds a threshold.
func (g *Grid) FilterThreshold(threshold int) []Vec {
	result := []Vec{}
	for point := range g.Points() {
		if g.Count(point) > threshold {
			result = append(result, point)
		}
	}
	return result
}

// MarkAll maks all points in the grid with the same value.
func (g *Grid) MarkAll(val int) error {
	for p := range g.Points() {
		if err := g.Mark(p, val); err != nil {
			return err
		}
	}
	return nil
}

// EnvFn is a function type used to determine a point's environment.
type EnvFn = func(point Vec) <-chan Vec

// MarkExistingEnv marks all points by the specified value in the environment of a given point. The
// environment is defined by envFn. Only points that actually exist will be marked.
func (g *Grid) MarkExistingEnv(val int, point Vec, envFn EnvFn) error {
	for neigh := range envFn(point) {
		if !g.Has(neigh) {
			continue
		}
		if err := g.Mark(neigh, val); err != nil {
			return err
		}
	}
	return nil
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

// TopLeft returns the coordinates of the top left node.
func (g *Grid) TopLeft() (int, int) {
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

// BottomRight returns the coordinates of the bottom right node.
func (g *Grid) BottomRight() (int, int) {
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

// end::grid[]
