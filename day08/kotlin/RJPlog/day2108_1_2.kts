import java.io.File

//fun main(args: Array<String>) {

	// tag::firstpart[]
	// part 1
	var solution1: Int = 0

	File("day2108_puzzle_input.txt").forEachLine {
		var instruction = it.split(" | ")
		var output = instruction[1].split(" ")
		output.forEach {
			if (it.length == 2 || it.length == 4 || it.length == 3 || it.length == 7) {
				solution1 += 1
			}
		}
	}
	// end::firstpart[]
	
	// tag::secondpart[]
	// part 2
	var solution2: Int = 0
	var mapTable = mutableListOf<String>()
	for (i in 0..9) {
		mapTable.add("")
	}
	File("day2108_puzzle_input.txt").forEachLine {
		var instruction = it.split(" | ")
		var inp = instruction[0].split(" ")
		
		// sort all input values alphabetically
		var input = mutableListOf<String>()
		inp.forEach {
			input.add(it.toCharArray().sorted().joinToString(""))
		}
		
		// assigning 1, 4, 7, 8
		input.forEach {
			if (it.length == 2) {
				mapTable[1] = it
			} else if (it.length == 4) {
				mapTable[4] = it
			} else if (it.length == 3) {
				mapTable[7] = it
			} else if (it.length == 7) {
				mapTable[8] = it
			}
		}
		// assigning 5
		input.forEach {
			var help: String = ""
			mapTable[4].forEach {
				if (!mapTable[1].contains(it)) {
					help = help + it
				}
			}
			if (it.length == 5) {
				if (it.toList().containsAll(mapTable[1].toList())) {     // if it contains 1 --> 3
					mapTable.set(3, it)
				} else if (it.toList().containsAll(help.toList())) {     // if it contains 4 -->5
					mapTable.set(5, it)
				} else {                                                  // else --> 2
					mapTable.set(2, it)
				}
			}
		}
		// assigning 6
		input.forEach {
			if (it.length == 6) {
				if (it.toList().containsAll(mapTable[3].toList())) {   		// if it contains 3 --> 9  
					mapTable.set(9, it)
				} else if (it.toList().containsAll(mapTable[5].toList())) {  // if it contains 5 --> 9  
					mapTable.set(6, it) // if it containns 4 --> 9
				} else {
					mapTable.set(0, it)
				}
			}
		}

		var out = instruction[1].split(" ")
		// sort all input values alphabetically
		var output = mutableListOf<String>()
		out.forEach {
			output.add(it.toCharArray().sorted().joinToString(""))
		}
		solution2 =
			solution2 + mapTable.indexOf(output[0]) * 1000 + mapTable.indexOf(output[1]) * 100 + mapTable.indexOf(output[2]) * 10 + mapTable.indexOf(output[3])
	}
	// end::secondpart[]

	// tag::output[]
// print solution for part 1
	println("***********************************")
	println("--- Day 8: Seven Segment Search ---")
	println("***********************************")
	println("Solution for part1")
	println("   $solution1 times do digits 1, 4, 7, or 8 appear")
	println()
// print solution for part 2
	println("*********************************")
	println("Solution for part2")
	println("   You get $solution2 if you add up all of the output values")
	println()
// end::output[]
//}