from solution import *


def test_parse_input():
    puzzle_in = read_input("input_test.txt")
    assert len(puzzle_in) == 10
    e1 = puzzle_in[0]
    assert len(e1) == 2
    assert e1[0] == P2D(0, 9)


def test_remove_diagonal_entries():
    puzzle_in = read_input("input_test.txt")
    remove_diagonal_entries(puzzle_in)
    assert len(puzzle_in) == 6, "Should modify list in place"


def test_p2d_points_to():
    p1 = P2D(0, 0)
    points = p1.points_to(P2D(3, 0))
    assert len(points) == 4
    assert p1 in points
    assert P2D(3, 0) in points
    assert len(p1.points_to(p1)) == 1, "Should return single point when going to itself"


def test_create_map():
    m = create_map(5, 2)
    assert len(m) == 3
    assert len(m[0]) == 6
    assert m[1][2] == 0


def test_star1():
    puzzle_in = read_input("input_test.txt")
    score = star1(puzzle_in)
    assert score == 5


def test_get_board_size():
    puzzle_in = read_input("input_test.txt")
    puzzle_in.append([P2D(4, 2), P2D(4, 11)])
    p = get_board_size(puzzle_in)
    assert p.x == 9
    assert p.y == 11


# part 2
def test_p2d_points_to_diagonal():
    p1 = P2D(1, 1)
    points = p1.points_to(P2D(3, 3))
    assert len(points) == 3
    assert p1 in points
    assert P2D(2, 2) in points
    assert P2D(3, 3) in points

    points = P2D(9, 7).points_to(P2D(7, 9))
    assert len(points) == 3
    assert P2D(8, 8) in points


def test_p2d_points_to_diagonal_error_case():
    p1 = P2D(8, 0)
    points = p1.points_to(P2D(0, 8))
    assert P2D(7, 1) in points
    assert P2D(6, 2) in points
    assert P2D(5, 3) in points
    assert P2D(4, 4) in points
    assert P2D(3, 5) in points
    assert P2D(2, 6) in points
    assert P2D(1, 7) in points
    assert P2D(0, 8) in points
    assert len(points) == 9


def test_star2():
    puzzle_in = read_input("input_test.txt")
    score = star2(puzzle_in)
    assert score == 12
