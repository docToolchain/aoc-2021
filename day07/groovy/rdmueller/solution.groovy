#!/usr/bin/env groovy
def input = new File("input.txt").text.split(",").collect{ it as Integer }
def max = input.max()
def min = input.min()
def dist = [:]
(min..max).each{ pos ->
    if (dist[pos] == null) {
        dist[pos] = 0
    }
    input.each { submarine ->
        dist[pos] += Math.abs(submarine - pos)
    }
}
println dist.values().min()