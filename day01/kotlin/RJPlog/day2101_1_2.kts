import java.io.File

// tag::sonar_sweep[]
fun sonar_sweep(): Int {
	
	var result: Int = 0
	var samples = mutableListOf<Int>()
	
	File("day2101_puzzle_input.txt").forEachLine {
		samples.add(it.toInt())
	}

	for (i in 1..samples.size-1) {
		if (samples[i] > samples[i-1]){
			result++
		}
	}
	return result
}
// end::sonar_sweep[]

// tag::sonar_sweep_noise[]
fun sonar_sweep_noise(): Int {
	
	var result: Int = 0
	var samples = mutableListOf<Int>()
	
	File("day2101_puzzle_input.txt").forEachLine {
		samples.add(it.toInt())
	}

	for (i in 1..samples.size-1-2) {

		var prev_sum: Int = samples[i-1]+samples[i]+ samples[i+1]
		var cur_sum: Int = samples[i]+samples[i+1]+ samples[i+2]

		if (cur_sum > prev_sum){
			result++
		}
	}
	return result
}
// end::sonar_sweep_noise[]


//fun main(args: Array<String>) {
	var solution1: Int
	var solution2: Int

// tag::part_1[]
	solution1 = sonar_sweep()
// end::part_1[]

// tag::part_2[]
	solution2 = sonar_sweep_noise()
// end::part_2[]

// tag::output[]
// print solution for part 1
	println("**************************")
	println("--- Day 1: Sonar Sweep ---")
	println("**************************")
	println("Solution for part1")
	println("   $solution1 are larger than the previous measurement ")
	println()
// print solution for part 2
	println("**************************")
	println("Solution for part2")
	println("   $solution2 sums are larger than the previous measurement")
	println()
// end::output[]
//}