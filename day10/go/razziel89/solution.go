// Package main is the main executable for razziel89's solution for this day's advent.
package main

import (
	"fmt"
	"log"
	"sort"
	"strings"
)

// tag::solution[]

var openerCloser = map[string]string{
	"(": ")",
	"[": "]",
	"{": "}",
	"<": ">",
}

//nolint:gomnd
var delimScorePart1 = map[string]int{
	")": 3,
	"]": 57,
	"}": 1197,
	">": 25137,
}

//nolint:gomnd
const (
	multScorePart2 = 5
)

//nolint:gomnd
var delimScorePart2 = map[string]int{
	")": 1,
	"]": 2,
	"}": 3,
	">": 4,
}

// Check the syntax. Return whether the syntax was OK, the sequence of expected closing delimiters,
// and the first offending character.
func checkSyntax(line string) (bool, Stack, string) {
	stack := Stack{}
	for _, char := range strings.Split(line, "") {
		if closer, isOpener := openerCloser[char]; isOpener {
			// We found a new opener. Add the corresponding closer to the stack.
			stack.Push(closer)
		} else {
			// We did not find an opener. Check whether the closer we found is the expected one.
			expectedCloser, ok := stack.Pop()
			if !ok {
				// The stack was empty. An empty offender means the line had more closing delimiters
				// than opening ones.
				return false, stack, ""
			}
			if expectedCloser != char {
				// We did not find the closer we expected. The current char is the offender.
				return false, stack, char
			}
		}
	}
	return true, stack, ""
}

//nolint: funlen
func main() {
	lines, err := ReadLines()
	if err != nil {
		log.Fatal(err.Error())
	}
	remainingLines := []string{}
	// Part 1.
	scorePart1 := 0
	for _, line := range lines {
		ok, _, offender := checkSyntax(line)
		// The second condition means we currently only check corrupted lines but not such ones that
		// are incomplete.
		if !ok && offender != "" {
			newScore, found := delimScorePart1[offender]
			if !found {
				log.Fatalf("cannot find score for offender %s", offender)
			}
			scorePart1 += newScore
		} else {
			// Keep all lines that are not corrupted.
			remainingLines = append(remainingLines, line)
		}
	}
	fmt.Printf("Score is %d points.\n", scorePart1)
	// Part 1.
	scoresPart2 := []int{}
	for _, line := range remainingLines {
		lineScore := 0
		ok, remainder, _ := checkSyntax(line)
		if !ok {
			// We wanted to filter out all corrupted lines already. Thus, finding one here is a bug.
			log.Fatalf("found corrupted line '%s' but we have filtered them all", line)
		}
		// Iterate over all the remaining characters.
		for char, nonEmpty := remainder.Pop(); nonEmpty; char, nonEmpty = remainder.Pop() {
			lineScore *= multScorePart2
			newScore, found := delimScorePart2[char]
			if !found {
				log.Fatalf("cannot find score for character %s", char)
			}
			lineScore += newScore
		}
		scoresPart2 = append(scoresPart2, lineScore)
	}
	// Pick the middle one.
	sort.Ints(scoresPart2)
	if len(scoresPart2)%2 != 1 {
		log.Fatal("we were promised an odd number of scores but found an even one")
	}
	// Integer division FTW.
	scorePart2 := scoresPart2[len(scoresPart2)/2]
	fmt.Printf("Score is %d points.\n", scorePart2)
}

// end::solution[]
