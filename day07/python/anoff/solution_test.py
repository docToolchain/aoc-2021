from solution import *


def test_parse_input():
    puzzle_in = read_input("input_test.txt")
    assert puzzle_in[1] == 2
    assert puzzle_in[16] == 1


def test_fuel_cost():
    d = read_input("input_test.txt")
    assert fuel_cost(d, 1) == 41
    assert fuel_cost(d, 10) == 71
    assert fuel_cost(d, 3) == 39


def test_star1():
    puzzle_in = read_input("input_test.txt")
    score = star1(puzzle_in)
    assert score == 37


def test_fuel_cost2():
    d = read_input("input_test.txt")
    assert fuel_cost2(d, 2) == 206
    assert fuel_cost2(d, 5) == 168


def test_star2():
    puzzle_in = read_input("input_test.txt")
    score = star2(puzzle_in)
    assert score == 168
