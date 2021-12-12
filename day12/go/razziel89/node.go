package main

// tag::node[]

// Node is a node in a tree.
type Node struct {
	ID          string
	Limit       int
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

// end::node[]
