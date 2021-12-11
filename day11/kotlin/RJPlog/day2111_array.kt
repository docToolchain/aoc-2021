import java.io.File

fun main(args: Array<String>) {
    var t1 = System.currentTimeMillis()
		for (time in 0..1000) {
	// tag::solution[]
	var solution1: Int = 0
	var solution2: Int = 0
	var dumboField = Array(10) { Array(10) { 0 } }
	var countFlash: Int = 0
	var greaterNine: Boolean
	var n: Int = 1
	var stepPartTwo: Int
	var allDumboFlash = false

	// read puzzle input in array
	var j: Int = 0
	File("day2111_puzzle_input.txt").forEachLine {
		for (i in 0..it.length - 1) {
			dumboField[i][j] = it[i].toString().toInt()
		}
		j += 1
	}

	// run for at least 100 steps or until all dumbo's flash at same time
	while (!allDumboFlash || n < 101) {
		greaterNine = false

		// first step: increase every tile by 1
		for (y in 0..9) {
			for (x in 0..9) {
				dumboField[x][y] = dumboField[x][y] + 1
				if (dumboField[x][y] > 9) {
					greaterNine = true
				}
			}
		}

		// second step: if tile > 9, flash and increase energy level of nighbours until no tile > 9 
		while (greaterNine) {  // loop while grid.contains 9
			greaterNine = false

			// go through field and search > nine
			for (y in 0..9) {
				for (x in 0..9) {
					if (dumboField[x][y] > 9 && dumboField[x][y] < 100) {
						// set field to burst (= 100), increase burstcount solution1 +=
						dumboField[x][y] = 100
						// count number of bursts
						countFlash += 1
						// now increase adjacent tiles
						for (dy in -1..1) {
							for (dx in -1..1) {
								if (!(dx == 0 && dy == 0)) {
									// if in grid increase adjacent tiles
									if ((y + dy) >= 0 && (y + dy) <= 9 && (x + dx) >= 0 && (x + dx) <= 9) {
										if (dumboField[x + dx][y + dy] < 100) {
											dumboField[x + dx][y + dy] = dumboField[x + dx][y + dy] + 1
										}
										if (dumboField[x + dx][y + dy] > 9) {
											greaterNine = true
										}
									}
								}
							}
						}
					}
				}
			}
		}    // loop greaterNine end

		// 100 turns --> solution for part1
		if (n == 100) {
			solution1 = countFlash
		}

		// third step: set all tiles flashed in current turn to zero
		stepPartTwo = 0
		for (y in 0..9) {
			for (x in 0..9) {
				if (dumboField[x][y] == 100) {
					dumboField[x][y] = 0
					stepPartTwo += 1
					
					// check if all dumbos flashed at same time --> solution for part2				
					if (stepPartTwo == 100) {
						solution2 = n
						allDumboFlash = true
					}
				}
			}
		}
		n += 1
	}  // end loop allDumboFlash
	// end::solution[]

// tag::output[]
// print solution for part 1
	println("*****************************")
	println("--- Day 11: Dumbo Octopus ---")
	println("*****************************")
	println("Solution for part1")
	println("   $solution1 total flashes are there after 100 steps")
	println()
// print solution for part 2
	println("*********************************")
	println("Solution for part2")
	println("   $solution2 is the first step during which all octopuses flash")
	println()
		}
	t1 = System.currentTimeMillis()-t1
	println("puzzle solved in $t1 ms")
// end::output[]			
}