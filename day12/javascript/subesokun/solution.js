const fs = require('fs')

// tag::star1[]
/**
 * @param {string} edge
 * @returns {boolean}
 */
function isSmallCave(edge) {
    return edge === edge.toLocaleLowerCase()
}

/**
 * @param {string[]} traversedPath 
 * @returns {*}
 */
function simpleFilterGenerator(traversedPath) {
    return (edge) => {
        return !isSmallCave(edge) || !traversedPath.includes(edge)
    }
}

/**
 * @param {Map<string,Array.<string>>} edgeMap
 * @param {string[]} traversedPath
 * @param {*} edgeFilterGenerator
 * @returns {string[]}
 */
function findDistinctPaths(edgeMap, traversedPath, edgeFilterGenerator) {
    /** @type {string[]} */
    let result = []
    const lastEdge = traversedPath[traversedPath.length - 1]
    const edgeFilter = edgeFilterGenerator(traversedPath)
    const nextEdges = edgeMap.get(lastEdge).filter(edgeFilter)
    for (const edge of nextEdges) {
        if (edge === 'end') {
            result.push([...traversedPath, edge])
        } else {
            result = result.concat([...findDistinctPaths(edgeMap, [...traversedPath, edge], edgeFilterGenerator)])
        }
    }
    return result
}

/**
 * @param {Map<string,Array.<string>>} edgeMap 
 */
function runSolutionPuzzleOne(edgeMap) {
    const paths = findDistinctPaths(edgeMap, ['start'], simpleFilterGenerator)
    console.log(`Solution to first puzzle: ${paths.length}`)
}
// end::star1[]

// tag::star2[]
/**
 * @param {string[]} traversedPath 
 * @returns {*}
 */
function complexFilterGenerator(traversedPath) {
    const maxSmallCaveVisitCount = Math.max(...traversedPath.filter(isSmallCave).map(v => traversedPath.filter(vv => vv === v).length))
    return (edge) => {
        return !isSmallCave(edge) || (maxSmallCaveVisitCount < 2 && isSmallCave(edge)) || !traversedPath.includes(edge)
    }
}

/**
 * @param {Map<string,string[]>} edgeMap 
 */
function runSolutionPuzzleTwo(edgeMap) {
    const paths = findDistinctPaths(edgeMap, ['start'], complexFilterGenerator)
    console.log(`Solution to second puzzle: ${paths.length}`)
}
// end::star2[]

// tag::input[]
/** @type {Array.<Array.<string>>} */
const pathList = fs.readFileSync('input.txt')
    .toString()
    .split('\n')
    .map(line => line.split('-'))
    /** @type {Map<string,string[]>} */
const edgeMap = new Map()
for (const path of pathList) {
    if (path[1] !== 'start') {
        if (edgeMap.has(path[0])) {
            edgeMap.get(path[0]).push(path[1])
        } else {
            edgeMap.set(path[0], [path[1]])
        }
    }
    if (path[0] !== 'start' && path[1] !== 'end') {
        if (edgeMap.has(path[1])) {
            edgeMap.get(path[1]).push(path[0])
        } else {
            edgeMap.set(path[1], [path[0]])
        }
    }
}
// end::input[]

runSolutionPuzzleOne(edgeMap)
runSolutionPuzzleTwo(edgeMap)