const fs = require('fs')

// tag::star1[]
/**
 * @param {number[]} crabList 
 */
function runSolutionPuzzleOne(crabList) {
    const minPos = Math.min(...crabList)
    const maxPos = Math.max(...crabList)
    let minFuelConsumption = Infinity
    for (let pos = minPos; pos <= maxPos; pos++) {
        const fuelConsumption = crabList.reduce((pV, cV) => pV + Math.abs(cV - pos), 0)
        if (fuelConsumption < minFuelConsumption) {
            minFuelConsumption = fuelConsumption
        }
    }
    console.log(`Solution to first puzzle: ${minFuelConsumption}`)
}
// end::star1[]

// tag::star2[]
/**
 * @param {number} fromPos 
 * @param {number} toPos 
 * @returns {number} Fuel consumption
 */
function calcFuelConsumption(fromPos, toPos) {
    let result = 0
    for (let i = 1; i <= Math.abs(toPos - fromPos); i++) {
        result += i
    }
    return result
}

/**
 * @param {number[]} crabList 
 */
function runSolutionPuzzleTwo(crabList) {
    const minPos = Math.min(...crabList)
    const maxPos = Math.max(...crabList)
    let minFuelConsumption = Infinity
    for (let pos = minPos; pos <= maxPos; pos++) {
        const fuelConsumption = crabList.reduce((pV, cV) => pV + calcFuelConsumption(cV, pos), 0)
        if (fuelConsumption < minFuelConsumption) {
            minFuelConsumption = fuelConsumption
        }
    }
    console.log(`Solution to second puzzle: ${minFuelConsumption}`)
}
// end::star2[]

// tag::input[]
const crabList = fs.readFileSync('input.txt')
    .toString()
    .split(',')
    .map(v => parseInt(v, 10))
    // end::input[]

runSolutionPuzzleOne(crabList)
runSolutionPuzzleTwo(crabList)