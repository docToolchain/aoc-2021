package main

import (
	"fmt"
	"log"

	"gonum.org/v1/gonum/mat"
)

const (
	numReqMatches = 12
)

func rotate(rot *mat.Dense, vecs []mat.Vector) []mat.Vector {
	result := make([]mat.Vector, 0, len(vecs))
	for _, vec := range vecs {
		resultVec := mat.NewVecDense(dims, nil)
		resultVec.MulVec(rot, vec)
		result = append(result, resultVec)
	}
	return result
}

func translate(trans mat.Vector, vecs []mat.Vector) []mat.Vector {
	result := make([]mat.Vector, 0, len(vecs))
	for _, vec := range vecs {
		resultVec := mat.NewVecDense(dims, nil)
		resultVec.AddVec(trans, vec)
		result = append(result, resultVec)
	}
	return result
}

func countMatches(posRef, posCheck []mat.Vector) int {
	matches := 0
	for _, ref := range posRef {
		for _, check := range posCheck {
			// Try actual equality first, we might need to switch to EqualApprox, which is something
			// like an all-close.
			if mat.Equal(ref, check) {
				matches++
				break
			}
		}
	}
	return matches
}

func assessMatch(rot *mat.Dense, refPositions, checkPositions []mat.Vector) bool {
	rotated := rotate(rot, checkPositions)
	for _, refPosRef := range refPositions {
		diff := mat.NewVecDense(dims, nil)

		for _, refPosCheck := range rotated {
			diff.SubVec(refPosRef, refPosCheck)

			checkPosTranslated := translate(diff, checkPositions)

			matches := countMatches(refPositions, checkPosTranslated)
			if matches >= numReqMatches {
				return true
			}
		}
	}
	return false
}

func main() {
	// Create all valid rotation matrices.
	rots := allRots()
	for _, rot := range rots {
		for row := 0; row < dims; row++ {
			fmt.Println(rot.RowView(row))
		}
		fmt.Println()
	}

	// Read data.
	sensorData, err := ReadLinesAsSensorData()
	if err != nil {
		log.Fatal(err.Error())
	}
	for _, data := range sensorData {
		for _, vec := range data {
			fmt.Println(vec)
		}
		fmt.Println()
	}
	_ = mat.NewVecDense
	_ = assessMatch

	fmt.Println(assessMatch(rots[0], sensorData[0], sensorData[0]))
}
