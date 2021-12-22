const fs = require('fs')

// tag::star1[]
/**
 * Dijkstra algorithm with optimized min heap
 * @param {Array.<Array.<number>>} riskMatrix
 * @returns {number}
 */
function getPathWithMinRisk(riskMatrix) {
    /** @type {Array.<Array.<{x: number, y: number, risk: number}>>} */
    const gridMatrix = new Array(riskMatrix.length)
        .fill(null)
        .map(_ => new Array(riskMatrix.length).fill(null))
    let minHeap = [{ x: 0, y: 0, risk: 0 }]
    gridMatrix[0][0] = minHeap[0]
    const tXY = riskMatrix.length - 1
    const adjacentGrids = [
        [-1, 0],
        [1, 0],
        [0, -1],
        [0, 1]
    ]
    while (minHeap.length > 0) {
        const u = minHeap.shift()
        if (u.x === tXY && u.y === tXY) {
            return u.risk
        }
        for (const [dX, dY] of adjacentGrids) {
            const vX = u.x + dX
            const vY = u.y + dY
            if (0 <= vX && vX < riskMatrix.length && 0 <= vY && vY < riskMatrix.length) {
                const uvRisk = u.risk + riskMatrix[vY][vX]
                let v = gridMatrix[vY][vX]
                if (v === null) {
                    v = { x: vX, y: vY, risk: uvRisk }
                    minHeap.unshift(v)
                    gridMatrix[vY][vX] = v
                } else if (uvRisk < v.risk) {
                    v.risk = uvRisk
                }
            }
        }
        minHeap.sort((a, b) => a.risk - b.risk)
    }
}

/**
 * @param {Array.<Array.<number>>} riskMatrix
 */
function runSolutionPuzzleOne(riskMatrix) {
    const result = getPathWithMinRisk(riskMatrix)
    console.log(`Solution to first puzzle: ${result}`)
}
// end::star1[]

// tag::star2[]
/**
 * @param {Array.<Array.<number>>} riskMatrix
 */
function runSolutionPuzzleTwo(riskMatrix) {
    const factor = 5
    const fullRiskMatrix = new Array(riskMatrix.length * factor)
        .fill(null)
        .map(_ => new Array(riskMatrix.length * factor).fill(null))
    for (n = 0; n < factor; n++) {
        for (m = 0; m < factor; m++) {
            const yOffset = n * riskMatrix.length
            for (let y = 0; y < riskMatrix.length; y++) {
                const xOffset = m * riskMatrix.length
                for (let x = 0; x < riskMatrix.length; x++) {
                    fullRiskMatrix[yOffset + y][xOffset + x] = ((riskMatrix[y][x] + n + m - 1) % 9) + 1
                }
            }
        }
    }
    const result = getPathWithMinRisk(fullRiskMatrix)
    console.log(`Solution to second puzzle: ${result}`)
}
// end::star2[]

// tag::input[]
/** @type {Array.<Array.<number>>} */
let riskMatrix = fs.readFileSync('input.txt')
    .toString()
    .split('\n')
    .map(line => line.split('').map(v => parseInt(v, 10)))
    // end::input[]

runSolutionPuzzleOne(riskMatrix)
runSolutionPuzzleTwo(riskMatrix)