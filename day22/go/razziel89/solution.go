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
		// fmt.Println("no overlap")
		return []Cuboid{}, false
	}
	if inside == onCub {
		// In this case, one cuboid is completely outside the other one.
		// fmt.Println("no overlap")
		return []Cuboid{}, true
	}
	// fmt.Println("inside", inside)
	// fmt.Println("on    ", onCub)
	// fmt.Println("off   ", offCub)
	for _, dim := range [3]int{0, 1, 2} {
		// Negative direction.
		{
			newCub := Cuboid{start: inside.start, end: inside.end}
			newCub.start.Set(dim, onCub.start.Get(dim))
			newCub.end.Set(dim, inside.start.Get(dim))
			// fmt.Println(newCub)
			if newCub.Size() > 0 {
				result = append(result, newCub)
			}
		}
		// Positive direction.
		{
			newCub := Cuboid{start: inside.start, end: inside.end}
			newCub.start.Set(dim, inside.end.Get(dim))
			newCub.end.Set(dim, onCub.end.Get(dim))
			// fmt.Println(newCub)
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
				// fmt.Println(newCub)
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
				// fmt.Println(newCub)
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
				// fmt.Println(newCub)
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
				// fmt.Println(newCub)
				if newCub.Size() > 0 {
					result = append(result, newCub)
				}
			}
		}
	}
	// count == 18
	// Eight corners.
	// x>0 y>0 z>0
	{
		newCub := Cuboid{start: inside.end, end: onCub.end}
		// fmt.Println(newCub)
		if newCub.Size() > 0 {
			result = append(result, newCub)
		}
	}
	// x>0 y>0 z<0
	{
		newCub := Cuboid{start: inside.end, end: onCub.end}
		newCub.start.z = onCub.start.z
		newCub.end.z = inside.start.z
		// fmt.Println(newCub)
		if newCub.Size() > 0 {
			result = append(result, newCub)
		}
	}
	// x>0 y<0 z>0
	{
		newCub := Cuboid{start: inside.end, end: onCub.end}
		newCub.start.y = onCub.start.y
		newCub.end.y = inside.start.y
		// fmt.Println(newCub)
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
		// fmt.Println(newCub)
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
		// fmt.Println(newCub)
		if newCub.Size() > 0 {
			result = append(result, newCub)
		}
	}
	// x<0 y>0 z<0
	{
		newCub := Cuboid{start: onCub.start, end: inside.start}
		newCub.start.y = inside.end.y
		newCub.end.y = onCub.end.y
		// fmt.Println(newCub)
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
		// fmt.Println(newCub)
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

// func cubsToGrid(cubs []Cuboid) Grid {
// 	grid := Grid{data: map[Vec]bool{}, clamp: clampVal, background: false}
// 	for _, cub := range cubs {
// 		grid.MarkCuboid(cub, true)
// 	}
// 	return grid
// }

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

	fmt.Println(clampVal)
	fmt.Println(cubs)
	fmt.Println(switches)

	stop := 20

	grid := Grid{data: map[Vec]bool{}, clamp: clampVal, background: false}

	for idx, cub := range cubs {
		swit := switches[idx]
		grid.MarkCuboid(cub, swit)
		fmt.Println(idx, len(grid.data), cub.Size(), cub)
		if idx == stop {
			break
		}
	}

	fmt.Println()
	fmt.Println("Part 2")

	currentCubs := []Cuboid{cubs[0]}
	if !switches[0] {
		log.Fatal("expect first one to switch something on")
	}
	fmt.Println(0, totalSize(currentCubs))

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
				// fmt.Println("NO OVERLAP")
				newCubs = append(newCubs, currCub)
			}
			// Otherwise, there was full overlap. Don't add anything back. The current cub will be
			// fully contained within the new cub. If this is not an addition but a removal, then
			// the odl cub will be removed entirely.
		}
		if swit {
			newCubs = append(newCubs, cub)
		}
		currentCubs = newCubs
		fmt.Println("GENERAL", idx+1, totalSize(currentCubs), cub, swit)

		// if idx+1 == stop {
		// 	cubGrid := cubsToGrid(currentCubs)
		// 	fmt.Println(len(cubGrid.data), len(grid.data))
		// 	for p := range cubGrid.data {
		// 		if _, found := grid.data[p]; !found {
		// 			fmt.Println("too many", p)
		// 		}
		// 	}
		// 	for p := range grid.data {
		// 		if _, found := cubGrid.data[p]; !found {
		// 			fmt.Println("missing", p)
		// 		}
		// 	}
		// }

		// // Test for overlaps.
		// for refIdx, refCub := range currentCubs {
		// 	for checkIdx, checkCub := range currentCubs {
		// 		if refIdx == checkIdx {
		// 			continue
		// 		}
		// 		ol1 := refCub.Overlaps(checkCub)
		// 		ol2 := checkCub.Overlaps(refCub)
		// 		if ol1 || ol2 {
		// 			fmt.Println(refCub, refIdx, len(currentCubs))
		// 			fmt.Println(checkCub, checkIdx, len(currentCubs))
		// 			log.Fatal("overlapping cubs kept")
		// 		}
		// 	}
		// }
	}

	// // Disable clamping for part 2.
	// grid = Grid{data: map[Vec]bool{}, clamp: 0, background: false}

	// for idx, cub := range cubs {
	// 	swit := switches[idx]
	// 	grid.MarkCuboid(cub, swit)
	// 	fmt.Println(len(grid.data))
	// }
}

// end::solution[]
