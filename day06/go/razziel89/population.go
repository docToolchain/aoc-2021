package main

import "fmt"

// tag::population[]

// Population tracks the time left until breeding of life forms and counts how many there are per
// such time.
type Population struct {
	pop            []int
	adultDuration  int
	infantDuration int
}

// NewPopulation initialises a new population with times for adults and infants until they breed.
func NewPopulation(adultDuration, infantDuration int) (Population, error) {
	if adultDuration > infantDuration {
		return Population{}, fmt.Errorf("infants must breed slower than adults")
	}
	return Population{
		pop:            make([]int, infantDuration),
		adultDuration:  adultDuration,
		infantDuration: infantDuration,
	}, nil
}

// PopulateFromSet populates a population from a counting set. This ignores entries in the set
// larger than the maximum supported time to breed.
func (p *Population) PopulateFromSet(set CountingSet) {
	for idx := 0; idx < p.infantDuration; idx++ {
		p.pop[idx] = set.Count(fmt.Sprint(idx))
	}
}

// Age ages a population by the given number of days, breeding appropriately.
func (p *Population) Age(days int) {
	for dayCount := 0; dayCount < days; dayCount++ {
		// This new population is auto-initialised with all zeroes.
		newPop := make([]int, len(p.pop))
		// Handle all non-special cases. Simply decrease time to breed by one.
		for breedTime := 0; breedTime < p.infantDuration-1; breedTime++ {
			newPop[breedTime] = p.pop[breedTime+1]
		}
		// Handle special case of those ready to breed.
		newPop[p.infantDuration-1] += p.pop[0]
		newPop[p.adultDuration-1] += p.pop[0]
		// Overwrite the data about the current population.
		_ = copy(p.pop, newPop)
	}
}

// Size determines the total size of the population.
func (p Population) Size() int {
	sum := 0
	for _, val := range p.pop {
		sum += val
	}
	return sum
}

// end::population[]
