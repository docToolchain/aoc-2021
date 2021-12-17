package main

import (
	"fmt"
)

const (
	areaInts = 4
)

// tag::grid[]

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

// Distance determines the distance of a point to the area, Manhattan metric.
func (a *Area) Distance(vec Vec) Vec {
	distance := Vec{}
	// X coordinate.
	if vec.x < a.xMin {
		distance.x += a.xMin - vec.x
	} else if vec.x > a.xMax {
		distance.x += vec.x - a.xMax
	}
	// Y coordinate.
	if vec.y < a.yMin {
		distance.y += a.yMin - vec.y
	} else if vec.y > a.yMax {
		distance.y += vec.y - a.yMax
	}
	return distance
}

// end::grid[]
