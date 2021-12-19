package main

import (
	"fmt"
	"log"

	"gonum.org/v1/gonum/mat"
)

func main() {
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
	_ = allRots()
}
