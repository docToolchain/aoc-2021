package main

import (
	"fmt"
	"log"

	"gonum.org/v1/gonum/mat"
)

const (
	numReqMatches = 12
	// tol           = 1e-7
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

func findMatchIndices(posRef, posCheck []mat.Vector) map[int]struct{} {
	matches := make(map[int]struct{}, len(posCheck))
	for _, ref := range posRef {
		for checkIdx, check := range posCheck {
			// Try actual equality first, we might need to switch to EqualApprox, which is something
			// like an all-close.
			if mat.Equal(ref, check) {
				matches[checkIdx] = struct{}{}
				break
			}
		}
	}
	return matches
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

func assessMatch(rot *mat.Dense, refPositions, checkPositions []mat.Vector) (mat.Vector, bool) {
	rotated := rotate(rot, checkPositions)
	for _, refPosRef := range refPositions {
		diff := mat.NewVecDense(dims, nil)

		for _, refPosCheck := range rotated {
			diff.SubVec(refPosRef, refPosCheck)
			// printMat(rot)
			// printVecs(diff)
			// printVecs(refPosRef)
			// printVecs(refPosCheck)

			checkPosTranslated := translate(diff, rotated)
			// printVec(checkPosTranslated)

			matches := countMatches(refPositions, checkPosTranslated)
			if matches >= numReqMatches {
				return diff, true
			}
		}
	}
	return nil, false
}

func printMat(matrix *mat.Dense) {
	for row := 0; row < dims; row++ {
		fmt.Println(matrix.RowView(row))
	}
	fmt.Println()

}

func printVecs(vec mat.Vector) {
	fmt.Println(vec)
	fmt.Println()

}

func printVec(vecs []mat.Vector) {
	for _, vec := range vecs {
		fmt.Println(vec)
	}
	fmt.Println()

}

func mergeOneMatch(data map[int][]mat.Vector, rots []*mat.Dense) bool {
	for refIdx, ref := range data {
		for checkIdx, check := range data {
			if checkIdx == refIdx {
				// Don't merge a sensor's data with its own data.
				continue
			}
			for _, rot := range rots {
				if diff, matching := assessMatch(rot, ref, check); matching {
					// Merge start.
					log.Println("merging", checkIdx, "into", refIdx)

					adjusted := translate(diff, rotate(rot, check))

					// Determine matching sensor data.
					matchingIdx := findMatchIndices(ref, adjusted)
					if len(matchingIdx) < numReqMatches {
						log.Fatalf("found only %d matches", len(matchingIdx))
					}

					// Keep all the data from the base list.
					// Only add those not yet present from the merge list.
					for mergeIdx, merge := range adjusted {
						if _, ok := matchingIdx[mergeIdx]; !ok {
							ref = append(ref, merge)
						}
					}

					// Remember the updated base data and remove the data that has been merged in.
					data[refIdx] = ref
					delete(data, checkIdx)

					return true
					// Merge end.
				}
			}
		}
	}
	return false
}

func main() {
	// Create all valid rotation matrices.
	rots := allRots()
	for _, rot := range rots {
		printMat(rot)
	}

	// Read data.
	sensorData, err := ReadLinesAsSensorData()
	if err != nil {
		log.Fatal(err.Error())
	}
	for _, data := range sensorData {
		printVec(data)
	}

	// Continuously merge sensor data until only one remains.
	for len(sensorData) != 1 {
		merged := mergeOneMatch(sensorData, rots)
		if !merged {
			log.Fatal("did not find any match")
		}
	}

	for _, dat := range sensorData {
		fmt.Println("There are", len(dat), "beacons in total.")
	}

	// This line stops Go complaining that this helper function was unused.
	_ = printVecs
}
