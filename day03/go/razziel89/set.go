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

// MostCommon determines the most common entry in the set. If the set is empty, this returns the
// empty string! A non-empty tie breaker will be returned in case there are multiple most common
// entries. If the tie breaker is empty but there are duplicates, an empty string will be returned.
func (c *CountingSet) MostCommon(tieBreaker string) string {
	return c.filter(
		tieBreaker,
		func(i1, i2 int) bool {
			return i1 > i2
		},
	)
}

// LeastCommon determines the least common entry in the set. If the set is empty, this returns the
// empty string! A non-empty tie breaker will be returned in case there are multiple least common
// entries. If the tie breaker is empty but there are duplicates, an empty string will be returned.
func (c *CountingSet) LeastCommon(tieBreaker string) string {
	return c.filter(
		tieBreaker,
		func(i1, i2 int) bool {
			return i1 < i2
		},
	)
}

type comparisonFunc = func(int, int) bool

func (c *CountingSet) filter(tieBreaker string, cmpFn comparisonFunc) string {
	var result string
	var resultCount int
	foundOne := false
	for entry, count := range *c {
		if !foundOne || cmpFn(count, resultCount) {
			foundOne = true
			result = entry
			resultCount = count
		}
	}
	// Check whether there are any duplicate findings and handle appropriately.
	for entry, count := range *c {
		if count == resultCount && entry != result {
			return tieBreaker
		}
	}
	return result
}

// end::set[]
