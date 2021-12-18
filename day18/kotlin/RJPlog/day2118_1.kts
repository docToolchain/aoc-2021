import java.io.File

//fun main(args: Array<String>) {
	var solution1: Int = 0
	var solution2: Int = 0

    // tag::snailFishMath[]   
	var snailFishNumber = mutableListOf<Int>()
	var snailFishNumLev = mutableListOf<Int>()
	var levelCount: Int = 0
	var startAddition: Boolean = false

    // tag::algo[]
	File("day2118_puzzle_input.txt").forEachLine {
		it.forEach {
			if (it == '[') {
				levelCount += 1
			} else if (it == ']') {
				levelCount -= 1
			} else if (it.isDigit()) {
				snailFishNumber.add(it.toString().toInt())
				snailFishNumLev.add(levelCount)
			}
		}

		if (startAddition) {
			// values are now added, therefore Level has to  be increased by one
			for (i in 0..snailFishNumLev.size - 1) {
				snailFishNumLev[i] += 1
			}

			//  reduce snail fish number
			var nestInProg: Boolean = true
			var j: Int = 0
			while (nestInProg) {// while ....
				j += 1
				var nested = snailFishNumLev.indexOf(5)
				var greater: Int = -1
				for (i in 0..snailFishNumber.size - 1) {
					if (snailFishNumber[i] > 9) {
						greater = i
						break
					}
				}

				if (nested > -1) {
					if (nested == 0) {
						snailFishNumber[nested + 2] = snailFishNumber[nested + 2] + snailFishNumber[nested + 1]
						snailFishNumber[nested + 1] = 0
						snailFishNumLev[nested + 1] -= 1
						snailFishNumber.removeAt(0)
						snailFishNumLev.removeAt(0)
					} else if (nested == snailFishNumber.size - 2) {
						snailFishNumber[nested - 1] = snailFishNumber[nested - 1] + snailFishNumber[nested]
						snailFishNumber[nested] = 0
						snailFishNumLev[nested] -= 1
						snailFishNumber.removeAt(nested + 1)
						snailFishNumLev.removeAt(nested + 1)
					} else {
						snailFishNumber[nested - 1] = snailFishNumber[nested - 1] + snailFishNumber[nested]
						snailFishNumber[nested + 2] = snailFishNumber[nested + 2] + snailFishNumber[nested + 1]
						snailFishNumber[nested + 1] = 0
						snailFishNumLev[nested + 1] -= 1
						snailFishNumber.removeAt(nested)
						snailFishNumLev.removeAt(nested)
					}

				} else if (greater > -1) {
					var value = snailFishNumber[greater]
					snailFishNumber[greater] = value / 2
					snailFishNumber.add(greater + 1, value / 2 + value % 2)
					snailFishNumLev[greater] = snailFishNumLev[greater] + 1
					snailFishNumLev.add(greater, snailFishNumLev[greater])
				} else if (nested == -1 && greater == -1) {
					nestInProg = false
				}

			}// end while
		}
		startAddition = true
	}

	println(snailFishNumLev)
	println(snailFishNumber)
	println()

	// calculate magnitude
	while(snailFishNumLev.contains(4)) {
		var lowLev = snailFishNumLev.indexOf(4)
		snailFishNumber[lowLev] = snailFishNumber[lowLev]*3 + snailFishNumber[lowLev+1]*2
		snailFishNumLev[lowLev]	= snailFishNumLev[lowLev]- 1
		snailFishNumber.removeAt(lowLev + 1)
		snailFishNumLev.removeAt(lowLev + 1)
	}
	while(snailFishNumLev.contains(3)) {
		var lowLev = snailFishNumLev.indexOf(3)
		snailFishNumber[lowLev] = snailFishNumber[lowLev]*3 + snailFishNumber[lowLev+1]*2
		snailFishNumLev[lowLev]	= snailFishNumLev[lowLev]- 1
		snailFishNumber.removeAt(lowLev + 1)
		snailFishNumLev.removeAt(lowLev + 1)
	}
	while(snailFishNumLev.contains(2)) {
		var lowLev = snailFishNumLev.indexOf(2)
		snailFishNumber[lowLev] = snailFishNumber[lowLev]*3 + snailFishNumber[lowLev+1]*2
		snailFishNumLev[lowLev]	= snailFishNumLev[lowLev]- 1
		snailFishNumber.removeAt(lowLev + 1)
		snailFishNumLev.removeAt(lowLev + 1)
	}	
	while(snailFishNumber.size > 1) {
		snailFishNumber[0] = snailFishNumber[0]*3 + snailFishNumber[1]*2

		snailFishNumber.removeAt(1)		
	}
	
	println("magnitude: $snailFishNumber")
	println()

	
solution1 = snailFishNumber[0]
// end::snailFishMath[]
	
	// tag::output[]
// print solution for part 1
	println("**************************")
	println("--- Day 18: Snailfish  ---")
	println("**************************")
	println("Solution for part1")
	println("   $solution1 ")
	println()
// print solution for part 2
	println("**************************")
	println("Solution for part2")
	println("   $solution2 ")
	println()
// end::output[]
//}