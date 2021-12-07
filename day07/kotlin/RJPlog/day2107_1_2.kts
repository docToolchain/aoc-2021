import java.io.File
import kotlin.math.*

//fun main(args: Array<String>) {

	var crabPos = mutableListOf<Int>()

	File("day2107_puzzle_input.txt").forEachLine {
		var instruction = it.split(",")
		instruction.forEach {
			crabPos.add(it.toInt())
		}
	}

// tag::part1[]	
	// Part 1
	var minPos: Int = 0
	for (i in 0..crabPos.size - 1) {
		var Pos: Int = 0
		crabPos.forEach {
			Pos = Pos + abs(i - it)
		}
		if (i == 0) {
			minPos = Pos
		} else if (Pos < minPos) {
			minPos = Pos
		}
	}
// end::part1[]	

// tag::part2[]	
	// Part 2
	var minPos2 = 0
	for (i in 0..crabPos.size - 1) {
		var Pos: Int = 0
		crabPos.forEach {
			var dist: Int = abs(i - it)
			var fuel: Int = 0
			for (j in 0..dist) {
				fuel = fuel + j
			}
			Pos = Pos + fuel
		}
		if (i == 0) {
			minPos2 = Pos
		} else if (Pos < minPos2) {
			minPos2 = Pos
		}
	}
// end::part2[]

// tag::output[]
// print solution for part 1
	println("**************************************")
	println("--- Day 7: The Treachery of Whales ---")
	println("**************************************")
	println("Solution for part1")
	println("   $minPos fuel must they spend to align to that position?")
	println()
// print solution for part 2
	println("*********************************")
	println("Solution for part2")
	println("   $minPos2 fuel must they spend to align to that position?")
	println()
// end::output[]	
//}