package main

import (
	"fmt"
	"strings"
)

// tag::node[]

// Node is a node in a tree.
type Node struct {
	ID          string
	Limit       int
	Value       int
	Connections []*Node
}

// AddConnection adds a connection to a node, but only if that connection does not yet exist. If it
// does, nothing is done.
func (n *Node) AddConnection(connection *Node) {
	for _, con := range n.Connections {
		if con == connection {
			return
		}
	}
	n.Connections = append(n.Connections, connection)
}

// NodeToStr provides a string representation for a node.
func NodeToStr(node *Node) string {
	connectionIDs := []string{}
	for _, con := range node.Connections {
		connectionIDs = append(connectionIDs, con.ID)
	}
	connections := strings.Join(connectionIDs, ",")
	str := fmt.Sprintf("{N: %s, L: %d, C: %d, N: %s}", node.ID, node.Limit, node.Value, connections)
	return str
}

// NodesToStr provides a string representation for multiple nodes.
func NodesToStr(nodes []*Node) string {
	str := ""
	for _, node := range nodes {
		str += NodeToStr(node) + "\n"
	}
	return str
}

// end::node[]
