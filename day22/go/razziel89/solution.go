package main

import (
	"fmt"
	"log"
)

// tag::solution[]

const (
	clampVal   = 50
	maxNumCubs = 26
)

func max(num1, num2 int) int {
	if num1 > num2 {
		return num1
	}
	return num2
}

func min(num1, num2 int) int {
	if num1 < num2 {
		return num1
	}
	return num2
}

// One cuboid eats a chunk out of another cuboid, splitting the first one into up to 26 new ones.
//nolint:funlen,gomnd,dupl
func eatChunk(onCub, offCub Cuboid) ([]Cuboid, bool) {
	result := make([]Cuboid, 0, maxNumCubs)
	// At first, determine a cuboid that is fully within both of the other cuboids.
	inside := Cuboid{
		start: Vec{
			x: max(onCub.start.x, offCub.start.x),
			y: max(onCub.start.y, offCub.start.y),
			z: max(onCub.start.z, offCub.start.z),
		},
		end: Vec{
			x: min(onCub.end.x, offCub.end.x),
			y: min(onCub.end.y, offCub.end.y),
			z: min(onCub.end.z, offCub.end.z),
		},
	}
	if inside.Size() == 0 {
		// In this case, one cuboid is completely outside the other one.
		return []Cuboid{}, false
	}
	if inside == onCub {
		// In this case, one cuboid is completely outside the other one.
		return []Cuboid{}, true
	}
	for _, dim := range [3]int{0, 1, 2} {
		// Negative direction.
		{
			newCub := Cuboid{start: inside.start, end: inside.end}
			newCub.start.Set(dim, onCub.start.Get(dim))
			newCub.end.Set(dim, inside.start.Get(dim))
			if newCub.Size() > 0 {
				result = append(result, newCub)
			}
		}
		// Positive direction.
		{
			newCub := Cuboid{start: inside.start, end: inside.end}
			newCub.start.Set(dim, inside.end.Get(dim))
			newCub.end.Set(dim, onCub.end.Get(dim))
			if newCub.Size() > 0 {
				result = append(result, newCub)
			}
		}
	}
	// count == 6
	for _, dim := range [3]int{0, 1, 2} {
		nextDim := (dim + 1) % 3
		// Negative dim direction
		{
			// Negative nextDim direction
			{
				newCub := Cuboid{start: inside.start, end: inside.end}
				newCub.start.Set(dim, onCub.start.Get(dim))
				newCub.end.Set(dim, inside.start.Get(dim))
				newCub.start.Set(nextDim, onCub.start.Get(nextDim))
				newCub.end.Set(nextDim, inside.start.Get(nextDim))
				if newCub.Size() > 0 {
					result = append(result, newCub)
				}
			}
			// Positive nextDim direction
			{
				newCub := Cuboid{start: inside.start, end: inside.end}
				newCub.start.Set(dim, onCub.start.Get(dim))
				newCub.end.Set(dim, inside.start.Get(dim))
				newCub.start.Set(nextDim, inside.end.Get(nextDim))
				newCub.end.Set(nextDim, onCub.end.Get(nextDim))
				if newCub.Size() > 0 {
					result = append(result, newCub)
				}
			}
		}
		// Positive dim direction
		{
			// Negative nextDim direction
			{
				newCub := Cuboid{start: inside.start, end: inside.end}
				newCub.start.Set(dim, inside.end.Get(dim))
				newCub.end.Set(dim, onCub.end.Get(dim))
				newCub.start.Set(nextDim, onCub.start.Get(nextDim))
				newCub.end.Set(nextDim, inside.start.Get(nextDim))
				if newCub.Size() > 0 {
					result = append(result, newCub)
				}
			}
			// Positive nextDim direction
			{
				newCub := Cuboid{start: inside.start, end: inside.end}
				newCub.start.Set(dim, inside.end.Get(dim))
				newCub.end.Set(dim, onCub.end.Get(dim))
				newCub.start.Set(nextDim, inside.end.Get(nextDim))
				newCub.end.Set(nextDim, onCub.end.Get(nextDim))
				if newCub.Size() > 0 {
					result = append(result, newCub)
				}
			}
		}
	}
	// Eight corners.
	// x>0 y>0 z>0
	{
		newCub := Cuboid{start: inside.end, end: onCub.end}
		if newCub.Size() > 0 {
			result = append(result, newCub)
		}
	}
	// x>0 y>0 z<0
	{
		newCub := Cuboid{start: inside.end, end: onCub.end}
		newCub.start.z = onCub.start.z
		newCub.end.z = inside.start.z
		if newCub.Size() > 0 {
			result = append(result, newCub)
		}
	}
	// x>0 y<0 z>0
	{
		newCub := Cuboid{start: inside.end, end: onCub.end}
		newCub.start.y = onCub.start.y
		newCub.end.y = inside.start.y
		if newCub.Size() > 0 {
			result = append(result, newCub)
		}
	}
	// x>0 y<0 z<0
	{
		newCub := Cuboid{start: inside.end, end: onCub.end}
		newCub.start.y = onCub.start.y
		newCub.end.y = inside.start.y
		newCub.start.z = onCub.start.z
		newCub.end.z = inside.start.z
		if newCub.Size() > 0 {
			result = append(result, newCub)
		}
	}
	// x<0 y<0 z<0
	{
		newCub := Cuboid{start: onCub.start, end: inside.start}
		if newCub.Size() > 0 {
			result = append(result, newCub)
		}
	}
	// x<0 y<0 z>0
	{
		newCub := Cuboid{start: onCub.start, end: inside.start}
		newCub.start.z = inside.end.z
		newCub.end.z = onCub.end.z
		if newCub.Size() > 0 {
			result = append(result, newCub)
		}
	}
	// x<0 y>0 z<0
	{
		newCub := Cuboid{start: onCub.start, end: inside.start}
		newCub.start.y = inside.end.y
		newCub.end.y = onCub.end.y
		if newCub.Size() > 0 {
			result = append(result, newCub)
		}
	}
	// x<0 y>0 z>0
	{
		newCub := Cuboid{start: onCub.start, end: inside.start}
		newCub.start.y = inside.end.y
		newCub.end.y = onCub.end.y
		newCub.start.z = inside.end.z
		newCub.end.z = onCub.end.z
		if newCub.Size() > 0 {
			result = append(result, newCub)
		}
	}
	if len(result) == 0 && inside.Size() > 0 {
		log.Fatal("something went wrong")
	}
	return result, true
}

