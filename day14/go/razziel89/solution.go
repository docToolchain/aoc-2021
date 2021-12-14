// Package main is the main executable for razziel89's solution for this day's advent.
package main

import (
	"fmt"
	"log"
)

// tag::solution[]

const (
	// We make the buffer large to achieve some degree of parallelism with goroutines.
	buffer      = 100000
	roundsPart1 = 10
	roundsPart2 = 40
)

func makeMap(replacements []Replacement) map[Match]rune {
	result := map[Match]rune{}
	for _, rep := range replacements {
		result[rep.Match] = rep.Insert
	}
	return result
}

func replace(input <-chan rune, output chan<- rune, replacements map[Match]rune, idx int) {
	myReps := map[Match]rune{}
	// Copy the map so that each goroutine has its own.
	for key, val := range replacements {
		myReps[key] = val
	}
	lastRune := <-input
	for char := range input {
		output <- lastRune
		if match, ok := myReps[Match{Left: lastRune, Right: char}]; ok {
			output <- match
		}
		lastRune = char
	}
	output <- lastRune
	close(output)
	fmt.Printf("Closing stage %d.\n", idx)
}

func feed(input string, channel chan<- rune) {
	for _, char := range input {
		channel <- char
	}
	close(channel)
	fmt.Println("Closing feeder.")
}

func count(input <-chan rune) map[rune]int {
	result := map[rune]int{}
	for char := range input {
		result[char]++
	}
	return result
}

func max(input map[rune]int) int {
	found := false
	var max int
	for _, val := range input {
		if !found || val > max {
			found = true
			max = val
		}
	}
	return max
}

func min(input map[rune]int) int {
	found := false
	var min int
	for _, val := range input {
		if !found || val < min {
			found = true
			min = val
		}
	}
	return min
}

// Run the polymerization riddle for a given number of rounds. Note how we never construct the full
// string in memory.
func runRounds(input string, reps map[Match]rune, rounds int) map[rune]int {
	// Construct pipeline.
	inChannel := make(chan rune, buffer)
	var outChannel chan rune
	// Run pipeline.
	go feed(input, inChannel)
	for roundIdx := 0; roundIdx < rounds; roundIdx++ {
		outChannel = make(chan rune, buffer)
		go replace(inChannel, outChannel, reps, roundIdx)
		inChannel = outChannel
	}
	counts := count(outChannel)
	return counts
}

//nolint: funlen
func main() {
	input, replacements, err := ReadLinesAsReplacementsOrInput()
	if err != nil {
		log.Fatal(err.Error())
	}
	reps := makeMap(replacements)
	counts := runRounds(input, reps, roundsPart1)
	fmt.Printf(
		"Part 1: max: %d, min: %d, diff: %d\n",
		max(counts), min(counts), max(counts)-min(counts),
	)
	counts = runRounds(input, reps, roundsPart2)
	fmt.Printf(
		"Part 2: max: %d, min: %d, diff: %d\n",
		max(counts), min(counts), max(counts)-min(counts),
	)
}

// end::solution[]
