// Package main is the main executable for razziel89's solution for this day's advent.
package main

import (
	"fmt"
	"log"
	"os"
	"strconv"
)

const (
	defaultWindowSize = 3
	windowEnvVarName  = "WINDOW_SIZE"
)

// tag::main[]

func main() {
	windowSize := defaultWindowSize
	if windowFromEnv, err := strconv.Atoi(os.Getenv(windowEnvVarName)); err == nil {
		log.Printf("Using window size %d from env var %s.", windowFromEnv, windowEnvVarName)
		windowSize = windowFromEnv
	}
	depths, err := ReadLinesAsInts()
	if len(depths) < windowSize {
		log.Fatalf("cannot process fewer numbers than %d but got %d", windowSize, len(depths))
	}
	if err != nil {
		log.Fatalf("cannot read depth values from stdin due to %v", err.Error())
	}
	depthsWithoutNoise := SlidingWindow(depths, windowSize, Sum)
	increments := CountIncrements(depthsWithoutNoise)
	fmt.Printf("Counted %d increments.\n", increments)
}

// end::main[]
