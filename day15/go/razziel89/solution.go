// Package main is the main executable for razziel89's solution for this day's advent.
package main

import (
	"fmt"
	"log"
	"os"
	"sort"
	"strconv"
	"strings"
)

// tag::solution[]

const (
	// Somehow, this implementation is not all too efficient. Thus, we run it multiple times but
	// start from the lowest cost found so far.
	startingCostEnvVar = "LOWEST_COST"
	// Select the algorithm. You should not use the slow one for anything other than very small
	// examples.
	algorithmSelectionEnvVar = "ALGO"
	replicas                 = 5
)

// func printStack(stack *Stack) {
// 	for _, node := range *stack {
// 		nodeStr := nodeToStr(node.Node)
// 		fmt.Printf("%s -> %d\n", nodeStr, *node.ConnectionCount)
// 	}
// }

func atOrOverLimit(node *Node, curr int) bool {
	if node.Limit == 0 {
		// This is a node we may visit any number of times.
		return false
	}
	return curr >= node.Limit
}

// A validityFn checks whether the currently-attempted solution is stil a valid one.
type validityFn = func(*CountingSet) bool

//nolint:funlen
func findConnections(nodes []*Node, start, end string, checkFn validityFn) (<-chan []*Node, error) {
	channel := make(chan []*Node)
	startNode := FindNode(nodes, start)
	endNode := FindNode(nodes, end)
	if startNode == nil || endNode == nil {
		close(channel)
		return channel, fmt.Errorf("start or end not defined")
	}
	stack := Stack{}
	stack.Push(startNode, 0)
	set := CountingSet{}
	_ = set.Add(startNode)

	go func() {
		for {
			topNode := stack.Peek().Node
			progress := stack.Peek().ConnectionCount

			if !checkFn(&set) {
				// This is no longer a valid solution. Don't continue to explore solutions based on
				// it. Make sure to skip it by setting the progress counter to its maximum.
				*progress = len(topNode.Connections)
			}

			if *progress < len(topNode.Connections) {
				// We have not yet tried out all connections. Try out the next one.
				// Get the next in line and check if we have not yet exceeded its visitation limit.
				var nextTop *Node
				for idx, checkNext := range topNode.Connections[*progress:] {
					if !atOrOverLimit(checkNext, set.Count(checkNext)) {
						*progress += idx
						nextTop = checkNext
						break
					}
				}
				if nextTop == nil {
					// We've not found any available connections. Thus, trigger the removal of the
					// topmost node and continue with the one beneath it. This is done by stating
					// that we've tried out all possible connections from this one.
					*progress = len(topNode.Connections)
				}
				if nextTop == endNode {
					// We have found a connection! Yeah! Emit it.
					stack.Push(nextTop, 0)
					channel <- stack.ToList()
					// fmt.Println("EMIT")
					_, _ = stack.Pop()
					// Make sure we don't check this connection again.
					*progress++
					// Make sure we never traverse the end node.
					continue
				}
				if nextTop != nil {
					// We know the next top node. Add it.
					stack.Push(nextTop, 0)
					_ = set.Add(nextTop)
				}
			}
			if *progress >= len(topNode.Connections) {
				// We have exceeded what we can achieve with this topmost node. Remove the top-most
				// one and continue from the one underneath.
				oldTop, nonEmpty := stack.Pop()
				set.Remove(oldTop.Node)
				if nonEmpty {
					// If the stack is not empty, make sure we check the next connection for the new
					// top node.
					// fmt.Println("Removing", nodeToStr(oldTop.Node))
					// If we ever remove the start node, we have reached the end.
					if oldTop.Node == startNode {
						close(channel)
						return
					}
					newTop := stack.Peek()
					*newTop.ConnectionCount++
				}
			}
		}
	}()

	return channel, nil
}

func setFromList(nodes []*Node) (CountingSet, error) {
	set := CountingSet{}
	for _, node := range nodes {
		if err := set.Add(node); err != nil {
			return CountingSet{}, err
		}
	}
	return set, nil
}

func getCost(set *CountingSet) int {
	sum := 0
	for node := range *set {
		sum += node.Value
	}
	return sum
}

func getCostFilterFn(cost *int) validityFn {
	fn := func(set *CountingSet) bool {
		if *cost < 0 {
			// As long as we haven't yet found a valid connection, don't filter out anything.
			return true
		}
		// Ignore all connections that are at least as costly as the cheapest one found yet.
		return getCost(set) < *cost
	}
	return fn
}

