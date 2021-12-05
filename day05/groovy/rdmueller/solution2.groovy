#!/usr/bin/env groovy

def readFile = { filename ->
    def lines = []
    new File(filename).text.eachLine { line ->
        def (start, end) = line.split(" -> ")
        lines << [
                start.split(",")
                        .collect { it as Integer},
                end.split(",")
                        .collect { it as Integer}]
    }
    return lines
}
def lines = []
lines = readFile("input.txt")
def field = [:]
lines.each { line ->
    def (start, end) = line
    if (start[0]==end[0]) {
        def x = start[0]
        (start[1]..end[1]).each { y ->
            if (!field["${x}-${y}"]) {field["${x}-${y}"] = 0}
            field["${x}-${y}"]++
        }
    }
    if (start[1]==end[1]) {
        def y = start[1]
        (start[0]..end[0]).each { x ->
            if (!field["${x}-${y}"]) {field["${x}-${y}"] = 0}
            field["${x}-${y}"]++
        }
    }
    if ((start[0]!=end[0])&&(start[1]!=end[1])) {
        def x = start[0]
        (start[1]..end[1]).each { y ->
            if (!field["${x}-${y}"]) {field["${x}-${y}"] = 0}
            field["${x}-${y}"]++
            if (start[0]<end[0]) {x++} else {x--}
        }
    }
}
def sum = 0
field.each { key, value ->
    if (value > 1) {
        sum++
    }
}
println sum