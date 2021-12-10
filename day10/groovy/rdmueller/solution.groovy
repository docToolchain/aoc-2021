#!/usr/bin/env groovy
def input = new File("input.txt").readLines()
def stack = []
def scores = [')': 3, ']': 57,'}':1197,'>':25137]
def scores2 = [')': 1, ']': 2,'}':3,'>':4,'(': 1, '[': 2,'{':3,'<':4]
BigDecimal score = 0
def opening = [')':'(','}':'{',']':'[','>':'<']
input.each { line ->
    stack = []
    def lineOK = true
    line.each { item ->
        if (lineOK && item) {
            if ("({[<".contains(item)) {
                stack.push(item)
            } else {
                if (stack.isEmpty()) {
                    // no matching opening bracket
                    lineOK = false
                } else {
                    def pop = stack.pop()
                    if (pop != opening[item]) {
                        score += scores[item]
                        lineOK = false
                    }
                }
            }
            println stack
        }
    }
}
println score
score = 0
def solution = []
input.each { line ->
    stack = []
    def lineOK = true
    score = 0
    def fix = ""
    line.each { item ->
        if (lineOK==true && item) {
            if ("({[<".contains(item)) {
                stack.push(item)
            } else {
                if (stack.isEmpty()) {
                    // no matching opening bracket
                    lineOK = false
                } else {
                    def pop = stack.pop()
                    if (pop != opening[item]) {
                        lineOK = false
                    }
                }
            }
        }
    }
    if (lineOK==true) {
        println stack
        while (stack.size() > 0) {
            def pop = stack.pop()
            if (pop) {
                score *= 5
                print pop
                score += scores2[pop]
                fix += pop
            }
        }
        println()
        println line + ": " + fix + " - " + score
        solution.add(score)
    }

}
println solution.sort()
println solution.sort()[Math.round(solution.size()/2)-1]