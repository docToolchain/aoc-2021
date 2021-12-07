package main

import "strconv"

// tag::set[]

// This file contains the counting set as used for day 3 but with ints instead of strings and with
// some unneeded functionality removed.

// CountingSet is a set that also knows how often each element has been added. It does support
// non-empty strings only.
type CountingSet map[int]int

// Add adds an entry to the set.
func (c *CountingSet) Add(entry string) error {
	conv, err := strconv.Atoi(entry)
	if err != nil {
		return err
	}
	// We don't have to handle non-existing values here since Go returns the zero value (0 for
	// integers) for such entries.
	(*c)[conv] = (*c)[conv] + 1
	return nil
}

// Min returns the smallest key.
func (c *CountingSet) Min() int {
	var result int
	found := false
	for key := range *c {
		if !found || key < result {
			result = key
			found = true
		}
	}
	return result
}

// Max returns the largest key.
func (c *CountingSet) Max() int {
	var result int
	found := false
	for key := range *c {
		if !found || key > result {
			result = key
			found = true
		}
	}
	return result
}

// Keys returns all keys of the set.
func (c *CountingSet) Keys() []int {
	result := make([]int, 0, len(*c))
	for key := range *c {
		result = append(result, key)
	}
	return result
}

// Count determines how often an entry has been added to the set.
func (c *CountingSet) Count(entry int) int {
	return (*c)[entry]
}

// RemoveAll removes all counts for a specific key.
func (c *CountingSet) RemoveAll(entry int) {
	delete(*c, entry)
}

// end::set[]
