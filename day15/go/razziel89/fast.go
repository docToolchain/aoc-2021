package main

// tag::fast[]

// This is an implementation using an A* algorithm implemented by me.

import (
	"fmt"
	"log"

	"github.com/razziel89/astar"
)

func fastAlgorithm(nodes map[*astar.Node]struct{}, start, end *astar.Node) {
	// Build heuristic that measures distance to end node.
	heuristic := astar.ConstantHeuristic{}
	// Extract payload. We know that these are the coordinates.
	endVec := end.Payload.(Vec)
	for node := range nodes {
		vec := node.Payload.(Vec)
		// The estimate assumes a cost of 1 for the most direct connection.
		estimate := (vec.x - endVec.x) + (vec.y - endVec.y)
		err := heuristic.AddNode(node, estimate)
		if err != nil {
			log.Fatal(err.Error())
		}
	}
	// Plot path.
	path, err := astar.FindPath(nodes, start, end, heuristic.Heuristic(0))
	if err != nil {
		log.Fatal(err.Error())
	}
	// Compute cost.
	cost := 0
	for _, node := range path[1:] {
		fmt.Println(node.ToString())
		cost += node.Cost
	}
	fmt.Println("Cost is", cost)
}

// end::fast[]
