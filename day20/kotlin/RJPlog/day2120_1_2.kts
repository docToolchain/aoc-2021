import java.io.File

//fun main(args: Array<String>) {

	var solution1: Int = 0
	var solution2: Int = 0

	//tag::readInput[]
	var width: Int = 0
	var height: Int = 0

	var enhAlg: String = ""

	var inputImage = mutableMapOf<Pair<Int, Int>, Int>()


	File("day2120_puzzle_input.txt").forEachLine {
		if (it.length == 512) {
			enhAlg = it
		} else if (it.length != 0) {
			width = it.length
			for (x in 0..it.length - 1) {
				var light: Int
				if (it[x].toString() == ".") {
					light = 0
				} else {
					light = 1
				}
				inputImage.put(Pair(x, height), light)
			}
			height = height + 1
		}
	}
	//end::readInput[]

	//tag::applyEnhAlg[]
	var outputImage = mutableMapOf<Pair<Int, Int>, Int>()

	for (n in 1..50) {
		// iterate over input
		var numLights = 0
		for (y in 0 - 5 - n..height - 1 + n + 5) {
			for (x in 0 - 5 - n..width - 1 + n + 5) {

				var xpos = x
				var ypos = y
				var index: String = ""

				// find index for each tile
				for (yy in ypos - 1..ypos + 1) {
					for (xx in xpos - 1..xpos + 1) {
						if (inputImage.containsKey(Pair(xx, yy))) {
							index = index + inputImage.getValue(Pair(xx, yy)).toString()
						} else {
							if (n % 2 == 0) {
								index = index + "1"
							} else {
								index = index + "0"
							}
						}
					}
				}

				// get value out of enhancedment algorithm and put Value in outputImage
				if (enhAlg[index.toInt(2)] == '#') {
					outputImage.put(Pair(x, y), 1)
					numLights = numLights + 1
					if (n == 2) {
						solution1 = numLights
					} else if (n == 50) {
						solution2 = numLights
					}
				} else {
					outputImage.put(Pair(x, y), 0)
				}
			}
		}

		inputImage.clear()
		inputImage.putAll(outputImage)
		outputImage.clear()
	}
	//end::applyEnhAlg[]

	// tag::output[]
	// print solution for part 1
	println("**************************")
	println("--- Day 20: Trench Map ---")
	println("**************************")
	println("Solution for part1")
	println("   $solution1 pixels are lit in the resulting image")
	println()
	// print solution for part 2
	println("**************************")
	println("Solution for part2")
	println("   $solution2 pixels are lit in the resulting image")
	println()
// end::output[]
//}