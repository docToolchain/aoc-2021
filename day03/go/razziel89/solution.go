// Package main is the main executable for razziel89's solution for this day's advent.
package main

import (
	"fmt"
	"log"
	"strconv"
	"strings"
)

// tag::solution[]

const (
	base = 2
)

func filterForGenerator(sli []string, set CountingSet) []bool {
	filter := set.MostCommon("1")
	return FilterBy(sli, filter)
}

func filterForScrubber(sli []string, set CountingSet) []bool {
	filter := set.LeastCommon("0")
	return FilterBy(sli, filter)
}

// This function fatals if an error is detected.
func mustConvertBinarySliceToInt(sli []string) int {
	str := strings.Join(sli, "")
	return mustConvertBinaryToInt(str)
}

// This function fatals if an error is detected.
func mustConvertBinaryToInt(str string) int {
	val, err := strconv.ParseInt(str, base, 0)
	if err != nil {
		log.Fatal(err.Error())
	}
	return int(val)
}

//nolint: funlen
func main() {
	// Read input.
	binaryNumsAsStrings, err := ReadLines()
	if err != nil {
		log.Fatalf("cannot read binary numbers from stdin due to %v", err.Error())
	}
	if len(binaryNumsAsStrings) == 0 {
		log.Fatal("no input provided")
	}
	counts, err := CountTokens(binaryNumsAsStrings)
	if err != nil {
		log.Fatal(err.Error())
	}

	// First part.
	epsilonSli := make([]string, 0, len(counts))
	gammaSli := make([]string, 0, len(counts))
	// Map the least common and most common operators to the obtained counting sets.
	for _, sli := range counts {
		// Epsilon
		newEpsilonDigit := sli.LeastCommon("0")
		epsilonSli = append(epsilonSli, newEpsilonDigit)
		// Gamma
		newGammaDigit := sli.MostCommon("1")
		gammaSli = append(gammaSli, newGammaDigit)
	}
	epsilon := mustConvertBinarySliceToInt(epsilonSli)
	gamma := mustConvertBinarySliceToInt(gammaSli)
	fmt.Printf("Counts are %v\n", counts)
	fmt.Printf("Gamma is %v and %d\n", gammaSli, gamma)
	fmt.Printf("Epsilon is %v and %d\n", epsilonSli, epsilon)
	fmt.Printf("Product of both numbers is %d\n", epsilon*gamma)

	// Second part. Here comes the fun.
	generatorRatingStr, err := FilterCounts(binaryNumsAsStrings, filterForGenerator)
	if err != nil {
		log.Fatal(err.Error())
	}
	scrubberRatingStr, err := FilterCounts(binaryNumsAsStrings, filterForScrubber)
	if err != nil {
		log.Fatal(err.Error())
	}
	generatorRating := mustConvertBinaryToInt(generatorRatingStr)
	scrubberRating := mustConvertBinaryToInt(scrubberRatingStr)
	fmt.Printf("Generator rating is %d\n", generatorRating)
	fmt.Printf("Scrubber rating is %d\n", scrubberRating)
	fmt.Printf("Product of both ratings is %d\n", generatorRating*scrubberRating)
}

// end::solution[]
