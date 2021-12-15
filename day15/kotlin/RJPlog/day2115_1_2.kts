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


	var ruleCheckPassed: Boolean = false
	var width: Int = 0
	var height: Int = 0
	var riskLevel = mutableListOf<Int>()

	// generate segments Pair(<x,y>,<x,y>) as in day12
	// generate searchpath start value (0,0-0,1 , 0,0-1,0)
	// new: generate list with risk level: ((x,y-x,y), value)
	// rule: single position is only allowed once

	File("day2115_puzzle_input.txt").forEachLine {
		width = it.length
		it.forEach {
			riskLevel.add(it.toString().toInt())
		}
		height += 1
	}

	println("width $width, height $height")
	println("risklevel: $riskLevel")

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
	currentPath.clear()
	currentPath.add(0)
	currentPath.add(width)
	searchPath.add(currentPath.toMutableList())

	println("segments: $segments")
	println("searchPath: $searchPath")


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
						validPath.add(newCurrentPath)
					} else {
						ruleCheckPassed = !(currentPath.contains(it.second))
						if (ruleCheckPassed) {
							searchPathNew.add(newCurrentPath)
							searchEnd = false
						}
					}
				} else if (lastSegment == it.second) {
					newCurrentPath = currentPath.toMutableList()
					newCurrentPath.add(it.first)
			//		println("newCurrentPath $newCurrentPath")
					if (it.first == width * height - 1) {
						validPath.add(newCurrentPath)
					} else {
						// extend rulecheck by already checking totalRisk.
						// in a 3x3 grid, direct way is max 5*9, you can take also already the input data, every new path with higher totalRisk dont have to be added.
						// or you check, if there is already a path reaching same spot with less totalRisk
						ruleCheckPassed = !(currentPath.contains(it.first))   
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

	println()
	println("-- path search ended, now calculation risk level")
	println()

	validPath.forEach {
		println(it)
	}
//	return validPath.size

	println("validPath:")
	println()
	var firstTotalRisk: Boolean = true
	var totalRiskMin: Int = 0
     validPath.forEach{
		 var totalRisk: Int = 0


		 for(i in 1..it.size-1) {
			 totalRisk = totalRisk + riskLevel[it[i]]
			}
		    if (firstTotalRisk) {
				totalRiskMin = totalRisk
				firstTotalRisk = false
			} else if (totalRisk < totalRiskMin) {
				totalRiskMin = totalRisk
			} 
	 	//println("$it /totalRisk: $totalRisk")

		}
	println()
			    println("totalRiskMin: $totalRiskMin")

	return totalRiskMin
}
// end::followPath_15[]


fun main(args: Array<String>) {

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
// end::output[]
}