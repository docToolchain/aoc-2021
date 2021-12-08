#!/usr/bin/env groovy
def numbers = []
def boards = []
def readInput = { filename ->
    def state = "nums"
    board = []
    new File(filename).text.eachLine { line ->
        switch (state) {
            case "nums":
                numbers = line.split(",").collect { it as Integer }
                state = "board"
                break
            case "board":
                if (line.trim()=="") {
                } else {

                    board += line.trim().split(" +").collect { if (it) {it as Integer} }
                    if (board.size()==25) {
                        boards.add(board)
                        board = []
                    }
                }
                break
        }
    }
}

hasWon = { board ->
    def won = false
    5.times { row ->
        def rowWon = true
        5.times { j ->
            if (board[row*5+j]!=null) {
                rowWon = false
            }
        }
        if (rowWon) {
            won = true
        }
    }
    5.times { col ->
        def colWon = true
        5.times { j ->
            if (board[col+j*5]!=null) {
                colWon = false
            }
        }
        if (colWon) {
            won = true
        }
    }
    return won
}

def sumOfBoard = { board ->
    def sum = 0
    board.each { num ->
        sum += num?:0
    }
    return sum
}
readInput ("input.txt")

numbers.each { num ->
    println "="*80
    println num
    boards.eachWithIndex { board, i ->
        if (board.indexOf(num)!=-1) {
            board[board.indexOf(num)] = null
        }
        println "board $i"
        5.times {
            println board[5*it..5*it+4].collect {(it?it.toString():".").padLeft(3)}.join(" ")
        }
        println ()
        if (hasWon(board)) {
            println("Part 1: $i")
            def sum = sumOfBoard(board)
            println (sum*num)
            throw new RuntimeException("done")
        }
    }

}
