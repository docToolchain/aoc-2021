const fs = require('fs')

// tag::star1[]
/**
 * @param {Array<string[]>} input 
 */
function runSolutionPuzzleOne(input) {
    const gammaCommonBitArray = new Array(input[0].length).fill(0)
    for (const line of input) {
        for (const [i, bitChar] of line.entries()) {
            if (bitChar === '0') {
                gammaCommonBitArray[i] -= 1
            } else {
                gammaCommonBitArray[i] += 1
            }
        }
    }
    const gammaBinStr = gammaCommonBitArray.reduce((binString, value) => value < 0 ? binString + '0' : binString + '1', '')
    const epsilonBinStr = gammaCommonBitArray.reduce((binString, value) => value < 0 ? binString + '1' : binString + '0', '')
    const gamma = parseInt(gammaBinStr, 2)
    const epsilon = parseInt(epsilonBinStr, 2)
    console.log(`Solution to first puzzle: ${gamma * epsilon}`)
}
// end::star1[]

// tag::star2[]
function getMostCommonValueForIndex(index, input) {
    let result = 0
    for (const line of input) {
        const bitChar = line[index]
        if (bitChar === '0') {
            result -= 1
        } else {
            result += 1
        }
    }
    return result >= 0 ? '1' : '0'
}

function filterInputByIndexValue(input, index, value) {
    return input.filter(line => line[index] === value)
}

function calculateRating(input, filterForMostCommon) {
    let currentInput = JSON.parse(JSON.stringify(input)) // Clone 2d Array. Slow but... ¯\_(ツ)_/¯
    for (let i = 0; i < input[0].length && currentInput.length > 1; i++) {
        let filterValue = getMostCommonValueForIndex(i, currentInput)
        if (!filterForMostCommon) {
            filterValue = filterValue === '1' ? '0' : '1'
        }
        currentInput = filterInputByIndexValue(currentInput, i, filterValue)
    }
    return parseInt(currentInput[0].join(''), 2)
}

function runSolutionPuzzleTwo(input) {
    const oxygen = calculateRating(input, true)
    const co2ScrubberRate = calculateRating(input, false)
    console.log(oxygen, co2ScrubberRate)
    console.log(`Solution to second puzzle: ${oxygen * co2ScrubberRate}`)
}
// end::star2[]

// tag::input[]
const rawInput = fs.readFileSync('input.txt').toString().split("\n")
const input = []
for (const line of rawInput) {
    input.push(line.split(''))
}
// end::input[]

runSolutionPuzzleOne(input)
runSolutionPuzzleTwo(input)