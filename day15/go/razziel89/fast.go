package main

// This is an implementation using an A* algorithm implemented by me.

import (
	"fmt"
	"log"

	"github.com/razziel89/astar"
)

func nameToCoords(name string) (int, int) {
	var endX, endY int
	// Extract end coordinates from the name.
	count, err := fmt.Sscanf(name, "{%d %d}", &endX, &endY)
	if err != nil {
		log.Fatal(err.Error())
	}
	if count != 2 { //nolint:gomnd
		log.Fatal("parsing error")
	}
	return endX, endY
}

func fastAlgorithm(nodes map[*astar.Node]struct{}, start, end *astar.Node) {
	// Build heuristic that measures distance to end node.
	heuristic := astar.ConstantHeuristic{}
	endX, endY := nameToCoords(end.ID)
	for node := range nodes {
		x, y := nameToCoords(node.ID)
		// The estimate assumes a cost of 1 for the most direct connection.
		estimate := (x - endX) + (y - endY)
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
