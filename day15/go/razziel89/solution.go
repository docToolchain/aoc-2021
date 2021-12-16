// Package main is the main executable for razziel89's solution for this day's advent.
package main

import (
	"fmt"
	"log"

	"github.com/razziel89/astar"
)

// tag::solution[]

const (
	numNeigh = 4
	replicas = 5
)

func gridToNodes(grid Grid) (map[*astar.Node]struct{}, *astar.Node, *astar.Node, error) {
	result := map[*astar.Node]struct{}{}
	var start, end *astar.Node
	// Remember which node belonged to which location.
	convertedNodes := map[Vec]*astar.Node{}

	startX, startY := grid.TopLeft()
	endX, endY := grid.BottomRight()
	startVec := Vec{startX, startY}
	endVec := Vec{endX, endY}

	// Process the nodes themselves.
	for vec, cost := range grid {
		if _, ok := convertedNodes[vec]; ok {
			err := fmt.Errorf("node already present")
			return result, start, end, err
		}

		node, err := astar.NewNode(fmt.Sprint(vec), cost, numNeigh)
		if err != nil {
			return result, start, end, err
		}

		convertedNodes[vec] = node
		result[node] = struct{}{}

		if vec == startVec {
			start = node
		}
		if vec == endVec {
			end = node
		}
	}
	if start == nil || end == nil {
		err := fmt.Errorf("cannot find start or end node")
		return result, start, end, err
	}
	// Add pairwise connections. This might take a while.
	for vec := range grid {
		node, ok := convertedNodes[vec]
		if !ok {
			err := fmt.Errorf("node not found")
			return result, start, end, err
		}
		for neigh := range pointEnv(vec) {
			if !grid.Has(neigh) {
				// Ignore nodes outside the grid.
				continue
			}
			neighNode, ok := convertedNodes[neigh]
			if !ok {
				err := fmt.Errorf("node not found")
				return result, start, end, err
			}
			// If a connection already exists, this is a no-op.
			node.AddConnection(neighNode)
			neighNode.AddConnection(node)
		}
	}
	return result, start, end, nil
}

//nolint: funlen
func main() {
	grid, err := ReadLinesAsGrid()
	if err != nil {
		log.Fatal(err.Error())
	}
	// Part 1.
	nodes, startNode, endNode, err := gridToNodes(grid)
	if err != nil {
		log.Fatal(err.Error())
	}

	fastAlgorithm(nodes, startNode, endNode)

	startX, startY := grid.TopLeft()
	endX, endY := grid.BottomRight()

	// Part 2.
	// Replicate grid. Then, run the fast algorithm
	largeGrid := Grid{}
	for p := range grid {
		for xRep := 0; xRep < replicas; xRep++ {
			for yRep := 0; yRep < replicas; yRep++ {
				newPoint := Vec{
					x: p.x + xRep*(endX-startX+1),
					y: p.y + yRep*(endY-startY+1),
				}
				newMarking := (grid.Count(p)-1+xRep+yRep)%9 + 1 //nolint:gomnd
				if largeGrid.Has(newPoint) {
					// Sanity check, we should never add a node twice.
					log.Fatal("node already there")
				}
				_ = largeGrid.Mark(newPoint, newMarking)
			}
		}
	}

	nodes, startNode, endNode, err = gridToNodes(grid)
	if err != nil {
		log.Fatal(err.Error())
	}

	fastAlgorithm(nodes, startNode, endNode)
}

// end::solution[]
