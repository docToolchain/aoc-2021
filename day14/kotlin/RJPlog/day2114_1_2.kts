import java.io.File

// tag::poly[]
fun poly(input1: Int): Int {
	var rules = mutableMapOf<String, String>()
	var polyTemp: String = ""

	File("day2114_puzzle_input.txt").forEachLine {
		if (it != "") {
			if (!it.contains("->")) {
				polyTemp = it
			} else {
				var instruction = it.split(" -> ")
				rules.put(instruction[0], instruction[1])
			}
		}
	}


	//iterate input1 number of steps and create new string with replacement of each pair with given rules
	var newPolyTemp: String = polyTemp.take(1)

	for (i in 1..input1) {
		for (j in 0..polyTemp.length - 2) {
			if (rules.containsKey(polyTemp.subSequence(j, j + 2).toString())) {
				newPolyTemp = newPolyTemp + rules.getValue(polyTemp.subSequence(j, j + 2).toString()) + polyTemp[j + 1]
			} else
				newPolyTemp = newPolyTemp + polyTemp[j + 1]
		}
		polyTemp = newPolyTemp
		newPolyTemp = polyTemp.take(1)
	}

	// count elements and determine max and min value	
	var countMap = mutableMapOf<Char, Int>()
	polyTemp.forEach {
		if (countMap.contains(it)) {
			countMap.set(it, countMap.getValue(it) + 1)
		} else {
			countMap.put(it, 1)
		}
	}

	var max: Int = 0
	var min: Int = 10000
	countMap.forEach {
		if (it.value > max) {
			max = it.value
		} else if (it.value < min) {
			min = it.value
		}
	}
	return max - min
}
// end::poly[]

// tag::polyAdv[]
fun polyAdv(input1: Int): Long {

	var rules = mutableMapOf<String, String>()
	var polyTemp: String = ""

	File("day2114_puzzle_input.txt").forEachLine {
		if (it != "") {
			if (!it.contains("->")) {
				polyTemp = it
			} else {
				var instruction = it.split(" -> ")
				rules.put(instruction[0], instruction[1])
			}
		}
	}

	//setup Map with subSequences and counter for occurency
	var polyPairs = mutableMapOf<String, Long>()
	var newPolyPairs = mutableMapOf<String, Long>()

	polyPairs.put(polyTemp.takeLast(1), 1)
	for (j in 0..polyTemp.length - 2) {
		if (polyPairs.containsKey(polyTemp.subSequence(j, j + 2).toString())) {
			polyPairs.set(
				polyTemp.subSequence(j, j + 2).toString(),
				polyPairs.getValue(polyTemp.subSequence(j, j + 2).toString()) + 1
			)
		} else {
			polyPairs.put(polyTemp.subSequence(j, j + 2).toString(), 1)
		}
	}

	// iterate input1 number of steps and determine, how many new sequences will be created for the current step
	for (i in 1..input1) {
		polyPairs.forEach {
			if (rules.contains(it.key)) {
				
				var new1 = it.key.take(1) + rules.getValue(it.key)
				var new2 = rules.getValue(it.key) + it.key.takeLast(1)

				if (newPolyPairs.containsKey(new1)) {
					newPolyPairs.put(
						new1,
						newPolyPairs.getValue(new1) + it.value
					)
				} else {
					newPolyPairs.put(new1, it.value)
				}
				if (newPolyPairs.containsKey(new2)) {
					newPolyPairs.put(new2, newPolyPairs.getValue(new2) + it.value)
				} else {
					newPolyPairs.put(new2, it.value)
				}
			} else {
				newPolyPairs.put(it.key, it.value)
			}
		}
		polyPairs.clear()
		polyPairs.putAll(newPolyPairs)
		newPolyPairs.clear()
	}

	// count elements and determine max and min value
	var countMap = mutableMapOf<String, Long>()

	for ((key, value) in polyPairs) {
		if (countMap.contains(key.take(1))) {
			countMap.set(key.take(1), countMap.getValue(key.take(1)) + value)
		} else {
			countMap.put(key.take(1), value)
		}
	}

	var max: Long = countMap.getValue(polyTemp.take(1))
	var min: Long = countMap.getValue(polyTemp.take(1))

	countMap.forEach {
		if (it.value > max) {
			max = it.value
		} else if (it.value < min) {
			min = it.value
		}
	}

	return (max - min)
}
// end::polyAdv[]

//fun main(args: Array<String>) {

	var solution1 = poly(10)
	var solution2 = polyAdv(40)

// tag::output[]
// print solution for part 1
	println("****************************************")
	println("--- Day  14: Extended Polymerization ---")
	println("****************************************")
	println("Solution for part1")
	println("   $solution1 you get if you take the quantity of the most common element and subtract the quantity of the least common element")
	println()
// print solution for part 2
	println("*******************************")
	println("Solution for part2")
	println("   $solution2 you get if you take the quantity of the most common element and subtract the quantity of the least common element")
	println()
// end::output[]
//}