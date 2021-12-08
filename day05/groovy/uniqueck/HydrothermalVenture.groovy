package io.uniqueck.aoc.day5

class HydrothermalVenture {

    static void main(String[] args) {
        def lines = new File("src/main/resources/day5/input.txt").readLines()
        new HydrothermalVenture().process(lines)
    }

    void process(List<String> input) {
        def lines = input.collect {Line.of(it)}
        println "HydrothermalVenture only horizontal and vertical lines: ${countHowManyPointsAreOverlappingFromMoreThenOnLine(lines.findAll {it.startAndEndOnSameRowOrColumn})}"
        println "HydrothermalVenture all type of lines: ${countHowManyPointsAreOverlappingFromMoreThenOnLine(lines)}"
    }

    @SuppressWarnings('GrMethodMayBeStatic')
    int countHowManyPointsAreOverlappingFromMoreThenOnLine(List<Line> lines) {
        int[][] result = new int[999][999]
        lines.collect { it.listOfCoordsFromStartToEnd}.flatten().each { Coord it ->
            result[it.y][it.x]++
        }
        ((List<List<Integer>>)result).flatten().findAll{it > 1}.size()
    }
}
