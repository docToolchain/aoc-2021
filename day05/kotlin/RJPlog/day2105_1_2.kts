import java.io.File

// tag::lines[]
fun lines(solution: Int = 0): Int {
	var ground = mutableMapOf<Pair<Int, Int>, Int>()
	var result: Int = 0

	File("day2105_puzzle_input.txt").forEachLine {
		var instruction = it.split(" -> ")
		var pos1 = instruction[0].split(",")
		var pos2 = instruction[1].split(",")
		var x1: Int = pos1[0].toInt()
		var y1: Int = pos1[1].toInt()
		var x2: Int = pos2[0].toInt()
		var y2: Int = pos2[1].toInt()

		if (x1 == x2) {
			for (i in minOf(y1, y2)..maxOf(y1, y2)) {
				if (ground.contains(Pair(x1, i))) {
					ground.put(Pair(x1, i), ground.getValue(Pair(x1, i)) + 1)
				} else {
					ground.put(Pair(x1, i), 1)
				}
			}

		} else if (y1 == y2) {
			for (i in minOf(x1, x2)..maxOf(x1, x2)) {
				if (ground.contains(Pair(i, y1))) {
					ground.put(Pair(i, y1), ground.getValue(Pair(i, y1)) + 1)
				} else {
					ground.put(Pair(i, y1), 1)
				}
			}
		} else {
		// tag::lines2[]	
		if (solution > 0) {
				for (j in 0..(maxOf(x1, x2)-minOf(x1, x2))) {
					if (x2 > x1 && y2 > y1 ) {
						if (ground.contains(Pair(j + minOf(x1, x2), j + minOf(y1, y2)))) {
							ground.put(
								Pair(j + minOf(x1, x2) , j + minOf(y1, y2)),
								ground.getValue(Pair(j + minOf(x1, x2), j + minOf(y1, y2))) + 1
							)
						} else {
							ground.put(Pair(j + minOf(x1, x2), j + minOf(y1, y2)), 1)
						}
					} else if (x2 < x1 && y2 > y1) {
						if (ground.contains(Pair( maxOf(x1, x2) - j, minOf(y1,y2)+j))) {
							ground.put(
								Pair(maxOf(x1, x2) - j, minOf(y1, y2)+j),
								ground.getValue(Pair(maxOf(x1, x2) - j, minOf(y1, y2) + j)) + 1
							)
						} else {
							ground.put(Pair(maxOf(x1, x2) - j, minOf(y1, y2) +j), 1)
						}
					} else if (x2 > x1 && y2 < y1) {
						if (ground.contains(Pair(j + minOf(x1, x2), maxOf(y1, y2) - j))) {
							ground.put(
								Pair(j + minOf(x1, x2) , maxOf(y1, y2) - j),
								ground.getValue(Pair(j + minOf(x1, x2), maxOf(y1, y2)-j)) + 1
							)
						} else {
							ground.put(Pair(j + minOf(x1, x2), maxOf(y1, y2)-j), 1)
						}
					} else if (x2 < x1 && y2 < y1) {
						if (ground.contains(Pair(maxOf(x1, x2) - j, maxOf(y1, y2) - j))) {
							ground.put(
								Pair(maxOf(x1, x2) - j , maxOf(y1, y2) - j),
								ground.getValue(Pair(maxOf(x1, x2) - j, maxOf(y1, y2)-j)) + 1
							)
						} else {
							ground.put(Pair(maxOf(x1, x2) - j, maxOf(y1, y2)-j), 1)
						}
					} 
				}
			}
		}
		// end::lines2[]
	}

	for ((key, value) in ground) {
		if (value > 1) {
			result += 1
		}
	}

	return result
}
// end::lines[]

//fun main(args: Array<String>) {
	var solution1: Int
	var solution2: Int

// tag::part_1[]
	solution1 = lines()
// end::part_1[]

// tag::part_2[]
	solution2 = lines(2)
// end::part_2[]

// tag::output[]
// print solution for part 1
	println("***********************************")
	println("--- Day 5: Hydrothermal Venture ---")
	println("***********************************")
	println("Solution for part1")
	println("   At $solution1 points at least two lines overlap ")
	println()
// print solution for part 2
	println("*********************************")
	println("Solution for part2")
	println("   At $solution2 points at least two lines overlap")  
	println()
// end::output[]	
//}