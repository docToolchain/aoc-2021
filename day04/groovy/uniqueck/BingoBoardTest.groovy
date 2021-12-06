package io.uniqueck.aoc.day4

import org.junit.jupiter.api.Test
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.Arguments
import org.junit.jupiter.params.provider.MethodSource

import java.util.stream.IntStream
import java.util.stream.Stream

import static org.junit.jupiter.api.Assertions.assertEquals
import static org.junit.jupiter.api.Assertions.assertFalse
import static org.junit.jupiter.api.Assertions.assertTrue

class BingoBoardTest {

    @Test
    void sumOfAllUnmarkedNumbers() {
        def board = BingoBoard.init(" 1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25")
        assertEquals(325, board.sumOfAllUnmarkedNumbers())
        board.markNumber(1)
        assertEquals(324, board.sumOfAllUnmarkedNumbers())
    }

    static Stream<Arguments> isWins() {
        return Stream.of(
                Arguments.of(BingoBoard.init("1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25"), [1, 2, 3, 4, 5] as int[], true),
                Arguments.of(BingoBoard.init("1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25"), [1, 6, 11, 16, 21] as int[], true),
                Arguments.of(BingoBoard.init("1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25"), [1, 5, 11, 16, 21] as int[], false),
        )
    }

    @ParameterizedTest
    @MethodSource
    void isWins(BingoBoard board, int[] markNumbers, boolean wins) {
        assertFalse(board.wins)
        markNumbers.each { board.markNumber(it) }
        assertEquals(wins, board.wins)
    }

    @Test
    void markNumber() {
        def board = BingoBoard.init("1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25")
        IntStream.range(1, 5).each { assertFalse(board.markNumber(it)) }
        assertTrue(board.markNumber(5))
    }


}
