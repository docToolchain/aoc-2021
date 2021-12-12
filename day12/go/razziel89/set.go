package main

import (
	"fmt"
)

// tag::set[]

// CountingSet is a set that also knows how often each element has been added. It does support
// non-empty strings only.
type CountingSet map[*Node]int

// Add adds an entry to the set. The empty *Node is not supported!
func (c *CountingSet) Add(entry *Node) error {
	if entry == nil {
		return fmt.Errorf("empty *Node not supported in counting set")
	}
	// We don't have to handle non-existing values here since Go returns the zero value (0 for
	// integers) for such entries.
	(*c)[entry] = (*c)[entry] + 1
	return nil
}

// Count determines how often an entry has been added to the set.
func (c *CountingSet) Count(entry *Node) int {
	return (*c)[entry]
}

// Remove removes one count for a specific key.
func (c *CountingSet) Remove(entry *Node) {
	if (*c)[entry] == 0 || (*c)[entry] == 1 {
		// Trying to remove an entry we don't have or one that has just one count left results in
		// that entry no longer being present.
		c.RemoveAll(entry)
		return
	}
	(*c)[entry] = (*c)[entry] - 1
}

// RemoveAll removes all counts for a specific key.
func (c *CountingSet) RemoveAll(entry *Node) {
	delete(*c, entry)
}

// FilterFn is a function that can be used to filter entries.
type FilterFn = func(*Node, int) bool

// Filter filters ebtries based on a predicate. That predicate, a FilterFn, gets the entry checked
// and its count in the data set.
func (c *CountingSet) Filter(predicate FilterFn) []*Node {
	result := []*Node{}
	for entry, count := range *c {
		if predicate(entry, count) {
			result = append(result, entry)
		}
	}
	return result
}

// FilterCount counts how many points would be filtered out by a FilterFn.
func (c *CountingSet) FilterCount(predicate FilterFn) int {
	result := 0
	for entry, count := range *c {
		if predicate(entry, count) {
			result++
		}
	}
	return result
}

// end::set[]
