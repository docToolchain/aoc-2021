package main

// This is an implementation using an A* algorithm implemented by me.

import (
	"fmt"
	"log"

	"github.com/razziel89/astar"
)

const (
	maxNumNeighbours = 4
)

func nodesToAstarNodes(nodes []*Node) ([]*astar.Node, error) {
	result := make([]*astar.Node, 0, len(nodes))
	for _, node := range nodes {
		if node.Limit != 1 {
			return []*astar.Node{}, fmt.Errorf("algorithm A* can only visit each node once")
		}
		newNode, err := astar.NewNode(node.ID, node.Value, maxNumNeighbours)
		if err != nil {
			return []*astar.Node{}, err
		}
		result = append(result, newNode)
	}
	// Add pairwise connections. This requires us to iterate quite often over the list of nodes
	// since the only properties nodes share are their IDs. Thus, we compare IDs.
	for idx, node := range nodes {
		astarNode := result[idx]
		for _, neighbour := range node.Connections {
			// Find the corresponding astar node.
			var astarNeighbour *astar.Node
			for _, possibleAstarNeighbour := range result {
				// Identify corresponding nodes via their IDs.
				if possibleAstarNeighbour.ID == neighbour.ID {
					astarNeighbour = possibleAstarNeighbour
					break
				}
			}
			if astarNeighbour == nil {
				return []*astar.Node{}, fmt.Errorf("cannot find neighbour")
			}
			// Add pairwise connections.
			astarNode.AddConnection(astarNeighbour)
			astarNeighbour.AddConnection(astarNode)
		}
	}
	return result, nil
}

func nameToCoords(name string) (int, int) {
	var endX, endY int
	// Extract end coordinates from the name.
	count, err := fmt.Sscanf(name, "%d-%d", &endX, &endY)
	if err != nil {
		log.Fatal(err.Error())
	}
	if count != 2 { //nolint:gomnd
		log.Fatal("parsing error")
	}
	return endX, endY
}

func fastAlgorithm(nodes []*Node, start, end string) {
	astarNodes, err := nodesToAstarNodes(nodes)
	if err != nil {
		log.Fatal(err.Error())
	}
	// Extract start and end nodes of both kinds.
	var startNode, endNode *astar.Node
	for _, node := range astarNodes {
		if node.ID == start {
			startNode = node
		}
		if node.ID == end {
			endNode = node
		}
	}
	endX, endY := nameToCoords(end)
	// Build heuristic that measures distance to end node.
	heuristic := astar.ConstantHeuristic{}
	for _, node := range astarNodes {
		x, y := nameToCoords(node.ID)
		// The estimat assumes a cost of 1 for the most direct connection.
		estimate := (x - endX) + (y - endY)
		err := heuristic.AddNode(node, estimate)
		if err != nil {
			log.Fatal(err.Error())
		}
	}
	// Plot path.
	path, err := astar.FindPath(astarNodes, startNode, endNode, heuristic.Heuristic(0))
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
