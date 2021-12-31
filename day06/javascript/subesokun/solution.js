const fs = require('fs')

// tag::star1[]
/**
 * @param {number[]} fishDayList
 * @param {number} days
 * @returns {number} Total number of fishes
 */
function runSimulation(fishDayList, days) {
    let resetFishesCnt = 0
    for (let day = 1; day <= days; day++) {
        for (let [key, fishCnt] of fishDayList.entries()) {
            if (key === 0) {
                resetFishesCnt = fishCnt
                fishDayList[0] = 0
            } else {
                fishDayList[key - 1] = fishCnt
            }
        }
        fishDayList[8] = resetFishesCnt
        fishDayList[6] += resetFishesCnt
    }
    return fishDayList.reduce((pV, cV) => pV + cV, 0)
}

/**
 * @param {number[]} fishDayList 
 */
function runSolutionPuzzleOne(fishDayList) {
    const result = runSimulation(fishDayList, 80)
    console.log(`Solution to first puzzle: ${result}`)
}
// end::star1[]

// tag::star2[]
/**
 * @param {number[]} fishDayList 
 */
function runSolutionPuzzleTwo(fishDayList) {
    const result = runSimulation(fishDayList, 256)
    console.log(`Solution to second puzzle: ${result}`)
}
// end::star2[]

// tag::input[]
const fishDayList = new Array(9).fill(0)
fs.readFileSync('input.txt')
    .toString()
    .split(',')
    .forEach(fishValueRaw => {
        fishDayList[parseInt(fishValueRaw, 10)] += 1
    })
    // end::input[]

runSolutionPuzzleOne([...fishDayList])
runSolutionPuzzleTwo([...fishDayList])