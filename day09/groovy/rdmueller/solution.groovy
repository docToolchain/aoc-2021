#!/usr/bin/env groovy
def map = new File("input.txt").readLines()
        .collect { it.split("").collect { height -> height as byte} }

def width = map[0].size()
def height = map.size()
def sum = 0
def lowPoints = []
map.eachWithIndex { row, y ->
    row.eachWithIndex { column, x ->
        def current = map[y][x]
        def surroundings = [10,10,10,10]
        if (y>0) {
            surroundings[0] = map[y-1][x]
        } else {
            surroundings[0] = null
        }
        if (y<height-1) {
            surroundings[2] = map[y+1][x]
        } else {
            surroundings[2] = null
        }
        if (x>0) {
            surroundings[4] = map[y][x-1]
        } else {
            surroundings[4] = null
        }
        if (x<width-1) {
            surroundings[1] = map[y][x+1]
        } else {
            surroundings[1] = null
        }
        def lowPoint = true
        surroundings.each {
            if (it != null && it <= current) {
                lowPoint = false
            }
        }
        if (lowPoint) {
            sum += 1+current
            lowPoints.add([y,x])
        }
    }
}
// 1716 too high
println "=> "+sum

class Helper {
    def fillBasin(map, y, x) {
        def width = map[0].size()
        def height = map.size()
        def size = 1
        map[y][x]=null
        if (y>0) {
            if (map[y - 1][x] != null && map[y - 1][x] != 9) {
                size += fillBasin(map, y - 1, x)
            }
        }
        if (y<height-1) {
            if (map[y + 1][x] != null && map[y + 1][x] != 9) {
                size += fillBasin(map, y + 1, x)
            }
        }
        if (x>0) {
            if (map[y][x - 1] != null && map[y][x - 1] != 9) {
                size += fillBasin(map, y, x - 1)
            }
        }
        if (x<width-1) {
            if (map[y][x + 1] != null && map[y][x + 1] != 9) {
                size += fillBasin(map, y, x + 1)
            }
        }
        return size
    }
}
def helper = new Helper()
def result = []
lowPoints.each { point ->
    result.add(helper.fillBasin(map, point[0], point[1]))
}
println "=> "+result.sort()[-1]*result.sort()[-2]*result.sort()[-3]