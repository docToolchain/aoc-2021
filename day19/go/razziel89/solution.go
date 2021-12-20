package main

import (
	"fmt"
	"log"

	"gonum.org/v1/gonum/mat"
)

// tag::solution[]

const (
	numReqMatches = 12
	maxDist       = 1000.
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

func countMatches(posRef, posCheck, refSens, checkSens []mat.Vector) int {
	matches := 0
	for _, ref := range posRef {
		matched := false
		for _, check := range posCheck {
			// Try actual equality first, we might need to switch to EqualApprox, which is something
			// like an all-close.
			if mat.Equal(ref, check) {
				matched = true
				matches++
				break
			}
		}
		// // Following is a sanity check that is supposed to ensure that no point that is not
		// // matched is within 1000 of another sensor. Switching it on causes there to be no
		// // matches. The implementation is likely incorrect.
		// if !matched {
		// 	for _, sens := range refSens {
		// 		for _, check := range posCheck {
		// 			// Check whether any check pos is closer to any of the ref sensors than maxDist.
		// 			// If so, this cannot be a match.
		// 			dist := mat.NewVecDense(dims, nil)
		// 			dist.SubVec(check, sens)
		// 			//nolint:gomnd
		// 			if math.Abs(dist.AtVec(0)) <= maxDist && math.Abs(dist.AtVec(1)) <= maxDist &&
		// 				math.Abs(dist.AtVec(2)) <= maxDist {
		// 				// log.Println("excluded due to cube check")
		// 				return 0
		// 			}
		// 		}
		// 	}
		// 	for _, sens := range checkSens {
		// 		for _, ref := range posRef {
		// 			// Check whether any check pos is closer to any of the ref sensors than maxDist.
		// 			// If so, this cannot be a match.
		// 			dist := mat.NewVecDense(dims, nil)
		// 			dist.SubVec(ref, sens)
		// 			//nolint:gomnd
		// 			if math.Abs(dist.AtVec(0)) <= maxDist && math.Abs(dist.AtVec(1)) <= maxDist &&
		// 				math.Abs(dist.AtVec(2)) <= maxDist {
		// 				// log.Println("excluded due to cube check")
		// 				return 0
		// 			}
		// 		}
		// 	}
		// }
		_ = matched
		_ = maxDist
	}
	return matches
}

// Returns true if a match is found. Also returns the translation vector needed to translate
// one set of co-ordinates to the other to match.
func assessMatch(
	rot *mat.Dense, refPositions, checkPositions, refSens, checkSens []mat.Vector,
) (mat.Vector, bool) {
	rotated := rotate(rot, checkPositions)
	rotatedSens := rotate(rot, checkSens)
	for _, refPosRef := range refPositions {
		diff := mat.NewVecDense(dims, nil)

		for _, refPosCheck := range rotated {
			diff.SubVec(refPosRef, refPosCheck)

			checkPosTranslated := translate(diff, rotated)
			checkSensTranslated := translate(diff, rotatedSens)

			matches := countMatches(refPositions, checkPosTranslated, refSens, checkSensTranslated)
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

func printVec(vec mat.Vector) {
	fmt.Println(vec)
	fmt.Println()

}

func printVecs(vecs []mat.Vector) {
	for _, vec := range vecs {
		fmt.Println(vec)
	}
	fmt.Println()

}

func mergeOneMatch(data, sensorPositions map[int][]mat.Vector, rots []*mat.Dense) bool {
	for refIdx, ref := range data {
		for checkIdx, check := range data {
			if checkIdx == refIdx {
				// Don't merge a sensor's data with its own data.
				continue
			}
			log.Printf("assessing %d and %d", refIdx, checkIdx)
			for _, rot := range rots {
				refSens := sensorPositions[refIdx]
				checkSens := sensorPositions[checkIdx]

				if diff, matching := assessMatch(rot, ref, check, refSens, checkSens); matching {
					// Merge start.
					log.Println("merging", checkIdx, "into", refIdx)

					adjusted := translate(diff, rotate(rot, check))
					adjSens := translate(diff, rotate(rot, checkSens))

					// Determine matching sensor data.
					matchingIdx := findMatchIndices(ref, adjusted)
					if len(matchingIdx) < numReqMatches {
						log.Fatalf("found only %d matches", len(matchingIdx))
					}

					// Keep all the data from the base list.
					// Only add those not yet present from the merge list.
					for mergeIdx, merge := range adjusted {
						if _, exists := matchingIdx[mergeIdx]; !exists {
							ref = append(ref, merge)
						}
					}

					// Remember the updated base data and remove the data that has been merged in.
					data[refIdx] = ref
					delete(data, checkIdx)

					// Remember the translated sensor positions merged in for cube sanity checks.
					refSens = append(refSens, adjSens...)

					// Remember the updated base sensor and remove the sensor data of the merge.
					sensorPositions[refIdx] = refSens
					delete(sensorPositions, checkIdx)

					return true
					// Merge end.
				}
			}
		}
	}
	return false
}

func maxSensorDist(sensorData map[int][]mat.Vector) float64 {
	result := 0.
	for _, sensors := range sensorData {
		if len(sensors) <= 1 {
			// Don't process sensor data sets that don't yet have enough data.
			continue
		}
		for _, sens := range sensors {
			for _, otherSens := range sensors {
				diff := mat.NewVecDense(dims, nil)
				diff.SubVec(sens, otherSens)
				norm := mat.Norm(diff, 1)
				if norm > result {
					result = norm
				}
			}
		}
	}
	return result
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
		printVecs(data)
	}
	sensorPositions := map[int][]mat.Vector{}
	for idx := range sensorData {
		sensorPositions[idx] = []mat.Vector{mat.NewVecDense(dims, []float64{0., 0., 0.})}
	}

	// Continuously merge sensor data until only one remains.
	for len(sensorData) != 1 {
		merged := mergeOneMatch(sensorData, sensorPositions, rots)
		if !merged {
			log.Fatal("did not find any match")
		}
		log.Printf("%d remaining", len(sensorData))
		log.Println("max sensor dist is", maxSensorDist(sensorPositions))
	}

	for _, dat := range sensorData {
		fmt.Println("There are", len(dat), "beacons in total.")
	}

	// This line stops Go complaining that this helper function was unused.
	_ = printVec
}

// end::solution[]
