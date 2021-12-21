import java.io.File
import kotlin.math.*

class scanX(field: List<Triple<Int, Int, Int>>, scannerID: String) {
	// this class should represent a single scan with data of puzzle input
	var beacons = field
	var beaconsFlipRot = mutableListOf<Triple<Int, Int, Int>>()
	var scannerID = scannerID

	var scanPosX: Int = 0
	var scanPosY: Int = 0
	var scanPosZ: Int = 0

	// this method is used to flip/rotate the scan
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
					beaconsFlipRot.add(Triple(it.first, -it.second, -it.third))
				}
			}
			(2) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(it.first, it.third, -it.second))
				}
			}
			(3) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(it.first, -it.third, it.second))
				}
			}
			(4) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(-it.first, it.second, -it.third))
				}
			}
			(5) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(-it.first, -it.second, it.third))
				}
			}
			(6) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(-it.first, it.third, it.second))
				}
			}
			(7) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(-it.first, -it.third, -it.second))
				}
			} 
			(8) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(it.second, it.first, -it.third))
				}
			}
			(9) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(it.second, -it.first, it.third))
				}
			}
			(10) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(it.second, it.third, it.first))
				}
			}
			(11) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(it.second, -it.third, -it.first))
				}
			}
			(12) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(-it.second, it.first, it.third))
				}
			}
			(13) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(-it.second, -it.first, -it.third))
				}
			}
			(14) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(-it.second, it.third, -it.first))
				}
			}
			(15) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(-it.second, -it.third, it.first))
				}
			}
			(16) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(it.third, it.first, it.second))
				}
			}
			(17) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(it.third, -it.first, -it.second))
				}
			}
			(18) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(it.third, it.second, -it.first))
				}
			}
			(19) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(it.third, -it.second, it.first))
				}
			}
			(20) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(-it.third, it.first, -it.second))
				}
			}
			(21) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(-it.third, -it.first, it.second))
				}
			}
			(22) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(-it.third, it.second, it.first))
				}
			}
			(23) -> {
				beaconsFlipRot.clear()
				beacons.forEach {
					beaconsFlipRot.add(Triple(-it.third, -it.second, -it.first))
				}
			}
		}
	}

	// this function is used to adapt the offset of the scan
	fun shiftByOffset(in1: Int, in2: Int, in3: Int) {
		for (i in 0..beaconsFlipRot.size - 1) {
			beaconsFlipRot[i] =
				Triple(beaconsFlipRot[i].first + in1, beaconsFlipRot[i].second + in2, beaconsFlipRot[i].third + in3)
		}
	}
	
	// this function is to set the absolut positions 
	fun setPosition(beacon1: Triple<Int, Int, Int>, beacon2: Triple<Int, Int, Int>, flipRot: Int) {
		when (flipRot) {
			(0) -> {
				scanPosX = beacon1.first - beacon2.first
				scanPosY = beacon1.second - beacon2.second
				scanPosZ = beacon1.third - beacon2.third
			}
			(1) -> {  // it.first, -it.second, -it.third
				scanPosX = beacon1.first - beacon2.first
				scanPosY = beacon1.second + beacon2.second
				scanPosZ = beacon1.third + beacon2.third
			}			
			(2) -> {  // it.first, it.third, -it.second
				scanPosX = beacon1.first - beacon2.first
				scanPosY = beacon1.second - beacon2.third
				scanPosZ = beacon1.third + beacon2.second
			}						
			(3) -> {  // it.first, -it.third, it.second
				scanPosX = beacon1.first - beacon2.first
				scanPosY = beacon1.second + beacon2.third
				scanPosZ = beacon1.third - beacon2.second
			}			
			(4) -> {  // -it.first, it.second, -it.third
				scanPosX = beacon1.first + beacon2.first
				scanPosY = beacon1.second - beacon2.second
				scanPosZ = beacon1.third + beacon2.third
			}
			(5) -> {  // -it.first, -it.second, it.third
				scanPosX = beacon1.first + beacon2.first
				scanPosY = beacon1.second + beacon2.second
				scanPosZ = beacon1.third - beacon2.third
			}		
			(6) -> {  // -it.first, it.third, it.second
				scanPosX = beacon1.first + beacon2.first
				scanPosY = beacon1.second - beacon2.third
				scanPosZ = beacon1.third - beacon2.second
			}		
			(7) -> { //  -it.first, -it.third, -it.second
				scanPosX = beacon1.first + beacon2.first
				scanPosY = beacon1.second + beacon2.third
				scanPosZ = beacon1.third + beacon2.second
			}				
			(8) -> { //  it.second, it.first, -it.third
				scanPosX = beacon1.first - beacon2.second
				scanPosY = beacon1.second - beacon2.first
				scanPosZ = beacon1.third + beacon2.third
			}			
			(9) -> { //  it.second, -it.first, it.third
				scanPosX = beacon1.first - beacon2.second
				scanPosY = beacon1.second + beacon2.first
				scanPosZ = beacon1.third - beacon2.third
			}			
			(10) -> { //  it.second, it.third, it.first
				scanPosX = beacon1.first - beacon2.second
				scanPosY = beacon1.second - beacon2.third
				scanPosZ = beacon1.third - beacon2.first
			}			
			(11) -> { //  it.second, -it.third, -it.first
				scanPosX = beacon1.first - beacon2.second
				scanPosY = beacon1.second + beacon2.third
				scanPosZ = beacon1.third + beacon2.first
			}				
			(12) -> { //  -it.second, it.first, it.third
				scanPosX = beacon1.first + beacon2.second
				scanPosY = beacon1.second - beacon2.first
				scanPosZ = beacon1.third - beacon2.third
			}			
			(13) -> { //  -it.second, -it.first, -it.third
				scanPosX = beacon1.first + beacon2.second
				scanPosY = beacon1.second + beacon2.first
				scanPosZ = beacon1.third + beacon2.third
			}			
			(14) -> { //  -it.second, it.third, -it.first
				scanPosX = beacon1.first + beacon2.second
				scanPosY = beacon1.second - beacon2.third
				scanPosZ = beacon1.third + beacon2.first
			}				
			(15) -> { //  -it.second, -it.third, it.first
				scanPosX = beacon1.first + beacon2.second
				scanPosY = beacon1.second + beacon2.third
				scanPosZ = beacon1.third - beacon2.first
			}			
			(16) -> {  // it.third, it.first, it.second
				scanPosX = beacon1.first - beacon2.third
				scanPosY = beacon1.second - beacon2.first
				scanPosZ = beacon1.third - beacon2.second
			}			
			(17) -> {  // it.third, -it.first, -it.second
				scanPosX = beacon1.first - beacon2.third
				scanPosY = beacon1.second + beacon2.first
				scanPosZ = beacon1.third + beacon2.second
			}			
			(18) -> {  // it.third, it.second, -it.first
				scanPosX = beacon1.first - beacon2.third
				scanPosY = beacon1.second - beacon2.second
				scanPosZ = beacon1.third + beacon2.first
			}			
			(19) -> {  // it.third, -it.second, it.first
				scanPosX = beacon1.first - beacon2.third
				scanPosY = beacon1.second + beacon2.second
				scanPosZ = beacon1.third - beacon2.first
			}
			(20) -> {  // -it.third, it.first, -it.second)
				scanPosX = beacon1.first + beacon2.third
				scanPosY = beacon1.second - beacon2.first
				scanPosZ = beacon1.third + beacon2.second
			}		
			(21) -> {  //-it.third, -it.first, it.second
				scanPosX = beacon1.first + beacon2.third
				scanPosY = beacon1.second + beacon2.first
				scanPosZ = beacon1.third - beacon2.second
			}					
			(22) -> {  // -it.third, it.second, it.first
				scanPosX = beacon1.first + beacon2.third
				scanPosY = beacon1.second - beacon2.second
				scanPosZ = beacon1.third - beacon2.first
			}					
			(23) -> { //-it.third, -it.second, -it.first
				scanPosX = beacon1.first + beacon2.third
				scanPosY = beacon1.second + beacon2.second
				scanPosZ = beacon1.third + beacon2.first
			}			
		}
	}
}

