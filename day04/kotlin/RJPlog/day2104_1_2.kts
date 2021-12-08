import java.io.File

// tag::bingo[]
class bingo(field: List<Int>) {
	// this class should represent a single bingo board. The board is initialized with it's individual numbers
	var field = field
	var fieldProgress = field.toMutableList()
	var win: Boolean = false
	var finalScore: Int = 0

	// this method is used to print the board
	fun print_board() {
		println("-- Board --")
		for (j in 0..4) {
			for (i in 0..4) {
				print("${fieldProgress[i + j * 5]} ")
			}
			println()
		}
		println()
	}

	// this method is used every time a new number is given, to check if bingo is complete and to calculate the final score
	fun nextNumber(Number: Int) {
		var currentNumber: Int = Number
		var sumOfUnmarkedNumbers: Int = 0

		// mark positions which have already fitted
		for (i in 0..fieldProgress.size - 1) {
			if (fieldProgress[i] == currentNumber) {
				fieldProgress[i] = -1
			}
		}

		// check if one row is complete - set win - claculate final score
		if (fieldProgress[0] + fieldProgress[1] + fieldProgress[2] + fieldProgress[3] + fieldProgress[4] == -5 ||
			fieldProgress[5] + fieldProgress[6] + fieldProgress[7] + fieldProgress[8] + fieldProgress[9] == -5 ||
			fieldProgress[10] + fieldProgress[11] + fieldProgress[12] + fieldProgress[13] + fieldProgress[14] == -5 ||
			fieldProgress[15] + fieldProgress[16] + fieldProgress[17] + fieldProgress[18] + fieldProgress[19] == -5 ||
			fieldProgress[20] + fieldProgress[21] + fieldProgress[22] + fieldProgress[23] + fieldProgress[24] == -5 ||
			fieldProgress[0] + fieldProgress[5] + fieldProgress[10] + fieldProgress[15] + fieldProgress[20] == -5 ||
			fieldProgress[1] + fieldProgress[6] + fieldProgress[11] + fieldProgress[16] + fieldProgress[21] == -5 ||
			fieldProgress[2] + fieldProgress[7] + fieldProgress[12] + fieldProgress[17] + fieldProgress[22] == -5 ||
			fieldProgress[3] + fieldProgress[8] + fieldProgress[13] + fieldProgress[18] + fieldProgress[23] == -5 ||
			fieldProgress[4] + fieldProgress[9] + fieldProgress[14] + fieldProgress[19] + fieldProgress[24] == -5
		) {
			win = true
			fieldProgress.forEach {
				if (it != -1) {
					sumOfUnmarkedNumbers = sumOfUnmarkedNumbers + it
				}
			}
			finalScore = currentNumber * sumOfUnmarkedNumbers
		}
	}
}
// end::bingo[]


//fun main(args: Array<String>) {

	// tag::read_puzzle_input[]
	// setup for reading puzzle input
	var numbers = mutableListOf<Int>()
	var card = mutableListOf<Int>()
	var cardboard = listOf<Int>()
	var fields = mutableListOf<bingo>()

	File("day2104_puzzle_input.txt").forEachLine {
		if (it.length > 15) {
			var instruction = it.split(",")
			instruction.forEach { numbers.add(it.toInt()) }
		} else if (it.length > 2) {
			var line: String = ""
			if (it[0].equals(' ')) {
				line = it.drop(1)
			} else {
				line = it
			}
			var instruction = line.replace("  ", " ").replace("  ", " ").split(" ")
			for (i in instruction) {
				card.add(i.toInt())
			}
		} else if (it.length < 2 && card.size > 10) {
			cardboard = card.toList()
			fields.add(bingo(cardboard))
			card.clear()
		}
	}
	// end::read_puzzle_input[]

	// print out puzzle input
	/* println(" Bingo numbers")
	numbers.forEach {
		print("$it,")
	}
	println()
	println()

	println(" Bingo boards")
	fields.forEach {
		it.print_board()
		println()
	}
	println() */

    // tag::play_bingo[]
	// start game
	// loop through numbers until gameend, then printout final score
	var currentNumber: Int
	var outOfGame = mutableListOf<Int>()
	var winningScores = mutableListOf<Int>()

	loop@ for (i in 0..numbers.size - 1) {
		currentNumber = numbers[i]

		var xx: Int = 0
		for (j in fields) {//fields.forEach {
			if (!outOfGame.contains(xx)) {
				j.nextNumber(currentNumber)
				if (j.win) {
					outOfGame.add(xx)
					winningScores.add(j.finalScore)
					if (outOfGame.size == fields.size) {
						break@loop
					}
				}
			}
			xx += 1
		}
	}
    // end::play_bingo[]
	
// tag::output[]
// print solution for part 1
	println("**************************")
	println("--- Day 4: Giant Squid ---")
	println("**************************")
	println("Solution for part1")
	println("   ${winningScores[0]} will your final score be if you choose that board")
	println()
// print solution for part 2
	println("*********************************")
	println("Solution for part2")
	println("   ${winningScores[winningScores.size-1]} would be the final score")
	println()
// end::output[]
//}