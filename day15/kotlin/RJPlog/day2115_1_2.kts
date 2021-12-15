import java.io.File


 // tag::followPath_15[]
fun followPath_15(): Int {
	var segments = mutableListOf<Pair<String, String>>()
	var searchPath = mutableListOf<String>()
	var searchPathNew = mutableListOf<String>()
	var validPath = mutableListOf<String>()
	var searchEnd: Boolean = false
	var currentPath: String
	var newCurrentPath: String
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
		it.forEach{
		riskLevel.add(it.toString().toInt())
		}
		height += 1
	}
	 
	 println("width $width, height $height")
	 println("risklevel: $riskLevel")
	 
	for (y in 0..height - 1) {
		for (x in 0..width-1) {
			if (x+1 < width) {
			segments.add(Pair(x.toString()+","+y.toString(),(x+1).toString()+","+y.toString()))	
			}
			if (y+1 <  height) {
			segments.add(Pair(x.toString()+","+y.toString(),x.toString()+","+(y+1).toString()))
			}
		}
	}
	searchPath.add("0,0-0,1")
	searchPath.add("0,0-1,0")  
	 
	 println("segments: $segments")
	 println("searchPath: $searchPath")
	 
	
	 
// var ii : Int = 0
	while (!searchEnd) {
//ii += 1
		searchEnd = true
		
		searchPath.forEach {
			currentPath = it
			//println("$ii: currentPath: $currentPath")
			var instruction = it.split("-")
			var lastSegment = instruction[instruction.size - 1]

			segments.forEach {
				if (lastSegment == it.first) {
					newCurrentPath = currentPath + "-" + it.second
					if (it.second == (width-1).toString()+","+(height-1).toString()) {
						//if (!validPath.contains(newCurrentPath)) {
							validPath.add(newCurrentPath)
						//}
					} else {
	
							ruleCheckPassed = !(currentPath.contains(it.second))  // change!!

						if (ruleCheckPassed) {
							searchPathNew.add(newCurrentPath)
							searchEnd = false
						}
					}
				} else if (lastSegment == it.second) {
					newCurrentPath = currentPath + "-" + it.first
					if (it.first == (width-1).toString()+","+(height-1).toString()) {
						//if (!validPath.contains(newCurrentPath)) {
							validPath.add(newCurrentPath)
						//}
					} else {
						// check rule
							ruleCheckPassed = !(currentPath.contains(it.first)) // change!!
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
	 
	 println("-- path search ended, now calculation risk level")
//	return validPath.size
	 
	 println()
	 var firstTotalRisk: Boolean = true
	 var totalRiskMin: Int = 0
     validPath.forEach{
		 var totalRisk: Int = 0
		 //	 	 println(it)
		 var steps = it.split("-")
		 for(i in 1..steps.size-1) {
			 var positions = steps[i].split(",")
			// 			 println(" ${steps[i]}, ${riskLevel[positions[0].toInt()+positions[1].toInt()*width]}")
			 totalRisk = totalRisk + riskLevel[positions[0].toInt()+positions[1].toInt()*width]
			}
		    if (firstTotalRisk) {
				totalRiskMin = totalRisk
				firstTotalRisk = false
			} else if (totalRisk < totalRiskMin) {
				totalRiskMin = totalRisk
			}
	 	//	println("totalRisk: $totalRisk")
		 //   println("totalRiskMin: $totalRiskMin")
		}

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