package main

import (
	"fmt"
)

const (
	areaInts = 4
)

// tag::area[]

// Vec is a 2D vector. Most of it has been taken from a previous solution.
type Vec struct {
	x, y int
}

// Add adds one vector to another one.
func (v Vec) Add(delta Vec) Vec {
	result := Vec{
		x: v.x + delta.x,
		y: v.y + delta.y,
	}
	return result
}

// Area describes the target area.
type Area struct {
	xMin, xMax, yMin, yMax int
}

// AreaFromString creates an area from a string.
func AreaFromString(str string) (Area, error) {
	areaFormat := "x=%d..%d, y=%d..%d"
	var xMin, xMax, yMin, yMax int
	count, err := fmt.Sscanf(str, areaFormat, &xMin, &xMax, &yMin, &yMax)
	if err != nil {
		return Area{}, fmt.Errorf("cannot parse area: %s", err.Error())
	}
	if count != areaInts {
		return Area{}, fmt.Errorf("not enough numbers processed, was %d wanted %d", count, areaInts)
	}
	area := Area{xMin, xMax, yMin, yMax}
	return area, nil
}

// Inside determines whether a point is inside the area.
func (a *Area) Inside(vec Vec) bool {
	return vec.x <= a.xMax && vec.x >= a.xMin && vec.y <= a.yMax && vec.y >= a.yMin
}

// Invalid determines whether a point is far off to the positive x direction or negative y
// direction.
func (a *Area) Invalid(vec Vec) bool {
	return vec.x > a.xMax || vec.y < a.yMin
}

// IsValid determines whether this is a valid area struct. For example, areas don't support negative
// x values for laziness reasons.
func (a *Area) IsValid() error {
	if a.xMax < 0 || a.xMin < 0 {
		return fmt.Errorf("areas with negative x values not supported")
	}
	return nil
}

// end::area[]
