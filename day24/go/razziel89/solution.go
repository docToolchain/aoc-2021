package main

import (
	"fmt"
	"log"
)

// tag::solution[]

const (
	// Digits and levels for the brute-force search.
	smallestDigit = 1
	largestDigit  = 9
	firstLevel    = 0
	lastLevel     = 13
)

type acl struct {
	// w, x, y int
	z int
}

type fn = func(acl, int) acl

var funcs []fn

const ten = 10

func pow10(exp int) int {
	result := 1
	for count := 0; count < exp; count++ {
		result *= ten
	}
	return result
}

type cacheElem struct {
	state acl
	level int
}

var cache map[cacheElem]int

func findNum(inState acl, startDig, endDig, increment, level int) (int, bool) {
	cacheHit := cacheElem{
		state: inState,
		level: level,
	}
	if val, hit := cache[cacheHit]; hit {
		return val, val != 0
	}
	myFn := funcs[level]
	for dig := startDig; dig != endDig+increment; dig += increment {
		state := myFn(inState, dig)
		if level == lastLevel {
			if state.z == 0 {
				cache[cacheHit] = dig
				return dig, true
			}
		} else {
			num, valid := findNum(state, startDig, endDig, increment, level+1)
			if valid {
				cache[cacheHit] = dig
				return pow10(lastLevel-level)*dig + num, true
			}
		}
	}
	cache[cacheHit] = 0
	return 0, false
}

func main() {
	// Read ops in.
	ops, err := ReadLinesAsOps()
	if err != nil {
		log.Fatal(err.Error())
	}
	// Convert ops to functions we want to use.
	funcs, err = opsToFuncs(ops)
	if err != nil {
		log.Fatal(err.Error())
	}

	// Initialise the cache.
	cache = make(map[cacheElem]int)

	// Part 1, largest accepted model number.
	state := acl{} // Automatically zero-initialised.
	num, valid := findNum(state, largestDigit, smallestDigit, -1, firstLevel)
	if !valid {
		log.Fatal("no solution found")
	}
	fmt.Println("Largest model number is", num, "- cached", len(cache), "function calls")

	// Clear the cache in between.
	cache = make(map[cacheElem]int)

	// Part 2, smallest accepted model number.
	state = acl{}
	num, valid = findNum(state, smallestDigit, largestDigit, 1, firstLevel)
	if !valid {
		log.Fatal("no solution found")
	}
	fmt.Println("Smallest model number is", num, "- cached", len(cache), "function calls")
}

// end::solution[]
