const fs = require('fs')

// tag::star1[]
function runSolutionPuzzleOne(commandsList) {
    let horizontal = 0
    let depth = 0
    for (const { command, value }
        of commandsList) {
        if (command === 'forward') {
            horizontal += value
        } else if (command === 'down') {
            depth += value
        } else if (command === 'up') {
            depth -= value
        }
    }
    console.log(`Solution to first puzzle: ${horizontal * depth}`)
}
// end::star1[]

// tag::star2[]
function runSolutionPuzzleTwo(commandsList) {
    let horizontal = 0
    let depth = 0
    let aim = 0
    for (const { command, value }
        of commandsList) {
        if (command === 'forward') {
            horizontal += value
            depth += aim * value
        } else if (command === 'down') {
            aim += value
        } else if (command === 'up') {
            aim -= value
        }
    }
    console.log(`Solution to second puzzle: ${horizontal * depth}`)
}
// end::star2[]

// tag::input[]
function getCommands(input) {
    const regex = /(forward|down|up) (\d+)/
    const commandsList = []
    for (const command of input) {
        const match = regex.exec(command)
        commandsList.push({
            command: match[1],
            value: parseInt(match[2], 10)
        })
    }
    return commandsList
}

const input = fs.readFileSync('input.txt').toString().split("\n")
const commandsList = getCommands(input)
    // end::input[]

runSolutionPuzzleOne(commandsList)
runSolutionPuzzleTwo(commandsList)