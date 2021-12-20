import java.io.File

class scanX(field: List<Triple<Int, Int, Int>>) {
	// this class should represent a single scan with data of puzzle input

	var beacons = field
	var beaconsFlipRot = mutableListOf<Triple<Int, Int, Int>>()

	// this method is used to flip/rotate the scan
	// I don't understand why its 24 instead of 48
	fun flipRotate(in1: Int) {
		when (in1) {
			(0) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(it.first, it.second, it.third))
				}
			}
			(1) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(it.first, it.second, -it.third))
				}
			}
			(2) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(it.first, -it.second, it.third))
				}
			}
			(3) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(it.first, -it.second, -it.third))
				}
			}
			(4) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(it.first, it.third, it.second))
				}
			}
			(5) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(it.first, it.third, -it.second))
				}
			}
			(6) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(it.first, -it.third, it.second))
				}
			}
			(7) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(it.first, -it.third, -it.second))
				}
			}
			(8) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(-it.first, it.second, it.third))
				}
			}
			(9) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(-it.first, it.second, -it.third))
				}
			}
			(10) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(-it.first, -it.second, it.third))
				}
			}
			(11) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(-it.first, -it.second, -it.third))
				}
			}
			(12) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(-it.first, it.third, it.second))
				}
			}
			(13) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(-it.first, it.third, -it.second))
				}
			}
			(14) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(-it.first, -it.third, it.second))
				}
			}
			(15) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(-it.first, -it.third, -it.second))
				}
			}
			(16) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(it.second, it.first, it.third))
				}
			}
			(17) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(it.second, it.first, -it.third))
				}
			}
			(18) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(it.second, -it.first, it.third))
				}
			}
			(19) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(it.second, -it.first, -it.third))
				}
			}
			(20) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(it.second, it.third, it.first))
				}
			}
			(21) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(it.second, it.third, -it.first))
				}
			}
			(22) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(it.second, -it.third, it.first))
				}
			}
			(23) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(it.second, -it.third, -it.first))
				}
			}
			(24) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(-it.second, it.first, it.third))
				}
			}
			(25) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(-it.second, it.first, -it.third))
				}
			}
			(26) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(-it.second, -it.first, it.third))
				}
			}
			(27) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(-it.second, -it.first, -it.third))
				}
			}
			(28) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(-it.second, it.third, it.first))
				}
			}
			(29) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(-it.second, it.third, -it.first))
				}
			}
			(30) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(-it.second, -it.third, it.first))
				}
			}
			(31) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(-it.second, -it.third, -it.first))
				}
			}
			(32) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(it.third, it.first, it.second))
				}
			}
			(33) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(it.third, it.first, -it.second))
				}
			}
			(34) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(it.third, -it.first, it.second))
				}
			}
			(35) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(it.third, -it.first, -it.second))
				}
			}
			(36) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(it.third, it.second, it.first))
				}
			}
			(37) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(it.third, it.second, -it.first))
				}
			}
			(38) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(it.third, -it.second, it.first))
				}
			}
			(39) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(it.third, -it.second, -it.first))
				}
			}
			(40) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(-it.third, it.first, it.second))
				}
			}
			(41) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(-it.third, it.first, -it.second))
				}
			}
			(42) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(-it.third, -it.first, it.second))
				}
			}
			(43) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(-it.third, -it.first, -it.second))
				}
			}
			(44) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(-it.third, it.second, it.first))
				}
			}
			(45) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(-it.third, it.second, -it.first))
				}
			}
			(46) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(-it.third, -it.second, it.first))
				}
			}
			(47) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(-it.third, -it.second, -it.first))
				}
			}
		}
	}

	// this method is to adapt the offset of the scan
	fun shiftByOffset(in1: Int, in2: Int, in3: Int) {
		// println("$in1, $in2, $in3")
		var newList = mutableListOf<Triple<Int, Int, Int>>()
		beaconsFlipRot.forEach {

			newList.add(Triple(it.first + in1, it.second + in2, it.third + in3))
		}
		beaconsFlipRot.clear()
		beaconsFlipRot.addAll(newList)
		newList.clear()
	}
}

