package main

import (
	"fmt"
	"log"
	"sort"
	"strings"
)

func mapFromPerm(baseStr string, perm []string) (map[string]string, error) {
	result := map[string]string{}
	if len(baseStr) != len(perm) {
		err := fmt.Errorf("permutation has unexpected length")
		return map[string]string{}, err
	}
	for _, char := range baseStr {
		seekChar := string(char)
		found := false
		for _, checkChar := range perm {
			if seekChar == checkChar {
				found = true
				break
			}
		}
		if !found {
			err := fmt.Errorf("incomplete permutation")
			return map[string]string{}, err
		}
	}
	for idx := 0; idx < len(baseStr); idx++ {
		char := string(baseStr[idx])
		result[perm[idx]] = char
	}
	if len(result) != len(baseStr) {
		err := fmt.Errorf("duplicate entries in permutation")
		return map[string]string{}, err
	}
	return result, nil
}

func applyMap(str string, myMap map[string]string) ([]string, error) {
	split := strings.Split(str, "")
	for idx := 0; idx < len(split); idx++ {
		mapped, ok := myMap[string(str[idx])]
		if !ok {
			return []string{}, fmt.Errorf("invalid character")
		}
		split[idx] = mapped
	}
	return split, nil
}

func getSolution(
	input []string, outputCount int, myMap map[string]string, numForStr map[string]int,
) (int, error) {
	nums := make([]int, 0, len(input))
	for _, in := range input {
		mapped, err := applyMap(in, myMap)
		if err != nil {
			return -1, fmt.Errorf("cannot map")
		}
		sort.Strings(mapped)
		converted := strings.Join(mapped, "")
		num, ok := numForStr[converted]
		if !ok {
			// The mapped string does not describe a valid number. This "solution" is thus not valid
			// here.
			return -1, nil
		}
		nums = append(nums, num)
	}
	result := 0
	for idx, digit := range nums[len(nums)-outputCount:] {
		result += pow(10, outputCount-idx-1) * digit // nolint:gomnd
	}
	return result, nil
}

const (
	baseStr = "abcdefg"
)

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
		myMap, err := mapFromPerm(baseStr, perm)
		if err != nil {
			log.Fatal(err.Error())
		}
		for solIdx := 0; solIdx < len(numbers); solIdx++ {
			if solutions[solIdx] != -1 {
				// We've already found that solution.
				continue
			}
			sol, err := getSolution(numbers[solIdx], outputCount, myMap, numForStr)
			if err != nil {
				log.Fatal(err.Error())
			}
			if sol >= 0 {
				// This is a valid mapping for this line, remember the output number.
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
