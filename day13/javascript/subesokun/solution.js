const fs = require('fs')

// tag::star1[]
/**
 * @param {Map<string, [{x: number, y: number}]>} dotMap
 * @param {{axis: string, value: number}} instruction
 */
function fold(dotMap, instruction) {
    for (const [dotHash, dot] of dotMap.entries()) {
        if (instruction.value < dot[instruction.axis]) {
            const axisPos = instruction.value - (dot[instruction.axis] - instruction.value)
            const newDot = {...dot }
            newDot[instruction.axis] = axisPos
            if (!dotMap.has(`${newDot.x},${newDot.y}`)) {
                dotMap.set(`${newDot.x},${newDot.y}`, { x: newDot.x, y: newDot.y })
            }
            dotMap.delete(dotHash)
        }
    }
}

/**
 * @param {Map<string, [{x: number, y: number}]>} dotMap
 * @param {Array.<{axis: string, value: number}>} instructionList
 */
function runSolutionPuzzleOne(dotMap, instructionList) {
    fold(dotMap, instructionList[0])
    console.log(`Solution to first puzzle: ${dotMap.size}`)
}
// end::star1[]

// tag::star2[]
/**
 * @param {Map<string, [{x: number, y: number}]>} dotMap
 */
function plotLetters(dotMap) {
    let xMax = 0
    let yMax = 0
    for (const dot of dotMap.values()) {
        xMax = Math.max(xMax, dot.x)
        yMax = Math.max(yMax, dot.y)
    }
    for (let y = 0; y < yMax + 1; y++) {
        let line = ''
        for (let x = 0; x < xMax + 1; x++) {
            line += dotMap.has(`${x},${y}`) ? '#' : '.'
        }
        console.log(line)
    }
}

/**
 * @param {Map<string, [{x: number, y: number}]>} dotMap
 * @param {Array.<{axis: string, value: number}>} instructionList
 */
function runSolutionPuzzleTwo(dotMap, instructionList) {
    for (const instruction of instructionList) {
        fold(dotMap, instruction)
    }
    console.log('Solution to second puzzle: (see plot)')
    plotLetters(dotMap)
}
// end::star2[]

// tag::input[]
/** @type {Array.<Array.<string>>} */
const [dotListRaw, foldInstructionListRaw] = fs.readFileSync('input.txt')
    .toString()
    .split('\n\n')
    /** @type {Map<string, [{x: number, y: number}]>} */
const dotMap = new Map(dotListRaw
    .split('\n')
    .map(line => {
        const [x, y] = line.split(',').map(v => parseInt(v, 10))
        return [`${x},${y}`, { x, y }]
    }))

const foldInstructionRegex = /fold along (.+)=(\d+)/
const foldInstructionList = foldInstructionListRaw
    .split('\n')
    .map(line => {
        const match = foldInstructionRegex.exec(line)
        return { axis: match[1], value: parseInt(match[2], 10) }
    })
    // end::input[]

runSolutionPuzzleOne(new Map(JSON.parse(JSON.stringify(Array.from(dotMap)))), foldInstructionList) // Ugly deep clone 🤷‍♂️
runSolutionPuzzleTwo(new Map(JSON.parse(JSON.stringify(Array.from(dotMap)))), foldInstructionList) // Ugly deep clone 🤷‍♂️