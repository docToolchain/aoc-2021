// Package main is the main executable for razziel89's solution for this day's advent.
package main

import (
	"fmt"
	"log"
	"strings"
)

// tag::solution[]

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

//nolint: funlen
func main() {
	nodes, err := ReadLinesAsNodes()
	if err != nil {
		log.Fatal(err.Error())
	}
	printNodes(nodes)
}

// end::solution[]
