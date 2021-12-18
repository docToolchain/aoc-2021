package main

import (
	"fmt"
)

const (
	explodeLevel   = 4
	splitThreshold = 10
)

// Reduce reduces a snailfish number.
//nolint:funlen
func Reduce(num Number) Number {
	for reduced := true; reduced; {

		fmt.Println(num, reduced)
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
		changeCheck = func(n Number, parent Number, level int, left bool) {
			//nolint:nestif
			if n.IsPair() {
				// A pair can only explode if it consists of actual numbers and nothing else.
				if !n.Left().IsPair() && !n.Right().IsPair() {
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
				if *n.Val() >= splitThreshold && splitMyChild == nil {
					splitMyChild = parent
					splitLeftChild = left
					splitIdx = len(rawDigits)
				}
				rawDigits = append(rawDigits, n.Val())
			}
		}
		changeCheck(num, nil, 0, true)

		if explodeIdx >= 0 {
			reduced = true
			// Explode!
			// Add numbers to neighbours.
			if explodeIdx > 0 {
				*rawDigits[explodeIdx-1] += *rawDigits[explodeIdx]
			}
			if explodeIdx+1 < len(rawDigits)-1 {
				*rawDigits[explodeIdx+2] += *rawDigits[explodeIdx+1]
			}
			// Replace pair by a literal zero.
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
			largeVal := *rawDigits[splitIdx]
			left := largeVal / 2 //nolint:gomnd
			right := left
			if largeVal%2 != 0 {
				// Rake care of "rounding up".
				right++
			}

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
	return num
}
