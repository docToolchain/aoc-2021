package main

// tag::stack[]

// Stack is a FIFO.
type Stack []string

// Push puts a value on the stack.
func (s *Stack) Push(val string) {
	*s = append(*s, val)
}

// Pop removes the topmost value and returns it. Also returns whether the stack was non-empty.
func (s *Stack) Pop() (string, bool) {
	if len(*s) == 0 {
		return "", false
	}
	val := (*s)[len(*s)-1]
	*s = (*s)[0 : len(*s)-1 : len(*s)-1]
	return val, true
}

// end::stack[]
