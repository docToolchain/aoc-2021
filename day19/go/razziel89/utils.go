package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"strings"

	"gonum.org/v1/gonum/mat"
)

const (
	scannerStart  = '-'
	scannerFormat = "--- scanner %d ---"
	vectorFormat  = "%f,%f,%f"
	vectorDims    = 3
)

var reader = bufio.NewReader(os.Stdin)

// Function readLine reads one line from stdin via a global reader instance.
func readLine() (string, error) {
	return reader.ReadString('\n')
}

// tag::utils[]

// ReadLinesAsSensorData reads line in as sensor data.
func ReadLinesAsSensorData() ([][]mat.Vector, error) {
	var result [][]mat.Vector
	var data []mat.Vector
	for {
		line, err := readLine()
		if err == io.EOF {
			// Success case, no more input to read.
			return result, nil
		}
		if err != nil {
			return [][]mat.Vector{}, err
		}
		line = strings.TrimSpace(line)
		//nolint:nestif
		if len(line) == 0 {
			result = append(result, data)
		} else if line[0] == scannerStart && line[1] == scannerStart && line[2] == scannerStart {
			var scannerIdx int
			count, err := fmt.Sscanf(line, scannerFormat, &scannerIdx)
			if err != nil {
				return [][]mat.Vector{}, err
			}
			if scannerIdx != len(result) || count != 1 {
				err := fmt.Errorf("error parsing line %s", line)
				return [][]mat.Vector{}, err
			}
			data = []mat.Vector{}
		} else {
			var vecData [vectorDims]float64
			count, err := fmt.Sscanf(line, vectorFormat, &vecData[0], &vecData[1], &vecData[2])
			if err != nil {
				return [][]mat.Vector{}, err
			}
			if count != vectorDims {
				err := fmt.Errorf("error parsing line %s", line)
				return [][]mat.Vector{}, err
			}
			data = append(data, mat.NewVecDense(vectorDims, vecData[:]))
		}
	}
}

// end::utils[]