// Convert a grid element to a node. This also makes sure to create a new node and add it to `nodes`
// in case that node does not yet exist. If a node already exists, it is returned.
func gridElemToNode(elem Vec, grid Grid, nodes *[]*Node) *Node {
	nodeName := fmt.Sprintf("%2d-%2d", elem.x, elem.y)
	if foundNode := FindNode(*nodes, nodeName); foundNode != nil {
		// Node already exists in list, return it.
		return foundNode
	}
	// Create a new node and return it.
	newNode := Node{
		ID:          nodeName,
		Value:       grid.Count(elem),
		Connections: []*Node{},
		// For this puzzle, we must never visit a node more than once.
		Limit: 1,
	}
	// Add the node to the list of nodes. That way, we can be sure all converted nodes exist in the
	// list.
	*nodes = append(*nodes, &newNode)
	return &newNode
}

func gridToNodes(grid Grid) []*Node {
	result := []*Node{}
	for p := range grid.Points() {
		node := gridElemToNode(p, grid, &result)
		for envP := range pointEnv(p) {
			if !grid.Has(envP) {
				// Ignore points not on the grid.
				continue
			}
			envNode := gridElemToNode(envP, grid, &result)
			// Add pairwise connections.
			node.AddConnection(envNode)
			envNode.AddConnection(node)
		}
	}
	// Sanity check.
	if len(result) != len(grid) {
		log.Fatal("length disagreement")
	}
	// Sort all nodes starting with low-rist connections.
	overallLessFn := func(idx1, idx2 int) bool {
		return result[idx1].Value < result[idx2].Value
	}
	sort.SliceStable(result, overallLessFn)
	// // Sort each node's connections according to their position.
	// for _, node := range result {
	// 	lessFn := func(idx1, idx2 int) bool {
	// 		return node.Connections[idx1].Value < node.Connections[idx2].Value
	// 	}
	// 	sort.SliceStable(node.Connections, lessFn)
	// 	fmt.Println(NodesToStr(node.Connections))
	// }
	return result
}

//nolint: funlen
func main() {
	grid, err := ReadLinesAsGrid()
	if err != nil {
		log.Fatal(err.Error())
	}
	// Part 1.
	nodes := gridToNodes(grid)
	fmt.Printf("Nodes: %s\n", NodesToStr(nodes))

	startX, startY := grid.TopLeft()
	startNodeName := fmt.Sprintf("%2d-%2d", startX, startY)
	endX, endY := grid.BottomRight()
	endNodeName := fmt.Sprintf("%2d-%2d", endX, endY)

	//nolint:nestif
	if os.Getenv(algorithmSelectionEnvVar) == "slow" {

		startNodeCost := grid[Vec{x: startX, y: startY}]
		endNodeCost := grid[Vec{x: endX, y: endY}]
		fmt.Println("Start node cost: ", startNodeCost, " End node cost: ", endNodeCost)

		cost := -1
		// Allow overwriting the starting cost with an external value.
		if startCostStr := os.Getenv(startingCostEnvVar); startCostStr != "" {
			startCostStr = strings.TrimSpace(startCostStr)
			if startCost, err := strconv.Atoi(startCostStr); err == nil {
				cost = startCost
			}
		}
		fmt.Println("Starting with cost", cost)

		costFilter := getCostFilterFn(&cost)
		iterator, err := findConnections(nodes, startNodeName, endNodeName, costFilter)
		if err != nil {
			log.Fatal(err)
		}

		var bestConnection []*Node
		for con := range iterator {
			set, err := setFromList(con)
			if err != nil {
				log.Fatal("internal error while filtering")
			}
			conCost := getCost(&set) - endNodeCost
			fmt.Println("found cost ", cost)
			if cost < 0 || conCost < cost {
				cost = conCost
				fmt.Println("lowest cost is", cost)
				bestConnection = con
			}
		}
		fmt.Printf("lowest cost is %d\n", cost-startNodeCost+endNodeCost)
		fmt.Println(NodesToStr(bestConnection))
	} else {
		fastAlgorithm(nodes, startNodeName, endNodeName)
	}
	// Part 2.
	// Replicate grid. Then, run the fast algorithm
}

// end::solution[]