fun main(args: Array<String>) {
	    var t1 = System.currentTimeMillis()
	var solution1: Int = 0
	var solution2: Int = 0

	// tag::read_puzzle_input[]
	// setup for reading puzzle input
	var scanInput = mutableListOf<Triple<Int, Int, Int>>()
	var allScans = mutableListOf<scanX>()

	// read all scans of puzzle input into scanX classes and add them to a list of unmathced scans (allScans)
	File("day2119_puzzle_input.txt").forEachLine {
		if (it.contains("--")) {
			scanInput.clear()
		} else if (it.length > 1) {
			var position = it.split(",")
			scanInput.add(
				Triple(
					position[0].toString().toInt(),
					position[1].toString().toInt(),
					position[2].toString().toInt()
				)
			)
		} else {
			//println(scanInput.sortedWith(compareBy({ it.first }, { it.second }, { it.third }))):  not necessary, does not bring any benefit
			allScans.add(scanX(scanInput.toList()))
		}
	}
	// end::read_puzzle_input[]

	println(allScans.size)

	// initialize list with matched scans (matchedScans) with first element of allScans list
	var matchedScans = mutableListOf<scanX>()
	matchedScans.add(allScans[0])
	matchedScans[0].flipRotate(0)
	allScans.removeAt(0)

	println("allScans:")
	allScans.forEach {
		println(it.beacons)
	}
	println()
	println("matchedScans:")
	matchedScans.forEach {
		println(it.beaconsFlipRot)
	}

	// while (allSize.isNotEmpty()) {


	var m: Int = 0
	var mm: Int = 0


	println()
	println("---------------------------")
	println("-- Start searching match --")
		println("---------------------------")
	println()

	var count: Int = 0
	// while (count < 1) {//while (allScans.isNotEmpty()) {
	count += 1

	var matchList = mutableListOf<Int>()


	//for (mm in 0..3) {
	while (allScans.size > 0) {

		
		for (m in 0..allScans.size - 1) {

			println("m: =$m, allScans.size ${allScans.size}")

			Loop@ for (ii in 0..47) {
				allScans[m].flipRotate(ii)

				//println("ii: $ii")
				for (j in 0..matchedScans[mm].beaconsFlipRot.size - 1) {
					var beacon1 = matchedScans[mm].beaconsFlipRot[j]
					//		println("j: $j   beacon1 $beacon1")
					for (i in 0..allScans[m].beaconsFlipRot.size - 1) {

						var beacon2 = allScans[m].beaconsFlipRot[i]
						//println("j: $j i: $i  ") //   beacon2 $beacon2")

						var x_off = beacon1.first - beacon2.first
						var y_off = beacon1.second - beacon2.second
						var z_off = beacon1.third - beacon2.third
						// println("$x_off, $y_off, $z_off")
						allScans[m].shiftByOffset(x_off, y_off, z_off)
						//println("${matchedScans[0].beaconsFlipRot}")
						//println("${allScans[m].beaconsFlipRot}")

						//println(matchedScans[mm].beaconsFlipRot.intersect(allScans[m].beaconsFlipRot).size)
						if (matchedScans[mm].beaconsFlipRot.intersect(allScans[m].beaconsFlipRot).size > 11) {  // set to 11
							println("x_off: $x_off, y_off: $y_off, z_off: $z_off, ii: $ii")
							// shift allScan into matchedScan and remove from allScan
							matchedScans.add(allScans[m])
							matchList.add(m)
							//allScans.removeAt(m)
							//mm += 1
							break@Loop
						}
						//println("--?--")
					}

					//allScans[0].beaconsFlipRot.forEach {
					//println("comp1 $comp1, ${Triple(comp1.first + x,comp1.second + y,comp1.third +z)}")
				}
			}
		}
		//}

		matchList.sortDescending()
		matchList.forEach {
			allScans.removeAt(it)
		}
		
		println("matchList $matchList")
		matchList.clear()

		println()

		println("allScans:")
		allScans.forEach {
			println(it.beaconsFlipRot)
		}
		println()
		println("matchedScans:")
		matchedScans.forEach {
			println(it.beaconsFlipRot)
		}
		println()
		
		mm += 1
	}

	var beaconsList = mutableListOf<Triple<Int, Int, Int>>()

	
	println()
	matchedScans.forEach {
		it.beaconsFlipRot.forEach {
			if (!beaconsList.contains(it)) {
				beaconsList.add(it)
			}
		}
	}
	
	solution1 = beaconsList.size
	
	println("beaconsList.size ${beaconsList.size}")


	println()

// tag::output[]
// print solution for part 1
	println("******************************")
	println("--- Day 19: Beacon Scanner ---")
	println("******************************")
	println("Solution for part1")
	println("   $solution1 beacons are there")
	println()
// print solution for part 2
	println("******************************")
	println("Solution for part2")
	println("   $solution2 is the largest Manhattan distance between any two scanners")
	println()
// end::output[]
					t1 = System.currentTimeMillis()-t1
	println("puzzle solved in ${t1} ms")
}