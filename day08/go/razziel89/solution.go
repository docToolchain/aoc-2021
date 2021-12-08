package main

import (
	"fmt"
	"log"
)

const (
	baseStr = "abcdefg"
)

// UnjumbleFn is what you get by callind GetStrMapFn.
type UnjumbleFn = func(string) (string, error)

// GetStrMapFn returns a function that, when applied to a string, will apply the same mapping that
// would be needed to transform jumbled into base.
func GetStrMapFn(base, jumbled string) (UnjumbleFn, error) {
	// Sanity check.
	if SortString(base) != SortString(jumbled) {
		err := fmt.Errorf("jumbled string could not be matched to base")
		return func(string) (string, error) { return "", err }, err
	}

	unjumble := map[rune]rune{}
	for idx, char := range jumbled {
		unjumble[char] = rune(base[idx])
	}

	mapFn := func(input string) (string, error) {
		result := make([]rune, 0, len(input))
		for _, char := range input {
			mapped, ok := unjumble[char]
			if !ok {
				return "", fmt.Errorf("don't know what to do with %c", char)
			}
			result = append(result, mapped)
		}
		return string(result), nil
	}

	return mapFn, nil
}

func getSolution(
	numsAsStrs []string,
	numOutputs int,
	stringToNumber func(string) (int, bool, error),
) (int, bool, error) {
	sum := 0
	firstOutputIdx := len(numsAsStrs) - numOutputs
	for idx, numAsStr := range numsAsStrs {
		num, ok, err := stringToNumber(numAsStr)
		if err != nil {
			return 0, false, err
		}
		if !ok {
			return 0, false, nil
		}
		if idx > firstOutputIdx {
			sum += Pow(10, numOutputs-(idx-firstOutputIdx)) * num //nolint:gomnd
		}
	}
	return sum, true, nil
}

// Taken from the puzzle. This maps each segment list with segments identified by the letters a
// through g to the corresponding number. This uses the same assignment as in the task, namely this:
//
//    aaaa
//   b    c
//   b    c
//    dddd
//   e    f
//   e    f
//    gggg
//
// Using SortString here is overkill but I want to make sure the strings are sorted.
// nolint:gomnd
var numForStr = map[string]int{
	SortString("abcefg"):  0,
	SortString("cf"):      1,
	SortString("acdeg"):   2,
	SortString("acdfg"):   3,
	SortString("bcdf"):    4,
	SortString("abdfg"):   5,
	SortString("abdefg"):  6,
	SortString("acf"):     7,
	SortString("abcdefg"): 8,
	SortString("abcdfg"):  9,
}

func main() {
	numbers, outputCount, err := ReadLinesAsInputsAndOutputs()
	if err != nil {
		log.Fatal(err.Error())
	}
	if SortString(baseStr) != baseStr {
		log.Fatal(fmt.Errorf("base string is not sorted"))
	}
	// Remember the score for each solution. A score of >=0 means a solution for this particular
	// line has been found.
	solutions := make([]int, len(numbers))
	for idx := 0; idx < len(solutions); idx++ {
		solutions[idx] = -1
	}
	for perm := range AllPermutations(baseStr) {

		// Define a function that will unjumble a string based on a permutation.
		unsortedMapFn, err := GetStrMapFn(baseStr, perm)
		if err != nil {
			log.Fatal(err.Error())
		}
		// Directly try to convert a string to a number based on a permutation. The returned bool
		// indicates whether a mapping could be constructed.
		stringToNumber := func(str string) (int, bool, error) {
			unsorted, err := unsortedMapFn(str)
			if err != nil {
				return 0, false, err
			}
			// The entries in numForStr are sorted character-wise. Thus, sort before putting it into
			// the map.
			sorted := SortString(unsorted)
			num, ok := numForStr[sorted]
			return num, ok, nil
		}

		for solIdx := 0; solIdx < len(numbers); solIdx++ {
			if solutions[solIdx] != -1 {
				// We've already found that solution.
				continue
			}

			sol, ok, err := getSolution(numbers[solIdx], outputCount, stringToNumber)
			if err != nil {
				log.Fatal(err.Error())
			}
			if ok {
				solutions[solIdx] = sol
			}
		}
	}
	fmt.Println(solutions)
	sum := 0
	for _, num := range solutions {
		sum += num
	}
	fmt.Println(sum)
}
