const fs = require('fs')

// tag::star1[]
/**
 * We just need to count the number of pairs and the numbers of single chars added.
 * The correct order of the pairs in the template string is irrelevant hence
 * we don't need to perform here any complex string modifications. This makes
 * the solution pretty simple :o)
 * @param {string} template 
 * @param {number} steps 
 * @param {Map<string, string>} rules 
 * @returns {Map<string, number>}
 */
function getCharCountForTemplate(template, steps, rules) {
    const pairMap = initializePairMap(template)
    const charCountMap = countChars(template)
    for (let step = 1; step <= steps; step++) {
        const pairMapClone = new Map([...pairMap])
        for (const [pair, count] of pairMapClone.entries()) {
            // Add new pairs depending on the quantity of the current pair
            const insertChar = rules.get(pair)
            const leftPair = pair[0] + insertChar
            const rightPair = insertChar + pair[1]
            incrementMapValue(pairMap, leftPair, count)
            incrementMapValue(pairMap, rightPair, count)

            // Update char count
            incrementMapValue(charCountMap, insertChar, count)

            // Finally, remove current pairs as we've split them up into two new pairs
            decrementMapValue(pairMap, pair, count)
            if (pairMap.get(pair) === 0) {
                pairMap.delete(pair)
            }
        }
    }
    return charCountMap
}

/**
 * Map of pairs appearing in the template string and their count
 * @param {string} template
 * @returns {Map<string, Map<string, number>>}
 */
function initializePairMap(template) {
    /** @type {Map<string, number>} */
    const pairMap = new Map()
    for (let i = 0; i < template.length - 1; i++) {
        const pair = template[i] + template[i + 1]
        pairMap.set(pair, pairMap.has(pair) ? pairMap.get(pair) + 1 : 1)
    }
    return pairMap
}

/**
 * @param {string} template 
 * @returns {Map<string, number>}
 */
function countChars(template) {
    const resultMap = new Map()
    for (const char of template) {
        incrementMapValue(resultMap, char, 1)
    }
    return resultMap
}

function incrementMapValue(map, key, value) {
    map.set(key, map.has(key) ? map.get(key) + value : value)
}

function decrementMapValue(map, key, value) {
    map.set(key, map.get(key) - value)
}

/**
 * @param {string} template 
 * @param {Map<string, string>} rules 
 */
function runSolutionPuzzleOne(template, rules) {
    const result = getCharCountForTemplate(template, 10, rules)
    const leastCommonCnt = Math.min(...result.values())
    const mostCommonCnt = Math.max(...result.values())
    console.log(`Solution to first puzzle: ${mostCommonCnt - leastCommonCnt}`)
}
// end::star1[]

// tag::star2[]
/**
 * @param {string} template 
 * @param {Map<string, string>} rules 
 */
function runSolutionPuzzleTwo(template, rules) {
    const result = getCharCountForTemplate(template, 40, rules)
    const leastCommonCnt = Math.min(...result.values())
    const mostCommonCnt = Math.max(...result.values())
    console.log(`Solution to second puzzle: ${mostCommonCnt - leastCommonCnt}`)
}
// end::star2[]

// tag::input[]
const [template, rulesRaw] = fs.readFileSync('input.txt')
    .toString()
    .split('\n\n')
const rulesMap = new Map(rulesRaw
        .split('\n')
        .map(line => line.split(' -> ')))
    // end::input[]

runSolutionPuzzleOne(template, rulesMap)
runSolutionPuzzleTwo(template, rulesMap)