import java.io.File
import kotlin.math.*

// tag::HillClimbing[]
fun HillClimbing(in1: Int): Int {

	var landscape: String = ""
	var xStart: Int = 0
	var yStart: Int = 0

	var width: Int = 0
	var height: Int = 0
	
		var riskLevel = mutableListOf<Int>()
	
	for (j in 0..in1 - 1) {
		File("day2115_puzzle_input.txt").forEachLine {
			width = it.length * in1
			for (i in 0..in1 - 1) {
				it.forEach {
					var risk = it.toString().toInt() + i + j
					if (risk > 9) {
						risk = risk % 9
					}
					riskLevel.add(risk)
					
				}
			}
			height += 1
		}
	}
	
	println("width $width, height $height")
	
	var xEnd: Int = width-1
	var yEnd: Int = height-1
	

	var distList = MutableList(width * height) { width * height * 9}
	distList[xStart * yStart * width] = 0

	// iterate over all points
	var gameEnd: Boolean = false
	var distListSum = distList.sum()

	while (!gameEnd) {
		for (y in 0..height - 1) {
			for (x in 0..width - 1) {
				// check successors of all known nodes
				var dist = distList[x + y * width]
				if (dist != width * height) {
					// calculate all possible directions
					if (x - 1 >= 0) {
							distList[x - 1 + y * width] = min(distList[(x - 1) + y * width], dist + riskLevel[(x-1) + y * width])
					}
					if (x + 1 < width) {
							distList[(x + 1) + y * width] = min(distList[(x + 1) + y * width], dist + riskLevel[(x+1) + y * width])
					}
					if (y - 1 >= 0) {
							distList[x + (y - 1) * width] = min(distList[x + (y - 1) * width], dist + riskLevel[x + (y-1) * width])
					}
					if (y + 1 < height) {
							distList[x + (y + 1) * width] = min(distList[x + (y + 1) * width], dist + riskLevel[x + (y+1) * width])
					}
				}
			}
		}
		if (distListSum == distList.sum()) {
			gameEnd = true
		}
		distListSum = distList.sum()
	}
	return distList[xEnd + yEnd * width]
}
// end::Hillclimbing[]

fun main() {
	var t1 = System.currentTimeMillis()

	var solution1 = HillClimbing(1)
	var solution2 = HillClimbing(5)

// tag::output[]
// print solution for part 1
	println("***************************************")
	println("--- Day 12: Hill Climbing Algorithm ---")
	println("***************************************")
	println("Solution for part1")
	println("   $solution1 is the fewest steps.")
	println()
// print solution for part 2
	println("***************************************")
	println("Solution for part2")
	println("   $solution2 is the fewest steps.")
// end::output[]

	t1 = System.currentTimeMillis() - t1
	println("puzzle solved in ${t1} ms")
}
