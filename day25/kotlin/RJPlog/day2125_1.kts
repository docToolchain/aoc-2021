// tag::part1[]
import java.io.File

//fun main(args: Array<String>) {
	var solution1: Int
	var width: Int = 0
	var depth: Int = 0
	var ground = mutableMapOf<Pair<Int, Int>, String>()

	File("day2125_puzzle_input.txt").forEachLine {

		width = 0
		it.forEach {
			ground.put(Pair(width, depth), it.toString())
			width += 1
		}
		depth += 1
	}

	// initial state
/*	println("Inital state:")
	for (y in 0..depth - 1) {
		for (x in 0..width - 1) {
			print(ground.getValue(Pair(x, y)))
		}
		println()
	} 
	println() */

	// start moves
	var n: Int = 0
	var groundNew = mutableMapOf<Pair<Int, Int>, String>()
	var moveEnd: Boolean = false

	while (!moveEnd) {  // move loop
		n += 1
		moveEnd = true

		// move east
		// prepare groundNew
		for (y in 0..depth - 1) {
			for (x in 0..width - 1) {
				groundNew.put(Pair(x, y), ground.getValue(Pair(x, y)))
			}
		}
		// move where possible
		for (y in 0..depth - 1) {
			for (x in 0..width - 1) {
				if (ground.getValue(Pair(x % width, y)).contains(">")) {
					if (ground.getValue(Pair((x + 1) % width, y)).contains(".")) {
						groundNew.put(Pair((x + 1) % width, y), ">")
						groundNew.put(Pair((x) % width, y), ".")
						moveEnd = false
					}
				}
			}
		}
		ground.clear()
		ground.putAll(groundNew)
		groundNew.clear()

		// move west
		// prepare groundNew
		for (y in 0..depth - 1) {
			for (x in 0..width - 1) {
				groundNew.put(Pair(x, y), ground.getValue(Pair(x, y)))
			}
		}
		// move where possible
		for (y in 0..depth - 1) {
			for (x in 0..width - 1) {
				if (ground.getValue(Pair(x, y % depth)).contains("v")) {
					if (ground.getValue(Pair(x, (y + 1) % depth)).contains(".")) {
						groundNew.put(Pair(x, (y + 1) % depth), "v")
						groundNew.put(Pair(x, y % depth), ".")
						moveEnd = false
					}
				}
			}
		}
		ground.clear()
		ground.putAll(groundNew)
		groundNew.clear()

		// n-th state
/*		println("After $n step:")
		for (y in 0..depth - 1) {
			for (x in 0..width - 1) {
				print(ground.getValue(Pair(x, y)))
			}
			println()
		}  
		println() */

	} // move loop	

	solution1 = n


	// print solution for part 1
	println("****************************")
	println("--- Day 25: Sea Cucumber ---")
	println("****************************")
	println("Solution for part1")
	println("   $solution1 is the first step on which no sea cucumbers move")
//}
// end::part1[]