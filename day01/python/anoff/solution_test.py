from solution import calc_deltas, read_input, star1, star2, windowed_sum


def test_calc_deltas():
    assert calc_deltas([0, 4, 2, 6]) == [4, -2, 4]


def test_star1():
    numbers = read_input("input_test.txt")
    assert star1(numbers) == 7


def test_windowed_sum():
    assert windowed_sum([0, 1, 0, 2, 5, 0, 3]) == [1, 3, 7, 7, 8]
    numbers = read_input("input_test.txt")
    assert windowed_sum(numbers) == [607, 618, 618, 617, 647, 716, 769, 792]


def test_star2():
    numbers = read_input("input_test.txt")
    assert star2(numbers) == 5
