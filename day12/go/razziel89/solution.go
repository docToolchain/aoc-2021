// Package main is the main executable for razziel89's solution for this day's advent.
package main

import (
	"fmt"
	"log"
	"strings"
)

// tag::solution[]

const (
	startNode = "start"
	endNode   = "end"
)

func nodeToStr(node *Node) string {
	connectionIDs := []string{}
	for _, con := range node.Connections {
		connectionIDs = append(connectionIDs, con.ID)
	}
	connections := strings.Join(connectionIDs, ",")
	str := fmt.Sprintf("{N: %s, L: %d, C: %s}", node.ID, node.Limit, connections)
	return str
}

func printNodes(nodes []*Node) {
	for _, node := range nodes {
		fmt.Println(nodeToStr(node))
	}
}

// func printStack(stack *Stack) {
// 	for _, node := range *stack {
// 		nodeStr := nodeToStr(node.Node)
// 		fmt.Printf("%s -> %d\n", nodeStr, *node.ConnectionCount)
// 	}
// }

func filterAtOrOverLimit(node *Node, curr int) bool {
	if node.Limit == 0 {
		// This is a node we may visit any number of times.
		return false
	}
	return curr >= node.Limit
}

func setFromStack(stack Stack) CountingSet {
	set, err := setFromList(stack.ToList())
	if err != nil {
		// We should never even be able to reach this point. That is, a stack only ever contains
		// non-nil nodes. Thus, fatalling here is fine.
		log.Fatal("internal error")
	}
	return set
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

// A validityFn checks whether the currently-attempted solution is stil a valid one.
type validityFn = func(*CountingSet) bool

//nolint:funlen
func findConnections(nodes []*Node, start, end string, checkFn validityFn) (<-chan []*Node, error) {
	channel := make(chan []*Node)
	startNode := FindNode(nodes, startNode)
	endNode := FindNode(nodes, endNode)
	if startNode == nil || endNode == nil {
		close(channel)
		return channel, fmt.Errorf("start or end not defined")
	}
	stack := Stack{}
	stack.Push(startNode, 0)

	go func() {
		it := 0
		for {
			// fmt.Println("It: ", it)
			it++
			// printStack(&stack)

			set := setFromStack(stack)
			overLimit := set.Filter(filterAtOrOverLimit)

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
					if FindNode(overLimit, checkNext.ID) == nil {
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
				}
			}
			if *progress >= len(topNode.Connections) {
				// We have exceeded what we can achieve with this topmost node. Remove the top-most
				// one and continue from the one underneath.
				if oldTop, nonEmpty := stack.Pop(); nonEmpty {
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

// Filter for part 1. Only count connections that don't visit any small cave more than once.
func filterPart1(set *CountingSet) bool {
	threshold := 1
	filterFn := func(node *Node, curr int) bool {
		if strings.ToUpper(node.ID) == node.ID {
			// This is a node we may visit any number of times. Thus, don't filter it out.
			return false
		}
		return curr > threshold
	}
	// If we didn't filter out anything, that means the input describes a connection that does not
	// visit any small cave more than once.
	return set.FilterCount(filterFn) == 0
}

// Filter for part 2. Only count connections that don't visit more than one small cave more than
// once.
func filterPart2(set *CountingSet) bool {
	threshold := 1
	filterFn := func(node *Node, curr int) bool {
		if strings.ToUpper(node.ID) == node.ID {
			// This is a node we may visit any number of times. Thus, don't filter it out.
			return false
		}
		return curr > threshold
	}
	// If we didn't filter out more than one entry, that means the input describes a connection that
	// does not visit more than one small cave more than twice.
	return set.FilterCount(filterFn) <= 1
}

//nolint: funlen
func main() {
	nodes, err := ReadLinesAsNodes()
	if err != nil {
		log.Fatal(err.Error())
	}
	fmt.Println("Nodes:")
	printNodes(nodes)
	iterator, err := findConnections(nodes, startNode, endNode, filterPart2)
	if err != nil {
		log.Fatal(err)
	}
	pathCountPart1 := 0
	pathCountPart2 := 0
	for con := range iterator {
		set, err := setFromList(con)
		if err != nil {
			log.Fatal("internal error while filtering")
		}
		if filterPart1(&set) {
			pathCountPart1++
		}
		// We have already filtered for part 2's validity function.
		pathCountPart2++
	}
	fmt.Printf("there are %d paths for part 1\n", pathCountPart1)
	fmt.Printf("there are %d paths for part 2\n", pathCountPart2)
}

// end::solution[]
