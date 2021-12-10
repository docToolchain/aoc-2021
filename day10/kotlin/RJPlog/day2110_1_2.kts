import java.io.File

//fun main(args: Array<String>) {

	var solution1: Int
	var solution2: Long

	var points = mutableMapOf<Int, Int>()
	points.put(3, 0)
	points.put(57, 0)
	points.put(1197, 0)
	points.put(25137, 0)

	var correctInstructions = mutableListOf<String>()
	var scoresSolution2 = mutableListOf<Long>()

	// tag::firstpart[]
	File("day2110_puzzle_input.txt").forEachLine {
		var instruction = it
		var searchEnd: Boolean = false
		var pattern0 = arrayOf("()", "[]", "{}", "<>")
		var pattern1 = arrayOf("[)", "{)", "<)")
		var pattern2 = arrayOf("(]", "{]", "<]")
		var pattern3 = arrayOf("(}", "[}", "<}")
		var pattern4 = arrayOf("(>", "[>", "{>")

		// remove iterative all legal pairs of chunks in string 
		while (!searchEnd) {
			if (pattern0.any { instruction.contains(it) }) {
				//if (instruction.contains("()") || instruction.contains("<>") || instruction.contains("[]") || instruction.contains("{}")) {
				instruction = instruction.replace("()", "")
				instruction = instruction.replace("[]", "")
				instruction = instruction.replace("{}", "")
				instruction = instruction.replace("<>", "")
			} else {
				searchEnd = true
			}
		}
		// determine if there is a wrong closing character in remaining string
		if (pattern1.any { instruction.contains(it) }) {
			points.put(3, points.getValue(3) + 1)
		} else if (pattern2.any { instruction.contains(it) }) {
			points.put(57, points.getValue(57) + 1)
		} else if (pattern3.any { instruction.contains(it) }) {
			points.put(1197, points.getValue(1197) + 1)
		} else if (pattern4.any { instruction.contains(it) }) {
			points.put(25137, points.getValue(25137) + 1)
		} else {
			// add all correct lines to a list, revert them already for later calculation of score for part2
			correctInstructions.add(instruction.reversed())
		}
	}

	solution1 = 3*points.getValue(3) + 57*points.getValue(57) + 1197*points.getValue(1197) + 25137*points.getValue(25137)
	// end::firstpart[]

	// tag::secondpart[]
	// evaluate total score for all incomplete but not corrupted lines
	correctInstructions.forEach {
		var totalScore: Long = 0
		it.forEach {
			if (it.equals('(')) {
				totalScore = totalScore * 5 + 1
			} else if (it.equals('[')) {
				totalScore = totalScore * 5 + 2
			} else if (it.equals('{')) {
				totalScore = totalScore * 5 + 3
			} else if (it.equals('<')) {
				totalScore = totalScore * 5 + 4
			}
		}
		scoresSolution2.add(totalScore)
	}

	scoresSolution2.sort()
	solution2 = scoresSolution2[scoresSolution2.size / 2]
	// end::secondpart[]

// tag::output[]
// print solution for part 1
	println("******************************")
	println("--- Day 10: Syntax Scoring ---")
	println("******************************")
	println("Solution for part1")
	println("   $solution1 is the total syntax error score for those errors")
	println()
// print solution for part 2
	println("*********************************")
	println("Solution for part2")
	println("   $solution2 is the middle score")
	println()
// end::output[]		
//}