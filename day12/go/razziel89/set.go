package main

import (
	"fmt"
)

// tag::set[]

// CountingSet is a set that also knows how often each element has been added. It does support
// non-empty strings only.
type CountingSet map[string]int

// Add adds an entry to the set. The empty string is not supported!
func (c *CountingSet) Add(entry string) error {
	if len(entry) == 0 {
		return fmt.Errorf("empty string not supported in counting set")
	}
	// We don't have to handle non-existing values here since Go returns the zero value (0 for
	// integers) for such entries.
	(*c)[entry] = (*c)[entry] + 1
	return nil
}

// Count determines how often an entry has been added to the set.
func (c *CountingSet) Count(entry string) int {
	return (*c)[entry]
}

// RemoveAll removes all counts for a specific key.
func (c *CountingSet) RemoveAll(entry string) {
	delete(*c, entry)
}

// FilterFn is a function that can be used to filter entries.
type FilterFn = func(string, int) bool

// Filter filters ebtries based on a predicate. That predicate, a FilterFn, gets the entry checked
// and its count in the data set.
func (c *CountingSet) Filter(predicate FilterFn) []string {
	result := []string{}
	for entry, count := range *c {
		if predicate(entry, count) {
			result = append(result, entry)
		}
	}
	return result
}

// end::set[]
