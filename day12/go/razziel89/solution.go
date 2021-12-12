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