func totalSize(cubs []Cuboid) int {
	sum := 0
	for _, cub := range cubs {
		sum += cub.Size()
	}
	return sum
}

//nolint:funlen
func main() {
	cubs, switches, err := ReadLinesAsCuboids()
	if err != nil {
		log.Fatal(err.Error())
	}
	if len(cubs) != len(switches) {
		log.Fatal("need as many switches as cuboids")
	}

	fmt.Println("Part 1")

	grid := Grid{data: map[Vec]bool{}, clamp: clampVal, background: false}

	for idx, cub := range cubs {
		swit := switches[idx]
		grid.MarkCuboid(cub, swit)
	}
	fmt.Println("Number of cubes marked:", len(grid.data))

	fmt.Println()
	fmt.Println("Part 2")

	currentCubs := []Cuboid{cubs[0]}
	if !switches[0] {
		log.Fatal("expect first one to switch something on")
	}

	for idx, cub := range cubs[1:] {
		swit := switches[idx+1]
		// Switch something on: Let the new chunk eat something out of all existing chunks.
		// Then, add the new chunk whole.
		// Switch something off: Let the new chunk eat something out of all existing chunks.
		// Only add those reduces chunks but don't add the new chunk.
		newCubs := []Cuboid{}
		for _, currCub := range currentCubs {
			chunks, overlap := eatChunk(currCub, cub)
			if len(chunks) > 0 {
				// There was definitely some overlap.
				newCubs = append(newCubs, chunks...)
			} else if !overlap {
				// No overlap, keep the old cub.
				newCubs = append(newCubs, currCub)
			} /* else {
				// Otherwise, there was full overlap. Don't add anything back. The current cub will
				// be fully contained within the new cub. If this is not an addition but a removal,
				// then the odl cub will be removed entirely.
			}*/
		}
		if swit {
			newCubs = append(newCubs, cub)
		}
		currentCubs = newCubs
	}
	fmt.Println("Number of cubes marked:", totalSize(currentCubs))
}

// end::solution[]
