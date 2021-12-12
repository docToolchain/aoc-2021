const fs = require('fs')

// tag::star1[]
/**
 * @param {Array.<{x1: number, y1: number, x2: number, y2: number}>} lineList
 * @returns {Map<string,number>}
 */
function rasterizeLines(lineList) {
    /** @type {Map<string,number>} */
    const rasterizeMap = new Map()
    for (const line of lineList) {
        let currentPos = { x: line.x1, y: line.y1 }
        const lineXLen = Math.abs(line.x2 - line.x1)
        const lineYLen = Math.abs(line.y2 - line.y1)
        const xStepDelta = line.x1 === line.x2 ? 0 : (line.x1 <= line.x2 ? 1 : -1)
        const yStepDelta = line.y1 === line.y2 ? 0 : (line.y1 <= line.y2 ? 1 : -1)
        while (Math.abs(currentPos.x - line.x1) <= lineXLen && Math.abs(currentPos.y - line.y1) <= lineYLen) {
            const pHash = `${currentPos.x},${currentPos.y}`
            const p = rasterizeMap.get(pHash)
            rasterizeMap.set(pHash, p !== undefined ? p + 1 : 1)
            currentPos.x += xStepDelta
            currentPos.y += yStepDelta
        }
    }
    return rasterizeMap
}

/**
 * @param {Array.<{x1: number, y1: number, x2: number, y2: number}>} lineList 
 */
function runSolutionPuzzleOne(lineList) {
    const rasterizeMap = rasterizeLines(lineList.filter(p => p.x1 === p.x2 || p.y1 === p.y2))
    let overlapCnt = 0
    for (const value of rasterizeMap.values()) {
        if (value > 1) {
            overlapCnt++
        }
    }
    console.log(`Solution to first puzzle: ${overlapCnt}`)
}
// end::star1[]

// tag::star2[]

/**
 * @param {Array.<{x1: number, y1: number, x2: number, y2: number}>} lineList 
 */
function runSolutionPuzzleTwo(lineList) {
    const rasterizeMap = rasterizeLines(lineList)
    let overlapCnt = 0
    for (const value of rasterizeMap.values()) {
        if (value > 1) {
            overlapCnt++
        }
    }
    console.log(`Solution to second puzzle: ${overlapCnt}`)
}
// end::star2[]

// tag::input[]
const rawInput = fs.readFileSync('input.txt').toString().split('\n')
const lineList = []
for (const line of rawInput) {
    const [p1, p2] = line
        .split(' -> ')
        .map(pRaw =>
            pRaw.split(',').map(v => parseInt(v, 10))
        )
    lineList.push({ x1: p1[0], y1: p1[1], x2: p2[0], y2: p2[1] })
}
// end::input[]

runSolutionPuzzleOne(lineList)
runSolutionPuzzleTwo(lineList)