const fs = require('fs')

// tag::star1[]
/**
 * @param {Array.<Array.<{value: number, marked: boolean}>>} board 
 * @param {number} lastSetRowIndex 
 * @param {number} lastSetColumnIndex 
 * @returns boolean
 */
function checkIfBingo(board, lastSetRowIndex, lastSetColumnIndex) {
    const isRowCompleted = board[lastSetRowIndex]
        .map(boardEntry => boardEntry.marked)
        .reduce((previousValue, currentValue) => currentValue && previousValue, true)
    const isColumnCompleted = board
        .map(row => row[lastSetColumnIndex])
        .map(boardEntry => boardEntry.marked)
        .reduce((previousValue, currentValue) => currentValue && previousValue, true)
    return isRowCompleted || isColumnCompleted
}

/**
 * @param {number[]} numberList
 * @param {Array.<Array.<{value: number, marked: boolean}>>} board 
 * @returns number
 */
function calculateNumberOfDrawsToWin(numberList, board) {
    let numberOfDraws = 0
    const numberToIndexMap = new Map()
    for ([rowIndex, row] of board.entries()) {
        for ([columnIndex, boardEntry] of row.entries()) {
            numberToIndexMap.set(boardEntry.value, [rowIndex, columnIndex])
        }
    }
    for (const number of numberList) {
        numberOfDraws++
        if (numberToIndexMap.has(number)) {
            const [rowIndex, columnIndex] = numberToIndexMap.get(number)
            board[rowIndex][columnIndex].marked = true
            if (checkIfBingo(board, rowIndex, columnIndex)) {
                return numberOfDraws
            }
        }
    }
    return null
}

/**
 * @param {Array.<Array.<{value: number, marked: boolean}>>} board 
 * @returns number
 */
function calculateSumOfUnmarkedNumbers(board) {
    let result = 0
    for (const row of board) {
        for (const boardEntry of row) {
            if (!boardEntry.marked) {
                result += boardEntry.value
            }
        }
    }
    return result
}

/**
 * @param {number[]} numberList
 * @param {Array.<Array.<Array.<{value: number, marked: boolean}>>>} boardList
 * @returns {*}
 */
function calculateDrawsForEachBoard(numberList, boardList) {
    const resultList = []
    for (const board of boardList) {
        resultList.push({
            numberOfDraws: calculateNumberOfDrawsToWin(numberList, board),
            board
        })
    }
    return resultList
}

/**
 * @param {Array.<Array.<Array.<{value: number, marked: boolean}>>>} boardList
 * @param {number[]} numberList
 */
function runSolutionPuzzleOne(numberList, boardList) {
    const resultList = calculateDrawsForEachBoard(numberList, boardList)
    let winningBoard = null
    for (const item of resultList) {
        if (winningBoard === null || item.numberOfDraws < winningBoard.numberOfDraws) {
            winningBoard = item
        }
    }
    const score = calculateSumOfUnmarkedNumbers(winningBoard.board) * numberList[winningBoard.numberOfDraws - 1]
    console.log(`Solution to first puzzle: ${score}`)
}
// end::star1[]

// tag::star2[]

/**
 * @param {Array.<Array.<Array.<{value: number, marked: boolean}>>>} boardList
 * @param {number[]} numberList
 */
function runSolutionPuzzleTwo(numberList, boardList) {
    const resultList = calculateDrawsForEachBoard(numberList, boardList)
    let winningBoard = null
    for (const item of resultList) {
        if (winningBoard === null || item.numberOfDraws > winningBoard.numberOfDraws) {
            winningBoard = item
        }
    }
    const score = calculateSumOfUnmarkedNumbers(winningBoard.board) * numberList[winningBoard.numberOfDraws - 1]
    console.log(`Solution to second puzzle: ${score}`)
}
// end::star2[]

// tag::input[]
const rawInput = fs.readFileSync('input.txt').toString().split('\n\n')
const numberList = rawInput[0].split(',').map(v => parseInt(v, 10))
const boardList = []
for (const boardRawInput of rawInput.slice(1)) {
    const boardData = boardRawInput
        .split('\n')
        .map(rowRaw =>
            rowRaw
            .replace(/\s+/g, ' ')
            .trim()
            .split(' ')
            .map(v => {
                return { value: parseInt(v, 10), marked: false }
            })
        )
    boardList.push(boardData)
}
// end::input[]

runSolutionPuzzleOne(numberList, JSON.parse(JSON.stringify(boardList)))
runSolutionPuzzleTwo(numberList, JSON.parse(JSON.stringify(boardList)))