const fs = require('fs')

// tag::star1[]
/**
 * @param {Array.<Array.<number>>} heighMatrix 
 * @param {number} xPos 
 * @param {number} yPos 
 * @returns {boolean} Is minimum
 */
function isMinimum(heighMatrix, xPos, yPos) {
    const currentHeight = heighMatrix[yPos][xPos]
    return heighMatrix[yPos][xPos - 1] > currentHeight &&
        heighMatrix[yPos][xPos + 1] > currentHeight &&
        heighMatrix[yPos + 1][xPos] > currentHeight &&
        heighMatrix[yPos - 1][xPos] > currentHeight
}

/**
 * @param {Array.<Array.<boolean>>} localMinimumMatrix 
 * @param {number} xPos 
 * @param {number} yPos 
 */
function setLocalMinimum(localMinimumMatrix, xPos, yPos) {
    localMinimumMatrix[yPos][xPos] = true
    localMinimumMatrix[yPos][xPos - 1] = false
    localMinimumMatrix[yPos][xPos + 1] = false
    localMinimumMatrix[yPos - 1][xPos] = false
    localMinimumMatrix[yPos + 1][xPos] = false
}

/**
 * @param {Array.<Array.<number>>} heighMatrix 
 * @returns {Array.<Array.<number>>} Local minimum matrix
 */
function generateLocalMinimumMatrix(heighMatrix) {
    const localMinimumMatrix = new Array(heighMatrix.length).fill(null).map(() => new Array(heighMatrix[0].length).fill(false))
    for (let xPos = 1; xPos < heighMatrix[0].length - 1; xPos++) {
        for (let yPos = 1; yPos < heighMatrix.length - 1; yPos++) {
            if (isMinimum(heighMatrix, xPos, yPos)) {
                setLocalMinimum(localMinimumMatrix, xPos, yPos)
            }
        }
    }
    return localMinimumMatrix
}

/**
 * @param {Array.<Array.<number>>} heighMatrix 
 */
function runSolutionPuzzleOne(heighMatrix) {
    const localMinimumMatrix = generateLocalMinimumMatrix(heighMatrix)
    let totalRiskLevel = 0
    for (let xPos = 1; xPos < heighMatrix[0].length - 1; xPos++) {
        for (let yPos = 1; yPos < heighMatrix.length - 1; yPos++) {
            if (localMinimumMatrix[yPos][xPos]) {
                totalRiskLevel += heighMatrix[yPos][xPos] + 1
            }
        }
    }
    console.log(`Solution to first puzzle: ${totalRiskLevel}`)
}
// end::star1[]

// tag::star2[]
/**
 * @param {Array.<Array.<number>>} heighMatrix
 * @param {number} xPos
 * @param {number} yPos
 * @returns {number} Size of basin
 */
function recurseBasin(heighMatrix, xPos, yPos) {
    if (heighMatrix[yPos][xPos] < 9) {
        heighMatrix[yPos][xPos] = 9 // Mark as visited
        let size = 1
        size += recurseBasin(heighMatrix, xPos - 1, yPos)
        size += recurseBasin(heighMatrix, xPos + 1, yPos)
        size += recurseBasin(heighMatrix, xPos, yPos - 1)
        size += recurseBasin(heighMatrix, xPos, yPos + 1)
        return size
    } else {
        return 0
    }
}

/**
 * @param {Array.<Array.<number>>} heighMatrix 
 */
function runSolutionPuzzleTwo(heighMatrix) {
    const localMinimumMatrix = generateLocalMinimumMatrix(heighMatrix)
    const basinList = []
    for (let xPos = 1; xPos < heighMatrix[0].length - 1; xPos++) {
        for (let yPos = 1; yPos < heighMatrix.length - 1; yPos++) {
            if (localMinimumMatrix[yPos][xPos]) {
                basinList.push(recurseBasin(heighMatrix, xPos, yPos))
            }
        }
    }
    const result = basinList.sort((a, b) => b - a).slice(0, 3).reduce((pV, cV) => pV * cV)
    console.log(`Solution to second puzzle: ${result}`)
}
// end::star2[]

// tag::input[]
/** @type {Array.<Array.<number>>} */
let heighMatrix = fs.readFileSync('input.txt')
    .toString()
    .split('\n')
    .map(line => [10, ...line.split('').map(v => parseInt(v, 10)), 10])
    // end::input[]
heighMatrix = [new Array(heighMatrix[0].length).fill(10), ...heighMatrix, new Array(heighMatrix[0].length).fill(10)]

runSolutionPuzzleOne(heighMatrix)
runSolutionPuzzleTwo(heighMatrix)