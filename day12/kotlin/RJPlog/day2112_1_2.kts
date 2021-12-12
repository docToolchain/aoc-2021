import java.io.File

// tag::ruleCheck2[]
fun ruleCheck2(input1: String): Boolean {
	
	var count = mutableMapOf<String, Int>()
	var instruction = input1.split(",")
	
	instruction.forEach {
		if (it.toLowerCase() == it) {
			if (count.containsKey(it)) {
				count.put(it, count.getValue(it) + 1)
				if (count.getValue(it) > 2) {
					return false
				}
			} else {
				count.put(it, 1)
			}
		}
	}
	var sum: Int = 0
	for ((key, value) in count.entries) {
		sum = sum + value
	}
	if (sum > count.size + 1) {
		return false
	}
	return true
}
// end::ruleCheck2[]

// tag::followPath[]
fun followPath(input1: Int): Int {
	var segments = mutableListOf<Pair<String, String>>()
	var searchPath = mutableListOf<String>()
	var searchPathNew = mutableListOf<String>()
	var validPath = mutableListOf<String>()
	var searchEnd: Boolean = false
	var currentPath: String
	var newCurrentPath: String
	var ruleCheckPassed: Boolean = false

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

	while (!searchEnd) { 

		searchEnd = true
		
		searchPath.forEach {
			currentPath = it
			var instruction = it.split(",")
			var lastSegment = instruction[instruction.size - 1]

			segments.forEach {
				if (lastSegment == it.first) {
					newCurrentPath = currentPath + "," + it.second
					if (it.second == "end") {
						//if (!validPath.contains(newCurrentPath)) {
							validPath.add(newCurrentPath)
						//}
					} else {
						if (input1 == 1) {
							ruleCheckPassed = !(it.second.toLowerCase() == it.second && currentPath.contains(it.second))
						} else if (input1 == 2) {
							ruleCheckPassed = ruleCheck2(newCurrentPath.drop(6))
						}
						if (ruleCheckPassed) {
							searchPathNew.add(newCurrentPath)
							searchEnd = false
						}
					}
				} else if (lastSegment == it.second) {
					newCurrentPath = currentPath + "," + it.first
					if (it.first == "end") {
						//if (!validPath.contains(newCurrentPath)) {
							validPath.add(newCurrentPath)
						//}
					} else {
						// check rule
						if (input1 == 1) {
							ruleCheckPassed = !(it.first.toLowerCase() == it.first && currentPath.contains(it.first))
						} else if (input1 == 2) {
							// rule for part 2
							ruleCheckPassed = ruleCheck2(newCurrentPath.drop(6))
						}
						if (ruleCheckPassed) {
							searchPathNew.add(newCurrentPath)
							searchEnd = false
						}
					}
				}
			}
		}
		searchPath.clear()
		searchPath.addAll(searchPathNew)
		searchPathNew.clear()
	}
	return validPath.size
}
// end::followPath[]

//fun main(args: Array<String>) {

	var solution1: Int
	var solution2: Int

	solution1 = followPath(1)
	solution2 = followPath(2)

// tag::output[]
// print solution for part 1
	println("*******************************")
	println("--- Day 12: Passage Pathing ---")
	println("*******************************")
	println("Solution for part1")
	println("   $solution1 paths through this cave system are there that visit small caves at most once")
	println()
// print solution for part 2
	println("*******************************")
	println("Solution for part2")
	println("   $solution2 how many paths through this cave system are there")
	println()
// end::output[]	
//}