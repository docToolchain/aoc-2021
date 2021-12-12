import java.io.File

fun followPath(input1: Int): Int {
	var segments = mutableListOf<Pair<String, String>>()
	var searchPath = mutableListOf<String>()
	var searchPathNew = mutableListOf<String>()
	var validPath = mutableListOf<String>()
	var searchEnd: Boolean = false
	var currentPath: String
	var newCurrentPath: String
	var ruleCheckPassed: Boolean = false

	//  read puzzle input as Pairs in segment List
	//  prepare list of searchPath already with path starting with start
	File("day2112_puzzle_input.txt").forEachLine {
		var instruction = it.split("-")
		if (instruction[0] == "start") {
			searchPath.add(instruction[0] + "," + instruction[1])
		} else if (instruction[1] == "start") {
			searchPath.add(instruction[1] + "," + instruction[0])
		} else {
			segments.add(Pair(instruction[0], instruction[1]))
		}
	}

	println("--- Start, reading puzzle input and preparing searchPath")
	println("    segments: $segments")
	println("    searchPath: $searchPath")
	println()

	while (!searchEnd) { //for (k in 0..1) { //
		//println("k: $k")
		searchEnd = true
		searchPath.forEach {
			currentPath = it
			var instruction = it.split(",")
			var lastSegment = instruction[instruction.size - 1]
			println("   --- searching next connections for $currentPath --> $lastSegment")
			println()
			segments.forEach {
				println("      --- checking segment $it")
				println("          ${it.second.toLowerCase() == it.second}")
				println("          ${it.first.toLowerCase() == it.first}")
				println("          ${it.second.toLowerCase() == it.second && !currentPath.contains(it.second)}")
				println("          ${it.first.toLowerCase() == it.first && !currentPath.contains(it.first)}")

				println()
				if (lastSegment == it.first) {
					newCurrentPath = currentPath + "," + it.second
					if (it.second == "end") {
						validPath.add(newCurrentPath)
					} else {
						// check rule
						if (input1 == 1) {
							ruleCheckPassed = !(it.second.toLowerCase() == it.second && currentPath.contains(it.second))
						} else if (input1 == 2) {
							// rule for part 2
						}
						if (ruleCheckPassed) {  // rule part1
							searchPathNew.add(newCurrentPath)
							searchEnd = false
						}
						println("           --- extended Path with $newCurrentPath")
						println()
					}
				} else if (lastSegment == it.second) {
					newCurrentPath = currentPath + "," + it.first
					if (it.first == "end") {
						validPath.add(newCurrentPath)
					} else {
						// check rule
						if (input1 == 1) {
							ruleCheckPassed = !(it.first.toLowerCase() == it.first && currentPath.contains(it.first))
						} else if (input1 == 2) {
							// rule for part 2
						}						
						if (!(it.first.toLowerCase() == it.first && currentPath.contains(it.first))) {  // rule part1
							searchPathNew.add(newCurrentPath)
							searchEnd = false
						}
						println("           --- extended Path with $newCurrentPath")
						println()
					}
				}

			}
		}

		searchPath.clear()
		searchPath.addAll(searchPathNew)
		searchPathNew.clear()

		println("--- searchPath ---")
		println(searchPath)
		println("--- searchPathNew ---")
		println(searchPathNew)
		println()
	}

	println("+++ validPath+++")
	println(validPath)

	return validPath.size
}

//fun main(args: Array<String>) {
	var solution1: Int
	var solution2: Int = 0

	solution1 = followPath(1)


// tag::output[]
// print solution for part 1
	println("*******************************")
	println("--- Day 12: Passage Pathing ---")
	println("*******************************")
	println("Solution for part1")
	println("   $solution1 paths through this cave system are there that visit small caves at most once")
	println()
// print solution for part 2
	println("*********************************")
	println("Solution for part2")
	println("   $solution2 is the first step during which all octopuses flash")
	println()
// end::output[]	
//}