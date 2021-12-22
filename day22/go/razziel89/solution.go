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
//nolint:funlen,gomnd
func eatChunk(onCub, offCub Cuboid, done map[int]bool) []Cuboid {
	count := 0
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
		return []Cuboid{}
	}
	// fmt.Println("inside", inside)
	for _, dim := range [3]int{0, 1, 2} {
		// Negative direction.
		{
			newCub := Cuboid{start: inside.start, end: inside.end}
			newCub.start.Set(dim, onCub.start.Get(dim))
			newCub.end.Set(dim, inside.start.Get(dim)-1)
			// fmt.Println(newCub)
			count++
			if newCub.Size() > 0 {
				result = append(result, newCub)
				done[count] = true
			}
		}
		// Positive direction.
		{
			newCub := Cuboid{start: inside.start, end: inside.end}
			newCub.start.Set(dim, inside.end.Get(dim)+1)
			newCub.end.Set(dim, onCub.end.Get(dim))
			// fmt.Println(newCub)
			count++
			if newCub.Size() > 0 {
				result = append(result, newCub)
				done[count] = true
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
				newCub.end.Set(dim, inside.start.Get(dim)-1)
				newCub.start.Set(nextDim, onCub.start.Get(nextDim))
				newCub.end.Set(nextDim, inside.start.Get(nextDim)-1)
				// fmt.Println(newCub)
				count++
				if newCub.Size() > 0 {
					result = append(result, newCub)
					done[count] = true
				}
			}
			// Positive nextDim direction
			{
				newCub := Cuboid{start: inside.start, end: inside.end}
				newCub.start.Set(dim, onCub.start.Get(dim))
				newCub.end.Set(dim, inside.start.Get(dim)-1)
				newCub.start.Set(nextDim, inside.end.Get(nextDim)+1)
				newCub.end.Set(nextDim, onCub.end.Get(nextDim))
				// fmt.Println(newCub)
				count++
				if newCub.Size() > 0 {
					result = append(result, newCub)
					done[count] = true
				}
			}
		}
		// Positive dim direction
		{
			// Negative nextDim direction
			{
				newCub := Cuboid{start: inside.start, end: inside.end}
				newCub.start.Set(dim, inside.end.Get(dim)+1)
				newCub.end.Set(dim, onCub.end.Get(dim))
				newCub.start.Set(nextDim, onCub.start.Get(nextDim))
				newCub.end.Set(nextDim, inside.start.Get(nextDim)-1)
				// fmt.Println(newCub)
				count++
				if newCub.Size() > 0 {
					result = append(result, newCub)
					done[count] = true
				}
			}
			// Positive nextDim direction
			{
				newCub := Cuboid{start: inside.start, end: inside.end}
				newCub.start.Set(dim, inside.end.Get(dim)+1)
				newCub.end.Set(dim, onCub.end.Get(dim))
				newCub.start.Set(nextDim, inside.end.Get(nextDim)+1)
				newCub.end.Set(nextDim, onCub.end.Get(nextDim))
				// fmt.Println(newCub)
				count++
				if newCub.Size() > 0 {
					result = append(result, newCub)
					done[count] = true
				}
			}
		}
	}
	// count == 18
	// Eight corners.
	// x>0 y>0 z>0
	{
		newCub := Cuboid{start: inside.end.Add(Vec{1, 1, 1}), end: onCub.end}
		// fmt.Println(newCub)
		count++
		if newCub.Size() > 0 {
			result = append(result, newCub)
			done[count] = true
		}
	}
	// x>0 y>0 z<0
	{
		newCub := Cuboid{start: inside.end.Add(Vec{1, 1, 1}), end: onCub.end}
		newCub.start.z = onCub.start.z
		newCub.end.z = inside.start.z - 1
		// fmt.Println(newCub)
		count++
		if newCub.Size() > 0 {
			result = append(result, newCub)
			done[count] = true
		}
	}
	// x>0 y<0 z>0
	{
		newCub := Cuboid{start: inside.end.Add(Vec{1, 1, 1}), end: onCub.end}
		newCub.start.y = onCub.start.y
		newCub.end.y = inside.start.y - 1
		// fmt.Println(newCub)
		count++
		if newCub.Size() > 0 {
			result = append(result, newCub)
			done[count] = true
		}
	}
	// x>0 y<0 z<0
	{
		newCub := Cuboid{start: inside.end.Add(Vec{1, 1, 1}), end: onCub.end}
		newCub.start.y = onCub.start.y
		newCub.end.y = inside.start.y - 1
		newCub.start.z = onCub.start.z
		newCub.end.z = inside.start.z - 1
		// fmt.Println(newCub)
		count++
		if newCub.Size() > 0 {
			result = append(result, newCub)
			done[count] = true
		}
	}
	// x<0 y<0 z<0
	{
		newCub := Cuboid{start: onCub.start, end: inside.start.Add(Vec{-1, -1, -1})}
		count++
		if newCub.Size() > 0 {
			result = append(result, newCub)
			done[count] = true
		}
	}
	// x<0 y<0 z>0
	{
		newCub := Cuboid{start: onCub.start, end: inside.start.Add(Vec{-1, -1, -1})}
		newCub.start.z = inside.end.z + 1
		newCub.end.z = onCub.end.z
		// fmt.Println(newCub)
		count++
		if newCub.Size() > 0 {
			result = append(result, newCub)
			done[count] = true
		}
	}
	// x<0 y>0 z<0
	{
		newCub := Cuboid{start: onCub.start, end: inside.start.Add(Vec{-1, -1, -1})}
		newCub.start.y = inside.end.y + 1
		newCub.end.y = onCub.end.y
		// fmt.Println(newCub)
		count++
		if newCub.Size() > 0 {
			result = append(result, newCub)
			done[count] = true
		}
	}
	// x<0 y>0 z>0
	{
		newCub := Cuboid{start: onCub.start, end: inside.start.Add(Vec{-1, -1, -1})}
		newCub.start.y = inside.end.y + 1
		newCub.end.y = onCub.end.y
		newCub.start.z = inside.end.z + 1
		newCub.end.z = onCub.end.z
		// fmt.Println(newCub)
		count++
		if newCub.Size() > 0 {
			result = append(result, newCub)
			done[count] = true
		}
	}
	return result
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

	fmt.Println(clampVal)
	fmt.Println(cubs)
	fmt.Println(switches)

	grid := Grid{data: map[Vec]bool{}, clamp: clampVal, background: false}

	for idx, cub := range cubs {
		swit := switches[idx]
		grid.MarkCuboid(cub, swit)
		fmt.Println(idx, len(grid.data), cub.Size(), cub)
	}

	fmt.Println()
	fmt.Println("Part 2")

	currentCubs := []Cuboid{cubs[0]}
	if !switches[0] {
		log.Fatal("expect first one to switch something on")
	}
	fmt.Println(0, totalSize(currentCubs))

	done := map[int]bool{}

	for idx, cub := range cubs[1:] {
		hereDone := map[int]bool{}
		swit := switches[idx+1]
		// Switch something on: Let the new chunk eat something out of all existing chunks.
		// Then, add the new chunk whole.
		// Switch something off: Let the new chunk eat something out of all existing chunks.
		// Only add those reduces chunks but don't add the new chunk.
		newCubs := []Cuboid{}
		for _, currCub := range currentCubs {
			// fmt.Println("START")
			// fmt.Println(cub)
			// fmt.Println(currCub)
			// fmt.Println("MID")
			chunks := eatChunk(currCub, cub, hereDone)
			if len(chunks) > 0 {
				newCubs = append(newCubs, chunks...)
			} else {
				// fmt.Println("no overlap outside")
				// fmt.Println(chunks)
				newCubs = append(newCubs, currCub)
			}
			// fmt.Println("END")
			// fmt.Println()
		}
		if swit {
			newCubs = append(newCubs, cub)
		}
		currentCubs = newCubs
		newDone := map[int]bool{}
		for action := range hereDone {
			if _, found := done[action]; !found {
				newDone[action] = true
			}
			done[action] = true
		}
		fmt.Println(idx+1, totalSize(currentCubs), cub, swit, newDone)
	}

	// // Disable clamping for part 2.
	// grid = Grid{data: map[Vec]bool{}, clamp: 0, background: false}

	// for idx, cub := range cubs {
	// 	swit := switches[idx]
	// 	grid.MarkCuboid(cub, swit)
	// 	fmt.Println(len(grid.data))
	// }
	_ = eatChunk
}

// end::solution[]
