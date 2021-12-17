import java.io.File

//fun main(args: Array<String>) {

	var xmin: Int = 0
	var xmax: Int = 0
	var ymin: Int = 0
	var ymax: Int = 0

	File("day2117_puzzle_input.txt").forEachLine {
		var instruction = it.split("=")
		var xArea = instruction[1].dropLast(3).split("..")
		xmin = xArea[0].toInt()
		xmax = xArea[1].toInt()
		var yArea = instruction[2].split("..")
		ymin = yArea[0].toInt()
		ymax = yArea[1].toInt()
	}

	var xpos: Int = 0
	var xvel: Int
	var validXvel = mutableListOf<Int>()

	// tag::findAllXvel[]
	// if x is  > xmax, initial window will be missed already after the first step.
	// find possible x velocities which are able to reach target window
	for (i in 1..xmax) {
		xvel = i
		while (true) {
			xpos = xpos + xvel
			xvel = maxOf(xvel - 1, 0)
			if (xpos >= xmin && xpos <= xmax) {
				validXvel.add(i)
				break
			} else if (xpos > xmax) {
				break
			} else if (xvel == 0) {
				break
			}
		}
		xpos = 0
	}
	// end::findAllXvel[]
	
	// tag::findAllYvel[]
	var ypos: Int = 0
	var yvel: Int
	var validYvel = mutableListOf<Int>()
	var reachedYMax: Int = 0
	var reachedYMaxOverall: Int = 0
	var numberOfInit: Int = 0

	validXvel.forEach {
		xvel = it

		// if starts lower ymin, target will already be missed with first step. If y > abs(ymin), target will also be missed
		for (i in ymin..-ymin) {
			yvel = i
			while (true) {
				xpos = xpos + xvel
				xvel = maxOf(xvel - 1, 0)
				ypos = ypos + yvel
				yvel = yvel - 1
				
				if (ypos > reachedYMax) {
					reachedYMax = ypos
				}

				if (ypos >= ymin && ypos <= ymax && xpos >= xmin && xpos <= xmax) {
					validYvel.add(i)
					if (reachedYMax > reachedYMaxOverall) {
						reachedYMaxOverall = reachedYMax
					}
					break
				} else if (ypos < ymin) {
					break
				} 
			}
			xpos = 0
			xvel = it
			ypos = 0
			reachedYMax = 0
		}
		numberOfInit = numberOfInit + validYvel.size
		validYvel.clear()
	}
	// end::findAllYvel[]

// tag::output[]
// print solution for part 1
	println("*************************")
	println("--- Day 17: Tick Shot ---")
	println("*************************")
	println("Solution for part1")
	println("   $reachedYMaxOverall is the highest y position it reaches on this trajectory")
	println()
// print solution for part 2
	println("*************************")
	println("Solution for part2")
	println("   $numberOfInit many distinct initial velocity values cause the probe to be within the target area after any step")
	println()
// end::output[]
//}