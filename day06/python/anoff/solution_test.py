from solution import *


def test_parse_input():
    puzzle_in = read_input("input_test.txt")
    assert len(puzzle_in) == 5
    e1 = puzzle_in[0]
    assert e1 == 3


def test_sunrise():
    puzzle_in = read_input("input_test.txt")
    sunrise(puzzle_in)
    assert puzzle_in == [2, 3, 2, 0, 1]
    sunrise(puzzle_in)
    assert puzzle_in == [1, 2, 1, 6, 0, 8]


def test_star1():
    puzzle_in = read_input("input_test.txt")
    score = star1(puzzle_in)
    assert score == 5934


def test_fish2dict():
    puzzle_in = read_input("input_test.txt")
    d = fish2dict(puzzle_in)
    assert d[3] == 2
    assert d[5] == 0


def test_sunset():
    puzzle_in = read_input("input_test.txt")
    fish = fish2dict(puzzle_in)
    sunset(fish)
    assert fish[0] == 1
    assert fish[2] == 2
    sunset(fish)
    assert fish[0] == 1
    assert fish[8] == 1


def test_star2():
    puzzle_in = read_input("input_test.txt")
    score = star2(puzzle_in)
    assert score == 26984457539
