package io.uniqueck.aoc.day5


import org.junit.jupiter.api.BeforeEach
import org.junit.jupiter.api.Test

import static org.junit.jupiter.api.Assertions.assertEquals

class HydrothermalVentureTest {

    List<String> input = []

    @BeforeEach
    void before() {
        input = new File("src/test/resources/day5/example.txt").readLines()
    }


    @Test
    void countHowManyPointsAreOverlappingFromMoreThenOnLine() {
        def underTest = new HydrothermalVenture()

        assertEquals(5, underTest.countHowManyPointsAreOverlappingFromMoreThenOnLine(input.collect{Line.of(it)}.findAll{it.startAndEndOnSameRowOrColumn}))
        assertEquals(12, underTest.countHowManyPointsAreOverlappingFromMoreThenOnLine(input.collect{Line.of(it)}))
    }


}
