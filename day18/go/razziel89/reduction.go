package main

// tag::reduction[]

const (
	explodeLevel   = 4
	splitThreshold = 10
)

// Reduce reduces a snailfish number.
//nolint:funlen
func Reduce(num Number) Number {
	result := num.Copy()
	for reduced := true; reduced; {
		// Assume no reduction happened.
		reduced = false

		// Determine whether the number needs an explosion or a split.

		// Explosion-related.
		// Also remember pointers to the raw digits to be able to easily change the values later.
		rawDigits := []*int{}
		explodeIdx := -1
		var explodeMyChild Number
		var explodeLeftChild bool
		// Split-related.
		var splitMyChild Number
		var splitLeftChild bool
		splitIdx := -1

		// Declare to use in recursion.
		var changeCheck func(Number, Number, int, bool)
		// Close over the above variables so that we can remember which numbers to change for splits
		// or explosions.
		// This function actually determines a lot. It checks whether an explosion or a split is
		// needed.
		changeCheck = func(n Number, parent Number, level int, left bool) {
			//nolint:nestif
			if n.IsPair() {
				// A pair can only explode if it consists of actual numbers and nothing else.
				if !n.Left().IsPair() && !n.Right().IsPair() {
					// Only explode if we are very far nested in and if no previous pair needs to
					// explode.
					if level >= explodeLevel && explodeMyChild == nil {
						explodeMyChild = parent
						explodeLeftChild = left
						// Even though a number with this index hasn't been added yet, we
						// already know it.
						explodeIdx = len(rawDigits)
					}
				}
				changeCheck(n.Left(), n, level+1, true)
				changeCheck(n.Right(), n, level+1, false)
			} else {
				// Only split if we exceed the threshold and if no previous number needs splitting.
				if *n.Val() >= splitThreshold && splitMyChild == nil {
					splitMyChild = parent
					splitLeftChild = left
					// Even though a number with this index hasn't been added yet, we already know
					// it.
					splitIdx = len(rawDigits)
				}
				// Remember a pointer to our actual value. That way, explosions can easily be
				// handled.
				rawDigits = append(rawDigits, n.Val())
			}
		}

		changeCheck(result, nil, 0, true)

		// Process explision and splits. Explosions take precedence.

		if explodeIdx >= 0 {
			reduced = true
			// Explode!
			// Add numbers to neighbours. We remembered pointers to all actual numbers, so this is
			// easy.
			if explodeIdx > 0 {
				// Add the left digit if the pair to its left neighbour, in whichever pair it is.
				// Don't do that if there is no such neighbour.
				*rawDigits[explodeIdx-1] += *rawDigits[explodeIdx]
			}
			if explodeIdx+1 < len(rawDigits)-1 {
				// Add the right digit if the pair to its right neighbour, in whichever pair it is.
				// Don't do that if there is no such neighbour.
				*rawDigits[explodeIdx+2] += *rawDigits[explodeIdx+1]
			}
			// Replace pair by a literal zero. This is where I struggled with pointer receivers in
			// Go. But this works nicely since Go has a garbage collector. Otherwise, manually
			// free'ing memory would be a nightmare with this hacked implementation.
			if explodeLeftChild {
				explodeMyChild.SetLeft(&Digit{0})
			} else {
				explodeMyChild.SetRight(&Digit{0})
			}
			// Start back at the beginning.
			continue
		}

		if splitIdx >= 0 {
			reduced = true
			// Split!
			// Determine the values on the left and the right.
			largeVal := *rawDigits[splitIdx]
			// This is integer division, which is equivalent to rounding down.
			left := largeVal / 2 //nolint:gomnd
			// Take care of "rounding up". This way, we don't have to use floats to round up.
			right := left + largeVal%2 //nolint:gomnd

			newPair := Pair{left: &Digit{left}, right: &Digit{right}}
			// Replace number by a new pair.
			if splitLeftChild {
				splitMyChild.SetLeft(&newPair)
			} else {
				splitMyChild.SetRight(&newPair)
			}
			// Start back at the beginning.
			continue
		}
	}
	return result
}

// end::reduction[]
