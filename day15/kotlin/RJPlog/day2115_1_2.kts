import java.io.File

// tag::followPath_15[]
fun followPath_15(): Int {
	var segments = mutableListOf<Pair<Int, Int>>()  // changed from String, String to Int, Int
	var currentPath = mutableListOf<Int>()
	var newCurrentPath = mutableListOf<Int>()
	var searchPath = MutableList(0) { mutableListOf<Int>() }
	var searchPathNew = MutableList(0) { mutableListOf<Int>() }
	var validPath = MutableList(0) { mutableListOf<Int>() }
	var searchEnd: Boolean = false
	var totalRiskPerTile = mutableMapOf<Int, Int>()


	var ruleCheckPassed: Boolean = false
	var width: Int = 0
	var height: Int = 0
	var riskLevel = mutableListOf<Int>()

	var firstTotalRisk: Boolean = true
	var totalRiskMin: Int = 0
	var totalRisk: Int = 0
	var tileRisk: Int = 0

	// generate segments Pair(<x,y>,<x,y>) as in day12
	// generate searchpath start value (0,0-0,1 , 0,0-1,0)
	// new: generate list with risk level: ((x,y-x,y), value)
	// rule: single position is only allowed once


	File("day2115_puzzle_input.txt").forEachLine {
		width = it.length
		it.forEach {
			riskLevel.add(it.toString().toInt())
		}
		if (firstTotalRisk) {
			for (i in 1..it.length - 1) {
				totalRiskMin = totalRiskMin + it[i].toString().toInt()
			}
			firstTotalRisk = false
		} else {
			totalRiskMin = totalRiskMin + it.takeLast(1).toString().toInt()
		}
		height += 1
	}

	//totalRiskMin = ((width + height) - 1) * 9

	println("width $width, height $height")
//	println("risklevel: $riskLevel")
	println("totalRiskMin: $totalRiskMin")

	for (y in 0..height - 1) {
		for (x in 0..width - 1) {
			if (x + 1 < width) {
				segments.add(Pair(x + y * width, (x + 1) + y * width))
			}
			if (y + 1 < height) {
				segments.add(Pair(x + y * width, x + (y + 1) * width))
			}
		}
	}
	currentPath.add(0)
	currentPath.add(1)
	searchPath.add(currentPath.toMutableList())
	totalRiskPerTile.put(1, 0)
	currentPath.clear()
	currentPath.add(0)
	currentPath.add(width)
	searchPath.add(currentPath.toMutableList())
	totalRiskPerTile.put(width, 0)

//	println("segments: $segments")
	println("searchPath: $searchPath")
	println("totalRiskPerTile: $totalRiskPerTile")
	println()


	var ii: Int = 0
	while (!searchEnd) {
		ii += 1
		searchEnd = true

		searchPath.forEach {
			currentPath = it
			//println("$ii: currentPath: $currentPath")

			var lastSegment = currentPath.last()

			segments.forEach {
				//	println("${it.first},  ${it.second}")
				if (lastSegment == it.first) {
					newCurrentPath = currentPath.toMutableList()
					newCurrentPath.add(it.second)
					//		println("newCurrentPath $newCurrentPath")
					if (it.second == width * height - 1) {
						//validPath.add(newCurrentPath)
						for (i in 1..newCurrentPath.size - 1) {
							totalRisk = totalRisk + riskLevel[newCurrentPath[i]]
						}
						if (totalRisk < totalRiskMin) {
							totalRiskMin = totalRisk
							validPath.add(newCurrentPath)
						//	println("validPath:  $newCurrentPath found, , totalRisk $totalRisk, totalRiskMin: $totalRiskMin")
						}


						totalRisk = 0

					} else {
						for (i in 1..newCurrentPath.size - 1) {
							totalRisk = totalRisk + riskLevel[newCurrentPath[i]]
						}
						if (totalRiskPerTile.contains(it.second)) {
							ruleCheckPassed =
								!(currentPath.contains(it.second)) && totalRisk < totalRiskPerTile.getValue(it.second)
						} else {
							ruleCheckPassed = !(currentPath.contains(it.second)) && totalRisk < totalRiskMin
						}

						if (ruleCheckPassed) {
							searchPathNew.add(newCurrentPath)
							totalRiskPerTile.put(it.second, totalRisk)
							//	println(" path added at totalRisk $totalRisk, totalRiskMin $totalRiskMin, ${it.second}")
							searchEnd = false
						}
						totalRisk = 0
					}
				} else if (lastSegment == it.second) {
					newCurrentPath = currentPath.toMutableList()
					newCurrentPath.add(it.first)
					//		println("newCurrentPath $newCurrentPath")
					if (it.first == width * height - 1) {
						//validPath.add(newCurrentPath)
						for (i in 1..newCurrentPath.size - 1) {
							totalRisk = totalRisk + riskLevel[newCurrentPath[i]]
						}
						if (totalRisk < totalRiskMin) {
							totalRiskMin = totalRisk
							validPath.add(newCurrentPath)
							//	println("validPath:  $newCurrentPath found, totalRisk $totalRisk, totalRiskMin: $totalRiskMin")
						}


						totalRisk = 0
					} else {
						for (i in 1..newCurrentPath.size - 1) {
							totalRisk = totalRisk + riskLevel[newCurrentPath[i]]
						}
						if (totalRiskPerTile.contains(it.first)) {
							ruleCheckPassed =
								!(currentPath.contains(it.first)) && totalRisk < totalRiskPerTile.getValue(it.first)

						} else {
							ruleCheckPassed = !(currentPath.contains(it.first)) && totalRisk < totalRiskMin

						}
						if (ruleCheckPassed) {
							searchPathNew.add(newCurrentPath)
							totalRiskPerTile.put(it.first, totalRisk)
						//	println(" path $newCurrentPath added at totalRisk $totalRisk, totalRiskMin $totalRiskMin, $it.first}")
							searchEnd = false
						}
						totalRisk = 0
					}
				}
			}
		}
		searchPath.clear()
//		println("serachPath $searchPath")
		searchPath.addAll(searchPathNew)
		println("$ii searchPath.size ${searchPath.size}")
	//	println("totalRiskPerTile $totalRiskPerTile")
		searchPathNew.clear()
//		println("serachPathNew $searchPathNew")
	}

	println()
	println("-- path search ended, now calculation risk level")
	println()

//	return validPath.size

//	println("validPath:")
	println()

/*	validPath.forEach {
		totalRisk = 0
		for (i in 1..it.size - 1) {
			totalRisk = totalRisk + riskLevel[it[i]]
		}
		if (firstTotalRisk) {
			totalRiskMin = totalRisk
			firstTotalRisk = false
		} else if (totalRisk < totalRiskMin) {
			totalRiskMin = totalRisk
		}
		println("$it /totalRisk: $totalRisk")
	}
	println()
	println("totalRiskMin: $totalRiskMin")
*/
	return totalRiskMin
}
// end::followPath_15[]


fun main(args: Array<String>) {
	var t1 = System.currentTimeMillis()
	var solution2: Int = 0

	var solution1 = followPath_15()

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