//fun main(args: Array<String>) {
	var t1 = System.currentTimeMillis()
	var solution1: Int = 0
	var solution2: Int = 0

	// tag::read_puzzle_input[]
	// setup for reading puzzle input
	var scanInput = mutableListOf<Triple<Int, Int, Int>>()
	var allScans = mutableListOf<scanX>()
	var scanID: String = ""

	// read all scans of puzzle input into scanX classes and add them to a list of unmathced scans (allScans)
	File("day2119_puzzle_input.txt").forEachLine {
		if (it.contains("--")) {
			scanInput.clear()
			scanID = it.drop(4).dropLast(4).replace(" ", "-")
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
			allScans.add(scanX(scanInput.toList(), scanID))
		}
	}
	// end::read_puzzle_input[]

	// tag::matchScans[]
	// initialize list with matched scans (matchedScans) with first element of allScans list
	var matchedScans = mutableListOf<scanX>()
	matchedScans.add(allScans[0])
	matchedScans[0].flipRotate(0)
	matchedScans[0].scanPosX = 0
	matchedScans[0].scanPosY = 0
	matchedScans[0].scanPosY = 0

	allScans.removeAt(0)

	// allScans contains the unmatched scans, if a match is detected the object will be moved to matchedScans
	// so if allScans is empty, no scan to match is left, job done
	var matchList = mutableListOf<Int>()
	var mm: Int = 0	
	
	while (allScans.isNotEmpty()) {

		for (m in 0..allScans.size - 1) {

			Loop@ for (ii in 0..23) {

				allScans[m].flipRotate(ii)

				for (j in 0..matchedScans[mm].beaconsFlipRot.size - 1) {
					
					var beacon1 = matchedScans[mm].beaconsFlipRot[j]

					for (i in 0..allScans[m].beaconsFlipRot.size - 1) {

						var beacon2 = allScans[m].beaconsFlipRot[i]
						var beacon2Raw = allScans[m].beacons[i]

						var x_off = beacon1.first - beacon2.first
						var y_off = beacon1.second - beacon2.second
						var z_off = beacon1.third - beacon2.third

						allScans[m].shiftByOffset(x_off, y_off, z_off)

						if (matchedScans[mm].beaconsFlipRot.intersect(allScans[m].beaconsFlipRot).size > 11) {
							
							// this is the right place to set positions of scans:
							// beacon1 is used as first input. beacon2 is already shifted and transformed, so we need var beacon2Raw = allScans[m].beacons[i]
							allScans[m].setPosition(beacon1, beacon2Raw, ii)

							// shift allScan into matchedScan
							matchedScans.add(allScans[m])
							matchList.add(m)
							break@Loop
						}
					}
				}
			}
		}

		// this is used to know which objects out of allScans where transfered into matchedScans and therefor can be deleted
		matchList.sortDescending()
		matchList.forEach {
			allScans.removeAt(it)
		}
		matchList.clear()

		mm += 1
	}
	// end::matchScans[]
	
    // tag::part1[]  
	// for part1 print all beacons in a list without dublicates and count
	var beaconsList = mutableListOf<Triple<Int, Int, Int>>()

	matchedScans.forEach {
		it.beaconsFlipRot.forEach {
			if (!beaconsList.contains(it)) {
				beaconsList.add(it)
			}
		}
	}
	
	solution1 = beaconsList.size
	// end::part1[]	

	// tag::part2[]
	// for part2 iterate through absolute coordinates of all Scans and calculate manhatten distance
	var manhattenDistMax: Int  = 0
	var manhattenDist: Int = 0
	
	for (i in 0..matchedScans.size-1) {
		for( j in 0..matchedScans.size-1) {
			if (i!=j) {
				manhattenDist = abs(matchedScans[i].scanPosX - matchedScans[j].scanPosX)
				manhattenDist = manhattenDist + abs(matchedScans[i].scanPosY - matchedScans[j].scanPosY)
				manhattenDist = manhattenDist + abs(matchedScans[i].scanPosZ - matchedScans[j].scanPosZ)

				if (manhattenDist > manhattenDistMax) {
					manhattenDistMax = manhattenDist
				} 					
			}
		}
	}
	
	solution2 = manhattenDistMax
	// end::part2[]
	
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
	t1 = System.currentTimeMillis() - t1
	println("puzzle solved in ${t1} ms")
	// end::output[]
//}