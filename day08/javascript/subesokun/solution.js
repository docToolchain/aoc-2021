const fs = require('fs')

// tag::star1[]
/**
 * @param {Array.<{usp: string[], output: string[]}>} entryList 
 */
function runSolutionPuzzleOne(entryList) {
    let simpleDigitsCnt = 0
    for (let entry of entryList) {
        for (let digitSignalLineCnt of entry.output.map(v => v.length)) {
            if ([2, 4, 3, 7].includes(digitSignalLineCnt)) {
                simpleDigitsCnt++
            }
        }
    }
    console.log(`Solution to first puzzle: ${simpleDigitsCnt}`)
}
// end::star1[]

// tag::star2[]
/**
 * @param {string[]} outputList 
 * @returns {string[]} Initialized digit list containing simple digits only
 */
function initializeDigitList(outputList) {
    const digitList = new Array(10).fill(null)
    for (const [index, digitSignalLineCnt] of outputList.map(v => v.length).entries()) {
        if (digitSignalLineCnt === 2) {
            digitList[1] = outputList[index]
        } else if (digitSignalLineCnt === 4) {
            digitList[4] = outputList[index]
        } else if (digitSignalLineCnt === 3) {
            digitList[7] = outputList[index]
        } else if (digitSignalLineCnt === 7) {
            digitList[8] = outputList[index]
        }
    }
    return digitList
}

/**
 * @param {string} digitA 
 * @param {string} digitB 
 * @returns {string} Distinct signal
 */
function getDistinctSignals(digitA, digitB) {
    const digitACharList = digitA.split('')
    const digitBCharList = digitB.split('')
    let result = ''
    for (const d of digitACharList) {
        if (!digitBCharList.includes(d)) {
            result += d
        }
    }
    for (const d of digitBCharList) {
        if (!digitACharList.includes(d)) {
            result += d
        }
    }
    return result
}

/**
 * @param {Array.<{usp: string[], output: string[]}>} entryList 
 */
function runSolutionPuzzleTwo(entryList) {
    let totalEntryValue = 0
    for (const entry of entryList) {
        // Initialize digit list with the simple digits
        const digitList = initializeDigitList(entry.usp)

        // Do a little deduction
        const aSignal = getDistinctSignals(digitList[1], digitList[7])
        const cfSignal = digitList[1]
        digitList[3] = entry.usp.filter(v => v.length === 5 && getDistinctSignals(cfSignal, v).length === 3)[0]
        const egSignal = getDistinctSignals(aSignal, getDistinctSignals(digitList[4], digitList[8]))
        digitList[9] = entry.usp.filter(v => v.length === 6 && getDistinctSignals(egSignal, v).length === 6)[0]
        const eSignal = getDistinctSignals(digitList[8], digitList[9])
        digitList[2] = entry.usp.filter(v => v.length === 5 && getDistinctSignals(eSignal, v).length === 4)[0]
        digitList[5] = entry.usp.filter(v => v.length === 5 && v !== digitList[2] && v !== digitList[3])[0]
        const dgSignal = getDistinctSignals(aSignal, getDistinctSignals(digitList[1], digitList[3]))
        digitList[6] = entry.usp.filter(v => v.length === 6 && getDistinctSignals(dgSignal, v).length === 4 && v !== digitList[9])[0]
        digitList[0] = entry.usp.filter(v => v.length === 6 && v !== digitList[6] && v !== digitList[9])[0]

        // Finally, convert output to number
        const digitCharMap = new Map()
        for (const [digit, chars] of digitList.entries()) {
            digitCharMap.set(chars, digit)
        }
        const entryValue = parseInt(entry.output.reduce((pV, cV) => `${pV}${digitCharMap.get(cV)}`, ''), 10)

        totalEntryValue += entryValue
    }
    console.log(`Solution to second puzzle: ${totalEntryValue}`)
}
// end::star2[]

// tag::input[]
/**
 * @param {string} input 
 * @returns {string} Sorted input
 */
function sortChars(input) {
    return input.split('').sort().join('')
}

/** @type {Array.<{usp: string[], output: string[]}>} */
const entryList = fs.readFileSync('input.txt')
    .toString()
    .split('\n')
    .map(line => {
        const [uspRaw, outputRaw] = line.split(' | ')
        return {
            usp: uspRaw.split(' ').map(sortChars),
            output: outputRaw.split(' ').map(sortChars)
        }
    })
    // end::input[]

runSolutionPuzzleOne(entryList)
runSolutionPuzzleTwo(entryList)