package io.uniqueck.aoc.day5

import java.util.stream.IntStream

class Coord {

    int x, y

    static Coord of(String stringRepresentation) {
        def (tempX, tempY) = stringRepresentation.tokenize(",")
        new Coord(tempX as int, tempY as int)
    }

    Coord(int x, y) {
        this.x = x
        this.y = y
    }

    boolean isOnSameRowOrColumn(Coord target) {
        return x == target.x || y == target.y
    }


    List<Coord> createListOfCoordsToConnectTo(Coord target) {

        List<Coord> listOfCoords = []
        // diagonal
        int stepSizeX = target.x - x < 0 ? -1 : target.x - x == 0 ? 0 : 1
        int stepSizeY = target.y - y < 0 ? -1 : target.y - y == 0 ? 0 : 1

        int stepX = x
        int stepY = y

        while (!(stepX == target.x && stepY == target.y)) {
            listOfCoords << new Coord(stepX, stepY)
            stepX += stepSizeX
            stepY += stepSizeY
        }
        listOfCoords << target

        return listOfCoords
    }

    boolean equals(o) {
        if (this.is(o)) return true
        if (getClass() != o.class) return false

        Coord coord = (Coord) o

        if (x != coord.x) return false
        if (y != coord.y) return false

        return true
    }

    int hashCode() {
        int result
        result = x
        result = 31 * result + y
        return result
    }

    String toString() {
        return "(${x},${y})"
    }

}
