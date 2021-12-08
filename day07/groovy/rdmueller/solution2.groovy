#!/usr/bin/env groovy
def input = new File("input.txt").text.split(",").collect{ it as Integer }
def max = input.max()
def min = input.min()
def dist = [:]
class Fuel {
    Integer consumption (Integer dist) {
        if (dist <= 1) {
            return dist
        } else {
            return dist + consumption(dist - 1)
        }
    }
}
def fuel = new Fuel()
(min..max).each{ pos ->
    if (dist[pos] == null) {
        dist[pos] = 0
    }
    input.each { submarine ->
        dist[pos] += fuel.consumption (Math.abs(submarine - pos))
    }
}
println dist.values().min()

