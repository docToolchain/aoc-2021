const fs = require('fs')

function solution() {
    const input = fs.readFileSync('input.txt').toString()
    console.log(input)
}

solution()