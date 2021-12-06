import java.io.File

// tag::laternfish[]
fun laternfish(day: Int): Int {
	var population = mutableListOf<Int>()

	File("day2106_puzzle_input.txt").forEachLine {
		var instruction = it.split(",")
		instruction.forEach {
			population.add(it.toInt())
		}
	}

	for (i in 1..day) {
		for (x in 0..population.size - 1) {
			if (population[x] == 0) {
				population.add(8)
				population[x] = 6
			} else {
				population[x] -= 1
			}
		}
	}
	return population.size
}
// end::laternfish[]


// tag::laternfish2[]
fun laternfish2(day: Long): Long {
	var result: Long = 0
	var population = mutableMapOf<Int, Long>()
	var new_population = mutableMapOf<Int, Long>()

	for (i in 0..8) {
		population.put(i, 0)
		new_population.put(i, 0)
	}

	File("day2106_puzzle_input.txt").forEachLine {
		var instruction = it.split(",")
		instruction.forEach {
			population.put(it.toInt(), population.getValue(it.toInt()) + 1)
		}
	}

	for (i in 1..day) {
		new_population.put(0, population.getValue(1))
		new_population.put(1, population.getValue(2))
		new_population.put(2, population.getValue(3))
		new_population.put(3, population.getValue(4))
		new_population.put(4, population.getValue(5))
		new_population.put(5, population.getValue(6))
		new_population.put(6, population.getValue(7) + population.getValue(0))
		new_population.put(7, population.getValue(8))
		new_population.put(8, population.getValue(0))

		for (j in 0..8) {
			population.put(j, new_population.getValue(j))
		}
	}

	for ((key, value) in population) {
		result = result + value
	}

	return result
}
// end::laternfish2[]

//fun main(args: Array<String>) {
	var solution1: Int
	var solution2: Long

// tag::part_1[]
	solution1 = laternfish(80)
// end::part_1[]

// tag::part_2[]
	solution2 = laternfish2(256)
// end::part_2[]

// tag::output[]
// print solution for part 1
	println("**************************")
	println("--- Day 6: Lanternfish ---")
	println("**************************")
	println("Solution for part1")
	println("   $solution1 laternfish would be there after 80 days")
	println()
// print solution for part 2
	println("*********************************")
	println("Solution for part2")
	println("   $solution2 laternfish would be there after 80 days")
	println()
// end::output[]	
//}