import java.io.File

// tag::followPath_15[]
fun followPath_15_test(input1: Int): Int {

//	var ruleCheckPassed: Boolean = false
	var width: Int = 0
	var height: Int = 0
	var riskLevel = mutableListOf<Int>()
	var riskLevelSum = mutableListOf<Int>()

	File("day2115_puzzle_input.txt").forEachLine {
		width = it.length
		it.forEach {
			riskLevel.add(it.toString().toInt())
			riskLevelSum.add(it.toString().toInt())
		}
		height += 1
	}
//	println("width $width, height $height")
//	println("risklevel: $riskLevel")
//	println("risklevelSum: $riskLevelSum")


	// create riskLevelSum

		for (y in 0..(height*input1 - 1)) {
			for (x in 0..(width*input1 - 1)) {
				if (!(x == 0 && y == 0)) {
					if (x == 0) {
						riskLevelSum[x + y * width] = riskLevel[x + y * width] + riskLevelSum[(y - 1) * width]
					} else if (y == 0) {
						riskLevelSum[x + y * width] = riskLevel[x + y * width] + riskLevelSum[(x - 1)]
					} else {
						riskLevelSum[x + y * width] = riskLevel[x + y * width] + minOf(
							riskLevelSum[(x - 1) + y * width],
							riskLevelSum[x + (y - 1) * width]
						)
					}
					//	println(riskLevelSum[x+y*width])
				}
			}
		
	}
//	println("riskLevelSum: $riskLevelSum")

	return (riskLevelSum.last() - riskLevelSum.first())
}
// end::followPath_15[]


fun main(args: Array<String>) {
	var t1 = System.currentTimeMillis()


	var solution1 = followPath_15_test(1)
	
	var solution2 = followPath_15_test(1)

	// tag::output[]
// print solution for part 1
	println("***********************")
	println("--- Day 15: Chiton  ---")
	println("***********************")
	println("Solution for part1")
	println("   $solution1 ")
	println()
// print solution for part 2
	println("*******************************")
	println("Solution for part2")
	println("   $solution2 ")
	println()
	t1 = System.currentTimeMillis() - t1
	println("puzzle solved in ${t1} ms")
// end::output[]
}