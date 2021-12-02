from solution import *


def test_input_parser():
    cmd = read_input("input_test.txt")
    assert cmd[0] == ("x", 5)
    assert cmd[1] == ("y", 5)
    assert cmd[3] == ("y", -3)


def test_move_sub():
    cmd = read_input("input_test.txt")
    pos = (0, 0)
    for c in cmd:
        pos = move_sub(c, pos)
    assert pos == (15, 10)


def test_star1():
    cmd = read_input("input_test.txt")
    score = star1(cmd)
    assert score == 150
