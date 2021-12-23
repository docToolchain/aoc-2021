const fs = require('fs')

// tag::star1[]
/** @type {Map<string,string>} */
const commandMap = new Map([
    ['(', ')'],
    ['[', ']'],
    ['{', '}'],
    ['<', '>']
])
const startCommandChars = [...commandMap.keys()]

/** @type {Map<string,number>} */
const corruptScoreMap = new Map([
    [')', 3],
    [']', 57],
    ['}', 1197],
    ['>', 25137]
])

/**
 * @param {string} line 
 * @returns {string}
 */
function getFirstCorruptedChar(line) {
    const stack = []
    for (const c of line) {
        if (startCommandChars.includes(c)) {
            stack.push(c)
        } else {
            const cPrev = stack.pop()
            if (commandMap.get(cPrev) !== c) {
                return c
            }
        }
    }
    return null
}

/**
 * @param {Array.<string>} lineList 
 */
function runSolutionPuzzleOne(lineList) {
    const corruptLineChars = new Map([
        [')', 0],
        [']', 0],
        ['}', 0],
        ['>', 0]
    ])
    for (const line of lineList) {
        const c = getFirstCorruptedChar(line)
        if (c !== null) {
            corruptLineChars.set(c, corruptLineChars.get(c) + 1)
        }
    }
    const points = [...corruptLineChars.entries()].reduce((pV, [c, count]) => pV + corruptScoreMap.get(c) * count, 0)
    console.log(`Solution to first puzzle: ${points}`)
}
// end::star1[]

// tag::star2[]
/** @type {Map<string,number>} */
const incompleteScoreMap = new Map([
    [')', 1],
    [']', 2],
    ['}', 3],
    ['>', 4]
])

/**
 * @param {string} line
 * @returns {string}
 */
function generateCompletionString(line) {
    const stack = []
    for (const c of line) {
        startCommandChars.includes(c) ? stack.push(c) : stack.pop()
    }
    return stack.reverse().reduce((pV, c) => pV + commandMap.get(c), '')
}

/**
 * @param {string} value
 * @returns {number}
 */
function scoreCompletionString(value) {
    let score = 0
    for (const c of value) {
        score = score * 5 + incompleteScoreMap.get(c)
    }
    return score
}

/**
 * @param {Array.<string>} lineList 
 */
function runSolutionPuzzleTwo(lineList) {
    const scoreList = []
    for (const line of lineList) {
        if (getFirstCorruptedChar(line) === null) {
            const completionString = generateCompletionString(line)
            scoreList.push(scoreCompletionString(completionString))
        }
    }
    const sortedScoreList = scoreList.sort((a, b) => a - b)
    const middleScore = sortedScoreList[Math.floor(sortedScoreList.length / 2)]
    console.log(`Solution to second puzzle: ${middleScore}`)
}
// end::star2[]

// tag::input[]
/** @type {Array.<string>} */
let lineList = fs.readFileSync('input.txt')
    .toString()
    .split('\n')
    // end::input[]

runSolutionPuzzleOne(lineList)
runSolutionPuzzleTwo(lineList)