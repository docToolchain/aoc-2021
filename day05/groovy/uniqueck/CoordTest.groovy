package io.uniqueck.aoc.day5

import org.assertj.core.api.Assertions
import org.junit.jupiter.api.Test

class CoordTest {

    @Test
    void createListOfCoordsToConnectTo() {
        // same y
        Assertions.assertThat(new Coord(1, 2).createListOfCoordsToConnectTo(new Coord(2,2))).containsExactly(new Coord(1,2), new Coord(2,2))
        Assertions.assertThat(new Coord(2, 2).createListOfCoordsToConnectTo(new Coord(1,2))).containsExactly(new Coord(2,2), new Coord(1,2))
        // same x
        Assertions.assertThat(new Coord(2, 1).createListOfCoordsToConnectTo(new Coord(2,2))).containsExactly(new Coord(2,1), new Coord(2,2))
        Assertions.assertThat(new Coord(2, 2).createListOfCoordsToConnectTo(new Coord(2,1))).containsExactly(new Coord(2,2), new Coord(2,1))
        // diagnoal
        Assertions.assertThat(new Coord(1, 1).createListOfCoordsToConnectTo(new Coord(2,2))).containsExactly(new Coord(1,1), new Coord(2,2))
        Assertions.assertThat(new Coord(2, 2).createListOfCoordsToConnectTo(new Coord(1,1))).containsExactly(new Coord(2,2), new Coord(1,1))
        Assertions.assertThat(new Coord(3, 3).createListOfCoordsToConnectTo(new Coord(1,1))).containsExactly(new Coord(3,3), new Coord(2,2), new Coord(1,1))
    }

}