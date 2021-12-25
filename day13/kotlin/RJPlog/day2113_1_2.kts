import java.io.File

//fun main(args: Array<String>) {
	var solution1: Int = 0
	var width: Int = 0
	var height: Int = 0
	var markedPoints = mutableListOf<String>()
	var newMarkedPoints = mutableListOf<String>()
	var foldInstruction = mutableListOf<String>()

	// get width and hight of puzzle input
	File("day2113_puzzle_input.txt").forEachLine {
		var instruction = it.split(",")
		if (it.contains(",")) {
			if (instruction[0].toInt() > width) {
				width = instruction[0].toInt()
			}
			if (instruction[1].toInt() > height) {
				height = instruction[1].toInt()
			}
		}
	}
	width += 1
	height += 1

	// initialize paper
	for (y in 0..height - 1) {
		for (x in 0..width - 1) {
			markedPoints.add(".")
		}
	}

	// print marked points into paper and fill fold instruction list
	File("day2113_puzzle_input.txt").forEachLine {
		if (it.contains(",")) {
			var instruction = it.split(",")
			markedPoints[instruction[0].toInt() + instruction[1].toInt() * width] = "#"
		} else if (it.contains("fold")) {
			var instruction = it.split(" ")
			foldInstruction.add(instruction[2])
		}
	}

	// tag::fold[]
	// go trough each fold instruction
	var firstInstruction : Boolean = true
	foldInstruction.forEach {
		var instruction = it.split("=")
		if (instruction[0] == "y") {
			// do the folding
			for (y in instruction[1].toInt()..height - 1) {
				for (x in 0..width - 1) {
					if (markedPoints[x + y * width] == "#") {
						markedPoints[x + (instruction[1].toInt() - (y - instruction[1].toInt())) * width] =
							markedPoints[x + y * width]
					}
				}
			}
			// create new list
			height = instruction[1].toInt()
			for (y in 0..height - 1) {
				for (x in 0..width - 1) {
					newMarkedPoints.add(markedPoints[x + y * width])
				}
			}
			// exchange lists
			markedPoints.clear()
			markedPoints.addAll(newMarkedPoints)
			newMarkedPoints.clear()

		} else if (instruction[0] == "x") {
			// do the folding
			for (y in 0..height - 1) {
				for (x in instruction[1].toInt()..width - 1) {
					if (markedPoints[x + y * width] == "#") {
						markedPoints[(instruction[1].toInt() - (x - instruction[1].toInt())) + y * width] =
							markedPoints[x + y * width]
					}
				}
			}
			// create new list
			for (y in 0..height - 1) {
				for (x in 0..instruction[1].toInt() - 1) {
					newMarkedPoints.add(markedPoints[x + y * width])
				}
			}
			width = instruction[1].toInt()
			// exchange lists
			markedPoints.clear()
			markedPoints.addAll(newMarkedPoints)
			newMarkedPoints.clear()
		}
		if (firstInstruction) {
				solution1 = markedPoints.count() { it == "#" }
			firstInstruction = false
		}
	}
// end::fold[]

// tag::output[]
// print solution for part 1
	println("***********************************")
	println("--- Day 13: Transparent Origami ---")
	println("***********************************")
	println("Solution for part1")
	println("   $solution1 dots are visible after completing just the first fold instruction on your transparent paper")
	println()
// print solution for part 2
	println("***********************************")
	println("Solution for part2")
	for (y in 0..height - 1) {
		for (x in 0..width - 1) {
			print(markedPoints[x + y * width])
		}
		println()
	}
	println("   do you use to activate the infrared thermal imaging camera system")
	println()
// end::output[]
//}