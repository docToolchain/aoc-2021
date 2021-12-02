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


def test_sub():
    s = Submarine()
    cmd = read_input("input_test.txt")
    for c in cmd:
        s.parse_command(c)
    assert s.x == 15
    assert s.y == 60


def test_star1():
    cmd = read_input("input_test.txt")
    score = star2(cmd)
    assert score == 900
