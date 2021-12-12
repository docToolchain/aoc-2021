const fs = require('fs')

// tag::star1[]
function runSolutionPuzzleOne(input) {
    let lastDepth = Infinity
    let increaseCnt = 0
    for (const depth of input) {
        if (lastDepth < depth) {
            increaseCnt++
        }
        lastDepth = depth
    }
    console.log(`Solution to first puzzle: ${increaseCnt}`)
}
// end::star1[]

// tag::star2[]
function runSolutionPuzzleTwo(input) {
    let lastDepthSum = Infinity
    let increaseCnt = 0
    for (let i = 0; i + 2 < input.length; i++) {
        const depthSum = input[i] + input[i + 1] + input[i + 2]
        if (lastDepthSum < depthSum) {
            increaseCnt++
        }
        lastDepthSum = depthSum
    }
    console.log(`Solution to second puzzle: ${increaseCnt}`)
}
// end::star2[]

const input = fs.readFileSync('input.txt').toString().split("\n").map(v => parseInt(v, 10))

runSolutionPuzzleOne(input)
runSolutionPuzzleTwo(input)