package main

import (
	"bufio"
	"io"
	"os"
	"strings"
)

const (
	connectionSep = "-"
)

var reader = bufio.NewReader(os.Stdin)

// Function readLine reads one line from stdin via a global reader instance.
func readLine() (string, error) {
	return reader.ReadString('\n')
}

func trimStrings(sli []string) []string {
	result := make([]string, 0, len(sli))
	for _, val := range sli {
		result = append(result, strings.TrimSpace(val))
	}
	return result
}

// tag::utils[]

// FindNode finds a node based on its ID in a list of them.
func FindNode(nodes []*Node, id string) *Node {
	for _, checkNode := range nodes {
		if checkNode.ID == id {
			return checkNode
		}
	}
	return nil
}

// ReadLinesAsNodes reads all lines from stdin as nodes. Note that each line refers to more than one
// node.
func ReadLinesAsNodes() ([]*Node, error) {
	var result []*Node
	for {
		line, err := readLine()
		if err == io.EOF {
			// Success case, no more input to read.
			return result, nil
		}
		if err != nil {
			return []*Node{}, err
		}
		fields := trimStrings(strings.Split(line, connectionSep))
		// Try to find an existing node of the names. If none is found, create one.
		for _, field := range fields {
			if node := FindNode(result, field); node == nil {
				// No node found, add a new one.
				// A limit of zero means no limit in visiting.
				limit := 0
				if strings.ToLower(field) == field {
					// Lower case node name, we cannot visit it more than once.
					limit = 1
				}
				newNode := &Node{
					ID:          field,
					Limit:       limit,
					Connections: []*Node{},
				}
				result = append(result, newNode)
			}
		}
		// For each node in this line, add links to all other nodes mentioned in it. Note that
		// FindNode will not be able to return nil here as we already added all nodes mentioned in
		// this line.
		for _, startField := range fields {
			startNode := FindNode(result, startField)
			for _, endField := range fields {
				if endField == startField {
					continue
				}
				endNode := FindNode(result, endField)
				// Luckily, AddConnection will not do anything if the node is already known.
				startNode.AddConnection(endNode)
				endNode.AddConnection(startNode)
			}
		}
	}
}

// end::utils[]
