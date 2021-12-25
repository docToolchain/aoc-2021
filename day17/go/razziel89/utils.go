package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"strings"
)

const (
	numExpectedFields = 4
)

var (
	areaStart = []string{"target", "area:"}
)

var reader = bufio.NewReader(os.Stdin)

// Function readLine reads one line from stdin via a global reader instance.
func readLine() (string, error) {
	return reader.ReadString('\n')
}

func trimStrings(sli []string) []string {
	result := make([]string, 0, len(sli))
	for _, val := range sli {
		result = append(result, strings.TrimSpace(val))
	}
	return result
}

// tag::utils[]

// ReadLinesAsAreas reads all lines from stdin as areas.
func ReadLinesAsAreas() ([]Area, error) {
	var result []Area
	for {
		line, err := readLine()
		if err == io.EOF {
			// Success case, no more input to read.
			return result, nil
		}
		if err != nil {
			return []Area{}, err
		}
		fields := trimStrings(strings.Fields(line))
		if len(fields) == 0 {
			// Skip empty lines.
			continue
		}
		if len(fields) != numExpectedFields {
			return []Area{}, fmt.Errorf("unexpected number of fields")
		}
		if fields[0] != areaStart[0] || fields[1] != areaStart[1] {
			return []Area{}, fmt.Errorf("unexpected start of line")
		}
		area, err := AreaFromString(strings.Join(fields[2:], " "))
		if err != nil {
			return []Area{}, err
		}
		result = append(result, area)
	}
}

// end::utils[]
