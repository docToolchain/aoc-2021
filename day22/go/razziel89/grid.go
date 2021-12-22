package main

import (
	"fmt"
	"sort"
)

// tag::grid[]

const (
	buffer = 100
)

// Vec is a £D vector. Most of it has been taken from a previous solution.
type Vec struct {
	x, y, z int
}

// Add adds one vector to another one.
func (v Vec) Add(delta Vec) Vec {
	result := Vec{
		x: v.x + delta.x,
		y: v.y + delta.y,
		z: v.z + delta.z,
	}
	return result
}

// Get gets the co-ordinate based on the index.
func (v Vec) Get(idx int) int {
	if idx == 0 {
		return v.x
	}
	if idx == 1 {
		return v.y
	}
	if idx == 2 { //nolint:gomnd
		return v.z
	}
	panic("incorrect co-ordinate")
}

// Set sets the co-ordinate based on the index.
func (v *Vec) Set(idx, val int) {
	if idx == 0 {
		v.x = val
		return
	}
	if idx == 1 {
		v.y = val
		return
	}
	if idx == 2 { //nolint:gomnd
		v.z = val
		return
	}
	panic("incorrect co-ordinate")
}

// Mul multiplies each component of a vector with a number.
func (v Vec) Mul(factor int) Vec {
	result := Vec{
		x: v.x * factor,
		y: v.y * factor,
		z: v.z * factor,
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

func abs(val int) int {
	if val < 0 {
		return -val
	}
	return val
}

// Size gives the size of space spanned by a vector.
func (v Vec) Size() int {
	return abs(v.x * v.y * v.z)
}

// Cuboid is a cubouid in space.
type Cuboid struct {
	start, end Vec
}

// Size gives the size of space spanned by a cuboid.
func (c Cuboid) Size() int {
	diff := c.end.Sub(c.start)
	if diff.x <= 0 || diff.y <= 0 || diff.z <= 0 {
		// In this case, this cuboid does not describe a valid one.
		return 0
	}
	return diff.Size()
}

// Overlaps determines whether two overlap.
func (c Cuboid) Overlaps(other Cuboid) bool {
	inside := Cuboid{
		start: Vec{
			x: max(c.start.x, other.start.x),
			y: max(c.start.y, other.start.y),
			z: max(c.start.z, other.start.z),
		},
		end: Vec{
			x: min(c.end.x, other.end.x),
			y: min(c.end.y, other.end.y),
			z: min(c.end.z, other.end.z),
		},
	}
	return inside.Size() != 0
}

// String provides a string rep.
func (c Cuboid) String() string {
	return fmt.Sprintf(
		"x=%d..%d,y=%d..%d,z=%d..%d",
		c.start.x, c.end.x, c.start.y, c.end.y, c.start.z, c.end.z,
	)
}

// NewCuboid gets a new cuboid, ensuring that the co-ordinates are ordered correctly.
func NewCuboid(x1, x2, y1, y2, z1, z2 int) Cuboid {
	if x2 < x1 {
		x1, x2 = x2, x1
	}
	if y2 < y1 {
		y1, y2 = y2, y1
	}
	if z2 < z1 {
		z1, z2 = z2, z1
	}
	cub := Cuboid{
		start: Vec{
			x: x1,
			y: y1,
			z: z1,
		},
		end: Vec{
			x: x2,
			y: y2,
			z: z2,
		},
	}
	return cub
}

// Contains determines whether a vector lies within a cuboid.
func (c *Cuboid) Contains(vec Vec) bool {
	return vec.x-c.start.x < c.end.x && vec.y-c.start.y < c.end.y && vec.z-c.start.z < c.end.z
}

// Grid is a lazily evaluated grid that supports marking points on it. Most of it has been taken
// from a previous solution.
type Grid struct {
	data       map[Vec]bool
	background bool
	clamp      int
}

// Mark marks a point on the grid as true or false. Any markings equal to the background are
// ignored.
func (g *Grid) Mark(entry Vec, val bool) {
	// We don't have to handle non-existing values here since Go returns the zero value (false for
	// boolean values) for such entries. Accepting any boolean value here allows us to ignore the
	// background setting and always call Mark the same way.
	if val != g.background {
		g.data[entry] = val
	} else {
		delete(g.data, entry)
	}
}

//nolint:funlen
func clamp(cub Cuboid, startVal, endVal int) (Cuboid, bool) {
	x1 := cub.start.x
	y1 := cub.start.y
	z1 := cub.start.z

	if x1 < startVal {
		x1 = startVal
	} else if x1 > endVal {
		// In this case, the entire cuboid is outside the area of validity.
		return Cuboid{}, false
	}

	if y1 < startVal {
		y1 = startVal
	} else if y1 > endVal {
		// In this case, the entire cuboid is outside the area of validity.
		return Cuboid{}, false
	}

	if z1 < startVal {
		z1 = startVal
	} else if z1 > endVal {
		// In this case, the entire cuboid is outside the area of validity.
		return Cuboid{}, false
	}

	x2 := cub.end.x - 1
	y2 := cub.end.y - 1
	z2 := cub.end.z - 1

	if x2 > endVal {
		x2 = endVal
	} else if x2 < startVal {
		// In this case, the entire cuboid is outside the area of validity.
		return Cuboid{}, false
	}

	if y2 > endVal {
		y2 = endVal
	} else if y2 < startVal {
		// In this case, the entire cuboid is outside the area of validity.
		return Cuboid{}, false
	}

	if z2 > endVal {
		z2 = endVal
	} else if z2 < startVal {
		// In this case, the entire cuboid is outside the area of validity.
		return Cuboid{}, false
	}

	result := Cuboid{
		start: Vec{
			x: x1,
			y: y1,
			z: z1,
		},
		end: Vec{
			x: x2 + 1,
			y: y2 + 1,
			z: z2 + 1,
		},
	}
	return result, true
}

// MarkCuboid marks a point on the grid as true or false within a cuboid.
func (g *Grid) MarkCuboid(area Cuboid, val bool) {
	// Permit disabling the clamping by providing a value <=0.
	if g.clamp > 0 {
		var valid bool
		area, valid = clamp(area, -g.clamp, g.clamp)
		if !valid {
			// The cuboid is completely outside the area of validity. Don't to anything.
			return
		}
	}
	// If the to-be-marked value is equal to the background, we can get away with not iterating over
	// the entirety of the cuboid. This is only then more efficient, if the cuboid contains more
	// points that the grid. Otherwise, we have to iterate over the entire cuboid, but clamped to
	// the supported area.
	if val == g.background && len(g.data) < area.Size() {
		for vec := range g.data {
			if area.Contains(vec) {
				g.RemoveAll(vec)
			}
		}
	} else {
		// Iterate over the entire cuboid and mark each point with the given value.
		for x := area.start.x; x < area.end.x; x++ {
			for y := area.start.y; y < area.end.y; y++ {
				for z := area.start.z; z < area.end.z; z++ {
					vec := Vec{x: x, y: y, z: z}
					g.Mark(vec, val)
				}
			}
		}
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

// String gives you a string.
func (g Grid) String() string {
	points := make([]Vec, 0, len(g.data))
	for p := range g.data {
		points = append(points, p)
	}

	lessFn := func(i, j int) bool {
		if points[i].x < points[j].x {
			return true
		}
		if points[i].y < points[j].y {
			return true
		}
		if points[i].z < points[j].z {
			return true
		}
		return false
	}

	sort.Slice(points, lessFn)
	str := ""
	for _, p := range points {
		str += fmt.Sprintf("%v\n", p)
	}
	return str
}

// end::grid[]
