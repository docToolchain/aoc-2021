import java.io.File

// tag::diagnostic_power[]
fun diagnostic_power(): Int {
	var count = mutableListOf(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
	var reports: Int = 0
	
	// search for most common bit
	File("day2103_puzzle_input.txt").forEachLine {
		reports += 1
		for (i in 0..it.length - 1) {
			if (it[i].equals('1')) {
				count[i] = count[i] + 1
			}
		}
	}
	
	// calculate gamma and epsilon rate out of most common bit list
	var gamma: String = ""
	var epsilon: String = ""
	count.forEach {
		if (it > reports - it) {
			gamma = gamma + "1"
			epsilon = epsilon + "0"
		} else {
			gamma = gamma + "0"
			epsilon = epsilon + "1"
		}
	}
	
	// return power consumption
	return gamma.toInt(2) * epsilon.toInt(2)
}
// end::diagnostic_power[]

// tag::diagnostic_life[]
fun diagnostic_life(): Int {
	val list = mutableListOf<String>()
	val list_co2 = mutableListOf<String>()
	var pos: Int = 0

	// setup lists for O2 and CO2 ratings
	File("day2103_puzzle_input.txt").forEachLine {
		list.add(it)
		list_co2.add(it)
	}

	// determine O2 generator rating
	while (list.size > 1) {
		var (first, second) = list.partition { it[pos].equals('1') }
		if (first.size >= second.size) {
			list.clear()
			first.forEach {
				list.add(it)
			}
		} else {
			list.clear()
			second.forEach {
				list.add(it)
			}
		}
		pos += 1
	}
	
	// determine CO2 scrubber rating
	pos = 0

	while (list_co2.size > 1) {
		var (first, second) = list_co2.partition { it[pos].equals('1') }
		if (first.size >= second.size) {
			list_co2.clear()
			second.forEach {
				list_co2.add(it)
			}
		} else {
			list_co2.clear()
			first.forEach {
				list_co2.add(it)
			}
		}
		pos += 1
	}

	// return life supporting rate
	return list[0].toInt(2) * list_co2[0].toInt(2)
}
// end::diagnostic_life[]


//fun main(args: Array<String>) {
	var solution1: Int
	var solution2: Int

// tag::part_1[]
	solution1 = diagnostic_power()
// end::part_1[]

// tag::part_2[]
	solution2 = diagnostic_life()
// end::part_2[]

// tag::output[]
// print solution for part 1
	println("********************************")
	println("--- Day 3: Binary Diagnostic ---")
	println("********************************")
	println("Solution for part1")
	println("   $solution1 is your power consumption ")
	println()
// print solution for part 2
	println("*********************************")
	println("Solution for part2")
	println("   $solution2 is your life supporting rate")
	println()
// end::output[]
//}