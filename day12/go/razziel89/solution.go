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

func printNode(node *Node) {
	connectionIDs := []string{}
	for _, con := range node.Connections {
		connectionIDs = append(connectionIDs, con.ID)
	}
	connections := strings.Join(connectionIDs, ",")
	str := fmt.Sprintf("{N: %s, L: %d, C: %s}", node.ID, node.Limit, connections)

	fmt.Println(str)
}

func printNodes(nodes []*Node) {
	for _, node := range nodes {
		printNode(node)
	}
}

func filterAtOrOverLimit(node *Node, curr int) bool {
	if node.Limit == 0 {
		// This is a node we may visit any number of times.
		return false
	}
	return curr >= node.Limit
}

func setFromStack(stack Stack) CountingSet {
	set := CountingSet{}
	for _, node := range stack.ToList() {
		if err := set.Add(node); err != nil {
			// We should never even be able to reach this point. That is, a stack only ever contains
			// non-nil nodes. Thus, fatalling here is fine.
			log.Fatal("internal error")
		}
	}
	return set
}

//nolint:funlen
func findConnections(nodes []*Node, start, end string) (<-chan []*Node, error) {
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
		for len(stack) != 0 {
			fmt.Println(it)
			it++
			printNodes(stack.ToList())
			set := setFromStack(stack)
			overLimit := set.Filter(filterAtOrOverLimit)
			topNode := stack.Peek().Node
			progress := stack.Peek().ConnectionCount
			*progress++
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
					_, _ = stack.Pop()
					// Make sure we never traverse the end node.
					continue
				}
				if nextTop != nil {
					// We know the next top node. Add it.
					stack.Push(nextTop, 0)
				}
			}
			if *progress == len(topNode.Connections) {
				// We have exceeded what we can achieve with this topmost node. Remove the top-most
				// one and continue from the one underneath.
				if _, isEmpty := stack.Pop(); isEmpty {
					// If the stack is empty now, we have reached the end of our search.
					break
				}
			}
		}
		close(channel)
	}()

	return channel, nil
}

//nolint: funlen
func main() {
	nodes, err := ReadLinesAsNodes()
	if err != nil {
		log.Fatal(err.Error())
	}
	printNodes(nodes)
	iterator, err := findConnections(nodes, startNode, endNode)
	if err != nil {
		log.Fatal(err)
	}
	connection := <-iterator
	fmt.Println(connection)
}

// end::solution[]
