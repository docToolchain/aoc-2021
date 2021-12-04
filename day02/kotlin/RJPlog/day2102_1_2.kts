import java.io.File

// tag::dive_1[]
fun dive_1(): Int {
	
	var depth: Int = 0
	var horizontal: Int = 0
	
	File("day2102_puzzle_input.txt").forEachLine {

		var instruction = it.split(' ')
		
		if (instruction[0].equals("forward")) {
			horizontal = horizontal + instruction[1].toInt()
		} else if (instruction[0].equals("down")) {
			depth = depth + instruction[1].toInt()
		} else if (instruction[0].equals("up")) {
			depth = depth - instruction[1].toInt()
		}   
	}
	return depth*horizontal
}
// end::dive_1[]

// tag::dive_2[]
fun dive_2(): Int {
	
	var depth: Int = 0
	var horizontal: Int = 0
	var aim: Int = 0
	
	File("day2102_puzzle_input.txt").forEachLine {

		var instruction = it.split(' ')
		
		if (instruction[0].equals("forward")) {
			horizontal = horizontal + instruction[1].toInt()
			depth = depth + aim * instruction[1].toInt()
		} else if (instruction[0].equals("down")) {
			aim = aim + instruction[1].toInt()
		} else if (instruction[0].equals("up")) {
			aim = aim - instruction[1].toInt()
		}   
	}
	return depth*horizontal
}
// end::dive_2[]


//fun main(args: Array<String>) {
	var solution1: Int
	var solution2: Int

// tag::part_1[]
	solution1 = dive_1()
// end::part_1[]

// tag::part_2[]
	solution2 = dive_2()
// end::part_2[]

// tag::output[]
// print solution for part 1
	println("*******************")
	println("--- Day 2: Dive ---")
	println("*******************")
	println("Solution for part1")
	println("   $solution1 is your final horizontal position multiplied by final depth ")
	println()
// print solution for part 2
	println("**************************")
	println("Solution for part2")
	println("   $solution2 is your final horizontal position multiplied by final depth")
	println()
// end::output[]
//}