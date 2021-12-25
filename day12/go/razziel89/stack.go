package main

// tag::stack[]

// StackedNode tracks nodes on the stack as well as how many of its connections have not yet been
// visited.
type StackedNode struct {
	Node            *Node
	ConnectionCount *int
}

// Stack is a FIFO.
type Stack []StackedNode

// Push puts a value on the stack.
func (s *Stack) Push(node *Node, connectionCount int) {
	stackedNode := StackedNode{
		Node:            node,
		ConnectionCount: &connectionCount,
	}
	*s = append(*s, stackedNode)
}

// Pop removes the topmost value and returns it. Also returns whether the stack was non-empty.
func (s *Stack) Pop() (*StackedNode, bool) {
	if len(*s) == 0 {
		return &StackedNode{}, false
	}
	val := &(*s)[len(*s)-1]
	*s = (*s)[0 : len(*s)-1 : len(*s)-1]
	return val, true
}

// Peek returns the top-most entry.
func (s *Stack) Peek() *StackedNode {
	if len(*s) == 0 {
		return &StackedNode{}
	}
	return &(*s)[len(*s)-1]
}

// ToList gets the nodes stacked in the stack in the order they were stacked in (FIFO).
func (s *Stack) ToList() []*Node {
	result := []*Node{}
	for _, stackedNode := range *s {
		result = append(result, stackedNode.Node)
	}
	return result
}

// end::stack[]
