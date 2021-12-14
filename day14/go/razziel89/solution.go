// Package main is the main executable for razziel89's solution for this day's advent.
package main

import (
	"fmt"
	"log"
	"strings"
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

// Split a string into pairs of characters. Each pair knows whether it's in the middle or at an
// edge. If it's at an edge, it also knows which one.
func stringToMatches(str string) []Match {
	result := []Match{}
	if len(str) == 0 {
		// This is an error case that will never happen. Thus, don't catch it.
		return []Match{}
	}
	split := strings.Split(str, "")
	lastChar := rune(split[0][0])
	for idx, c := range split[1:] {
		char := rune(c[0])
		var leftEdge, rightEdge bool
		if idx == 0 {
			leftEdge = true
		}
		if idx == len(split)-2 {
			rightEdge = true
		}
		match := Match{Left: lastChar, Right: char, LeftEdge: leftEdge, RightEdge: rightEdge}
		lastChar = char
		result = append(result, match)
	}
	return result
}

// Perform one pair split according to the rules. That is, a pair is converted into two pairs by
// inserting a letter in between. This also takes care of propagating the information of which
// matches are at the edges.
func translateMatch(match Match, reps *map[Match]rune) []Match {
	// make sure to strip away the edge information. Otherwise, matches at the edges will never be
	// found.
	matchWithoutEdge := Match{Right: match.Right, Left: match.Left}
	if insert, foundMatch := (*reps)[matchWithoutEdge]; foundMatch {
		leftMatch := Match{
			Left:      match.Left,
			Right:     insert,
			LeftEdge:  match.LeftEdge,
			RightEdge: false,
		}
		rightMatch := Match{
			Left:      insert,
			Right:     match.Right,
			LeftEdge:  false,
			RightEdge: match.RightEdge,
		}
		// fmt.Printf(
		// 	"Conv %s -> %s + %s\n",
		// 	match.ToString(), leftMatch.ToString(), rightMatch.ToString(),
		// )
		return []Match{leftMatch, rightMatch}
	}
	// No match found, return the original one.
	// fmt.Printf("Keep %s\n", match.ToString())
	return []Match{match}
}

// func printMatches(input map[Match]int) {
// 	fmt.Println()
// 	for match, count := range input {
// 		fmt.Printf("%d -> %s\n", count, match.ToString())
// 	}
// 	fmt.Println()
// }
//
// func printCounts(input map[rune]int) {
// 	fmt.Println()
// 	for char, count := range input {
// 		fmt.Printf("%c: %d\n", char, count)
// 	}
// 	fmt.Println()
// }

// Run the polymerization riddle for a given number of rounds, but do it differently. This approach
// does not scale exponentially with the number of rounds.
func runRoundsDifferently(input string, reps map[Match]rune, rounds int) map[rune]int {
	matches := stringToMatches(input)
	// We treat all identical matches the same way.
	matchCounts := map[Match]int{}
	for _, match := range matches {
		matchCounts[match]++
	}
	// Run!
	for round := 0; round < rounds; round++ {
		newMatchCounts := map[Match]int{}
		for match, count := range matchCounts {
			newMatches := translateMatch(match, &reps)
			for _, newM := range newMatches {
				newMatchCounts[newM] += count
			}
		}
		matchCounts = newMatchCounts
	}
	foundLeftEdge := false
	foundRightEdge := false
	// Count characters! Each char is in two pairs apart from the ones at the edges.
	result := map[rune]int{}
	// First, count all characters in all pairs. This will be twice the number we want.
	for match, count := range matchCounts {
		// Count each character in a pair for each pair that has it.
		result[match.Right] += count
		result[match.Left] += count
		// Handle edge cases (pun very much intended).
		if (match.LeftEdge || match.RightEdge) && count != 1 {
			// Error case, should never happen, so fatalling is OK if it does.
			log.Fatal("more than one character at the left edge")
		}
		// Also add one for each character at an edge. This makes sure we also counted them twice.
		if match.LeftEdge {
			if foundLeftEdge {
				log.Fatal("there can only be one left edge")
			}
			result[match.Left]++
		}
		if match.RightEdge {
			if foundRightEdge {
				log.Fatal("there can only be one right edge")
			}
			result[match.Right]++
		}
	}
	// Then, divide all counts by two.
	for char, count := range result {
		if count%2 != 0 {
			// Error case, should never happen, so fatalling is OK if it does.
			log.Fatalf("cannot divide odd number %d by 2 for %c", count, char)
		}
		result[char] /= 2
	}
	return result
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
	// Part 2 cannot be solved this way, it just takes too long.
	// counts = runRounds(input, reps, roundsPart2)
	counts = runRoundsDifferently(input, reps, roundsPart1)
	fmt.Printf(
		"Part 1 differently: max: %d, min: %d, diff: %d\n",
		max(counts), min(counts), max(counts)-min(counts),
	)
	counts = runRoundsDifferently(input, reps, roundsPart2)
	fmt.Printf(
		"Part 2 differently: max: %d, min: %d, diff: %d\n",
		max(counts), min(counts), max(counts)-min(counts),
	)
}

// end::solution[]
