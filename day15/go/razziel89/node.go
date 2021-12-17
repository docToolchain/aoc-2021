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

// ToStr provides a string representation for a node.
func (n *Node) ToStr() string {
	connectionIDs := []string{}
	for _, con := range n.Connections {
		connectionIDs = append(connectionIDs, con.ID)
	}
	connections := strings.Join(connectionIDs, ",")
	str := fmt.Sprintf("{N: %s, L: %d, C: %d, N: %s}", n.ID, n.Limit, n.Value, connections)
	return str
}

// end::node[]
