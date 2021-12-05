package io.uniqueck.aoc.day5

class Line {

    Coord start, end

    // converts 1,2 -> 3,2 to a line with start coordinate 1,2 and end coordinate 3,2
    static Line of(String stringRepresentation) {
        def (tempStart, tempEnd) = stringRepresentation.tokenize("->")
        return new Line(Coord.of(tempStart), Coord.of(tempEnd))
    }

    Line(Coord start, end) {
        this.start = start
        this.end = end
    }

    boolean isStartAndEndOnSameRowOrColumn() {
        return start.isOnSameRowOrColumn(end)
    }

    List<Coord> getListOfCoordsFromStartToEnd() {
        return start.createListOfCoordsToConnectTo(end)
    }



}
