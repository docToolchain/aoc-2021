package main

import (
	"log"
)

// tag::grid[]

// Vec is a 2D vector. Most of it has been taken from a previous solution.
type Vec struct {
	x, y int
}

const (
	allZeroesAsDec = 0
	allOnesAsDec   = 511
	buffer         = 9
)

var (
	unitX = Vec{x: 1}
	unitY = Vec{y: 1}
	// The order of these relevant disps is important for binary to decimal conversion!
	relevantDisps = []Vec{
		// y==-1
		unitX.Inv().Sub(unitY),
		unitY.Inv(),
		unitX.Sub(unitY),
		// y==0
		unitX.Inv(),
		Vec{},
		unitX,
		// y==1
		unitX.Inv().Add(unitY),
		unitY,
		unitX.Add(unitY),
	}
)

// Obtain an iterator over all relevant points.
func relevantPoints(point Vec) <-chan Vec {
	channel := make(chan Vec, buffer)
	go func() {
		for _, disp := range relevantDisps {
			displaced := point.Add(disp)
			channel <- displaced
		}
		close(channel)
	}()
	return channel
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
type Grid struct {
	data       map[Vec]bool
	background bool
}

// Mark marks a point on the grid as true or false. Any markings equal to the background are
// ignored.
func (g *Grid) Mark(entry Vec, val bool) {
	// We don't have to handle non-existing values here since Go returns the zero value (0 for
	// integers) for such entries.
	if val != g.background {
		g.data[entry] = val
	}
}

// Marked determines whether a point has been marked.
func (g *Grid) Marked(entry Vec) bool {
	if val, found := g.data[entry]; found {
		return val
	}
	return g.background
}

// Has determines whether a point is on the grid.
func (g *Grid) Has(entry Vec) bool {
	_, ok := g.data[entry]
	return ok
}

// RemoveAll removes all markings for a specific point.
func (g *Grid) RemoveAll(entry Vec) {
	delete(g.data, entry)
}

// Points returns an iterator over all points on the grid that deviate from the background.
func (g *Grid) Points() <-chan Vec {
	channel := make(chan Vec, buffer)
	go func() {
		for point := range g.data {
			if g.Marked(point) != g.background {
				channel <- point
			}
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

// Pretty creates a pretty string representation of this grid.
func (g *Grid) Pretty(border int) string {
	result := ""
	empty := "."
	filled := "#"
	minX, minY := g.TopLeft()
	maxX, maxY := g.BottomRight()
	for y := minY - border; y <= maxY+border; y++ {
		for x := minX - border; x <= maxX+border; x++ {
			if g.Marked(Vec{x: x, y: y}) {
				result += filled
			} else {
				result += empty
			}
		}
		result += "\n"
	}
	return result
}

func newPointVal(g *Grid, point *Vec, algo *[]bool) bool {
	// Convert binary to decimal.
	bin := ""
	index := 0
	for p := range relevantPoints(*point) {
		index *= 2
		if g.Marked(p) {
			index++
			bin += "1"
		} else {
			bin += "0"
		}
	}
	if index > len(*algo) {
		log.Fatalf("algorithm not long enough for %d", index)
	}
	// Extract new value.
	newVal := (*algo)[index]
	return newVal
}

// Convert converts a grid to another one according to the algorithm.
func (g *Grid) Convert(algo []bool) Grid {
	// Determine background of new grid.
	var newBackground bool
	if g.background {
		newBackground = algo[allOnesAsDec]
	} else {
		newBackground = algo[allZeroesAsDec]
	}
	grid := Grid{background: newBackground, data: make(map[Vec]bool)}

	// Fill all points. We need to check each neighbour of each point for whether it needs to be
	// marked.
	// This algorithm will mark some points multiple times. But that's OK, we'll just be wasting
	// time but not causing any errors that way.
	for p := range g.Points() {
		for markMe := range relevantPoints(p) {
			newVal := newPointVal(g, &markMe, &algo)
			grid.Mark(markMe, newVal)
		}
	}

	return grid
}

// end::grid[]
