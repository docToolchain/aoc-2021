package main

import (
	"gonum.org/v1/gonum/mat"
)

const (
	dims = 3
)

func allRots() []mat.Matrix {
	result := []mat.Matrix{}

	indices := []mat.Vector{
		mat.NewVecDense(dims, []float64{0., 1., 2.}),
		mat.NewVecDense(dims, []float64{0., 2., 1.}),
		mat.NewVecDense(dims, []float64{1., 0., 2.}),
		mat.NewVecDense(dims, []float64{1., 2., 0.}),
		mat.NewVecDense(dims, []float64{2., 0., 1.}),
		mat.NewVecDense(dims, []float64{2., 1., 0.}),
	}

	signs := []mat.Vector{
		mat.NewVecDense(dims, []float64{1., 1., 1.}),
		mat.NewVecDense(dims, []float64{1., 1., -1.}),
		mat.NewVecDense(dims, []float64{1., -1., 1.}),
		mat.NewVecDense(dims, []float64{1., -1., -1.}),
		mat.NewVecDense(dims, []float64{-1., 1., 1.}),
		mat.NewVecDense(dims, []float64{-1., 1., -1.}),
		mat.NewVecDense(dims, []float64{-1., -1., 1.}),
		mat.NewVecDense(dims, []float64{-1., -1., -1.}),
	}

	for _, targetIdx := range indices {
		for _, sign := range signs {
			rotMatIdx := mat.VecDenseCopyOf(targetIdx)
			rotMatIdx.MulElemVec(rotMatIdx, sign)

			rotMat := mat.NewDense(dims, dims, []float64{0., 0., 0., 0., 0., 0., 0., 0., 0.})

			for orgDim := 0; orgDim < dims; orgDim++ {
				targetDim := targetIdx.AtVec(orgDim)
				sig := sign.AtVec(orgDim)
				rotMat.Set(int(orgDim), int(targetDim), sig)
			}
			result = append(result, rotMat)
		}
	}

	return result
}
