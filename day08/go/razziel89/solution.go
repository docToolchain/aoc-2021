package main

import (
	"fmt"
	"log"
	"sort"
	"strings"
)

const (
	baseStr = "abcdefg"
)

func permutations(str []string) [][]string {
	var helper func([]string, int)
	res := [][]string{}

	for _, char := range str {
		if len(char) != 1 {
			return [][]string{}
		}
	}

	helper = func(str []string, n int) {
		if n == 1 {
			tmp := make([]string, len(str))
			copy(tmp, str)
			res = append(res, tmp)
		} else {
			for i := 0; i < n; i++ {
				helper(str, n-1)
				if n%2 == 1 {
					tmp := str[i]
					str[i] = str[n-1]
					str[n-1] = tmp
				} else {
					tmp := str[0]
					str[0] = str[n-1]
					str[n-1] = tmp
				}
			}
		}
	}
	helper(str, len(str))
	return res
}

// nolint:gomnd
var numForStr = map[string]int{
	"abcefg":  0,
	"cf":      1,
	"acdeg":   2,
	"acdfg":   3,
	"bcdf":    4,
	"abdfg":   5,
	"abdefg":  6,
	"acf":     7,
	"abcdefg": 8,
	"abcdfg":  9,
}

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

func pow(base, exponent int) int {
	result := 1
	for exp := 0; exp < exponent; exp++ {
		result *= base
	}
	return result
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

func main() {
	numbers, outputCount, err := ReadLinesAsInputsAndOutputs()
	if err != nil {
		log.Fatal(err.Error())
	}
	solutions := make([]int, len(numbers))
	for idx := 0; idx < len(solutions); idx++ {
		solutions[idx] = -1
	}
	perms := permutations(strings.Split(baseStr, ""))
	for _, perm := range perms {
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
				fmt.Printf("found solution for line %d\n", solIdx+1)
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
