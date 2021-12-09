#!/usr/bin/env groovy
def input = new File("input.txt").readLines()
        .collect { it.split(" [|] ") }
        .collect { it.collect{ item -> item.split(" ")} }

def sum = 0
input.each { line ->
    println line

    def digits = line[1]
    digits.each { digit ->
        //println pattern
        if (digit.size() in [2,3,4,7]) {
            sum ++
        }
    }
}
println sum

// 7 is contained in 0, 3, 8, 9
// 6 segements is 6, 9
def possibleNums = [0:[],1:[],2:[1],3:[7],4:[4],5:[2,3,5],6:[0,6,9],7:[8]]
def segments = [:]
sum = 0
input.each { line ->
    segments = [:]
    def patterns = line[0].collect{ it.split("").sort().join()}
    def digits = line[1].collect{ it.split("").sort().join()}
    println patterns.toString()+" | "+digits
    patterns.each { pattern ->
        if (pattern.size() in [2,3,4,7]) {
            if (pattern.size()==2) {
                segments[1] = pattern
            }
            if (pattern.size()==3) {
                segments[7] = pattern
            }
            if (pattern.size()==4) {
                segments[4] = pattern
            }
            if (pattern.size()==7) {
                segments[8] = pattern
            }
        }
    }
    patterns.each { pattern ->
        if (pattern.size() in [5,6]) {
            if (pattern.size()==5) {
                //must be 2, 3 or 5
                if (pattern.contains(segments[1][0]) && pattern.contains(segments[1][1])) {
                    segments[3] = pattern
                }
            }
            if (pattern.size()==6) {
                //must be 0, 6 or 9
                if ((pattern - segments[4][0] - segments[4][1] - segments[4][2] - segments[4][3]).size()==2) {
                    segments[9] = pattern
                } else if ((pattern-segments[1][0]-segments[1][1]).size()==4){
                    segments[0] = pattern
                } else {
                    segments[6] = pattern
                }
            }
        }
    }
    def segmentC = ""
    if (segments[6].contains(segments[1][0])) {
        segmentC = segments[1][1]
    } else {
        segmentC = segments[1][0]
    }
    patterns.each { pattern ->
        if (pattern.size()==5) {
            //must be 2, 3 or 5
            if (pattern.contains(segments[1][0]) && pattern.contains(segments[1][1])) {
                segments[3] = pattern
            } else if (pattern.contains(segmentC)) {
                segments[2] = pattern
            } else {
                segments[5] = pattern
            }
        }
    }
    def number = 0
    digits.each { digit ->
        num = -1
        segments.each { key, value ->
            if (digit==value) {
                num = key
            }
        }
        if (num==-1) {
            print "*"
        } else {
            number *= 10
            number += num
        }
    }
    sum += number
    println()
    println segments
    println ()
}
println sum
