const fs = require('fs')

// tag::star1[]
/**
 * @param {Array.<Array.<number>>} octopusMatrix 
 * @param {number} x
 * @param {number} y 
 */
function flashOctopus(octopusMatrix, x, y) {
    octopusMatrix[y][x] = 0
    for (let ty = Math.max(y - 1, 0); ty <= Math.min(y + 1, octopusMatrix.length - 1); ty++) {
        for (let tx = Math.max(x - 1, 0); tx <= Math.min(x + 1, octopusMatrix[0].length - 1); tx++) {
            if (!(ty === y && tx === x) && octopusMatrix[ty][tx] !== 0) {
                octopusMatrix[ty][tx] += 1
            }
        }
    }
}

/**
 * @param {Array.<Array.<number>>} octopusMatrix 
 * @returns {number} Number of flashes ocurred in this step
 */
function executeStep(octopusMatrix) {
    for (let y = 0; y < octopusMatrix.length; y++) {
        for (let x = 0; x < octopusMatrix[0].length; x++) {
            octopusMatrix[y][x] += 1
        }
    }
    let didFlash = true
    let flashCnt = 0
    while (didFlash) {
        didFlash = false
        for (let y = 0; y < octopusMatrix.length; y++) {
            for (let x = 0; x < octopusMatrix[0].length; x++) {
                if (octopusMatrix[y][x] > 9) {
                    flashCnt++
                    didFlash = true
                    flashOctopus(octopusMatrix, x, y)
                }
            }
        }
    }
    return flashCnt
}

/**
 * @param {Array.<Array.<number>>} octopusMatrix 
 */
function runSolutionPuzzleOne(octopusMatrix) {
    let flashCnt = 0
    for (let step = 1; step <= 100; step++) {
        flashCnt += executeStep(octopusMatrix)
    }
    console.log(`Solution to first puzzle: ${flashCnt}`)
}
// end::star1[]

// tag::star2[]

/**
 * @param {Array.<Array.<number>>} octopusMatrix 
 */
function runSolutionPuzzleTwo(octopusMatrix) {
    const maxFlashCnt = octopusMatrix.length * octopusMatrix[0].length
    let lastFlashCnt = 0
    let stepCnt = 0
    do {
        stepCnt++
        lastFlashCnt = executeStep(octopusMatrix)
    } while (lastFlashCnt !== maxFlashCnt)
    console.log(`Solution to second puzzle: ${stepCnt}`)
}
// end::star2[]

// tag::input[]
/** @type {Array.<Array.<number>>} */
const octopusMatrix = fs.readFileSync('input.txt')
    .toString()
    .split('\n')
    .map(line => line.split('').map(v => parseInt(v, 10)))
    // end::input[]

runSolutionPuzzleOne(JSON.parse(JSON.stringify(octopusMatrix)))
runSolutionPuzzleTwo(JSON.parse(JSON.stringify(octopusMatrix)))