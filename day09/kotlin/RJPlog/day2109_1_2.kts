import java.io.File

// tag::smokeBasin[]
fun smokeBasin(input1: Int, input2: Int, input3: List<Int>, input4: Int, input5: Int): Int {
	var xpos = input1
	var ypos = input2
	var map = mutableListOf<Char>()
	var width = input4
	var height = input5
	var result: Int = 1

	input3.forEach {
		map.add((it + 48).toChar())
	}
	map.set(xpos + ypos * width, 'x')

	var searchEnd: Boolean = false
	while (!searchEnd) {
		searchEnd = true
		for (y in 1..height - 2) {
			for (x in 1..width - 2) {
				if (map[x + y * width].isDigit()) { // check if digit (no evaluation for already placed 'x')
					if (map[x + y * width].toString().toInt() < 9 && map[(x - 1) + (y) * width] == 'x') {
						result = result + 1
						map.set(x + y * width, 'x')
						searchEnd = false
					} else if (map[x + y * width].toString().toInt() < 9 && map[(x + 1) + (y) * width] == 'x') {
						result = result + 1
						map.set(x + y * width, 'x')
						searchEnd = false
					} else if (map[x + y * width].toString().toInt() < 9 && map[(x) + (y - 1) * width] == 'x') {
						result = result + 1
						map.set(x + y * width, 'x')
						searchEnd = false
					} else if (map[x + y * width].toString().toInt() < 9 && map[(x) + (y + 1) * width] == 'x') {
						result = result + 1
						map.set(x + y * width, 'x')
						searchEnd = false
					} 

				} // if digit			
			} //end for x
		} // end for y
	} // searchEnd
	return result
}
// end::smokeBasin[]

//fun main(args: Array<String>) {

	// tag::solution[]
	var solution1: Int = 0
	var solution2: Int
	var solution2_results = mutableListOf<Int>()
	var heightmap = mutableListOf<Int>()
	var width: Int = 0
	var height: Int = 0

	// read puzzle input to evaluate width and height of grid
	File("day2109_puzzle_input.txt").forEachLine {
		width = it.length
		height += 1
	}

	// add a frame to grid, makes following evaluations easier, you don't have to check if value is out of grid at borders
	width = width + 2
	height = height + 2
	for (i in 0..width - 1) {
		heightmap.add(9)
	}
    
	// read puzzle input into list
	File("day2109_puzzle_input.txt").forEachLine {
		heightmap.add(9)
		it.forEach {
			heightmap.add(it.toString().toInt())
		}
		heightmap.add(9)
	}
	for (i in 0..width - 1) {
		heightmap.add(9)
	}

	// check for sinks and add up risk level, for each sink start fun smokeBasin to get size of basin
	for (y in 1..height - 2) {
		for (x in 1..width - 2) {
			if (heightmap[x + y * width] < heightmap[(x - 1) + (y) * width] && heightmap[x + y * width] < heightmap[(x + 1) + (y) * width] && heightmap[x + y * width] < heightmap[(x) + (y + 1) * width] && heightmap[x + y * width] < heightmap[(x) + (y - 1) * width]) {
				solution1 = solution1 + heightmap[x + y * width] + 1
				solution2_results.add(smokeBasin(x, y, heightmap, width, height))
			} 
		}
	}

	// sort list to find highest 3
	solution2_results.sort()			
	solution2 = solution2_results[solution2_results.size-1] * solution2_results[solution2_results.size-2] * solution2_results[solution2_results.size-3]
	// end::solution[]

	// tag::output[]
// print solution for part 1
	println("**************************")
	println("--- Day 9: Smoke Basin ---")
	println("**************************")
	println("Solution for part1")
	println("   $solution1 is the sum of the risk levels of all low points on your heightmap")
	println()
// print solution for part 2
	println("*********************************")
	println("Solution for part2")
	println("   You get $solution2 if you add up all of the output values")
	println()
// end::output[]		
//}