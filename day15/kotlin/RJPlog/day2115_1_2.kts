import java.io.File

// tag::followPath_15[]
fun followPath_15_test(input1: Int): Int {

//	var ruleCheckPassed: Boolean = false
	var width: Int = 0
	var height: Int = 0
	var riskLevel = mutableListOf<Int>()
	var riskLevelSum = mutableListOf<Int>()

	for (j in 0..input1 - 1) {
		File("day2115_puzzle_input.txt").forEachLine {
			width = it.length * input1
			for (i in 0..input1 - 1) {
				it.forEach {
					var risk = it.toString().toInt() + i + j
					if (risk > 9) {
						risk = risk % 9
					}
					riskLevel.add(risk)
					riskLevelSum.add(risk)
				}
			}
			height += 1
		}
	}

	println("width $width, height $height")
/*	println("risklevel: $riskLevel")
	println("risklevelSum: $riskLevelSum")
	for (y in 0..height - 1) {
		for (x in 0..width - 1) {
			print(riskLevel[x + y * width])
		}
		println()
	} */

	println("next")
	// create riskLevelSum

	for (y in 0..height - 1) {
		for (x in 0..width - 1) {
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

	println("riskLevelSum.size ${riskLevelSum.size}")
	println("${riskLevelSum.last()}, ${riskLevelSum.first()}")


	return (riskLevelSum.last() - riskLevelSum.first())
}
// end::followPath_15[]


fun main(args: Array<String>) {
	var t1 = System.currentTimeMillis()


	var solution1 = followPath_15_test(1)
	println()

	var solution2 = followPath_15_test(5)

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
	println("   $solution2 ")   //3001 is to high
	println()
	t1 = System.currentTimeMillis() - t1
	println("puzzle solved in ${t1} ms")
// end::output[]
}