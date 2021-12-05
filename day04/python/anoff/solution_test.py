from solution import *


def test_parse_input():
    puzzle_in = read_input("input_test.txt")
    (numbers, boards) = puzzle_in
    assert len(numbers) > 20
    assert type(numbers[0]) == int
    assert len(boards) == 3
    assert len(boards[0]) == 5
    assert len(boards[0][0]) == 5
    assert type(boards[0][0][0]) == int


def test_play_board():
    puzzle_in = read_input("input_test.txt")
    (numbers, boards) = puzzle_in
    board = boards[0]
    play_board(board, 7)
    assert board[2][4] == "7"
    play_board(board, 11)
    assert board[2][4] == "7"
    assert board[0][3] == "11"


def test_score_board():
    puzzle_in = read_input("input_test.txt")
    (numbers, boards) = puzzle_in
    board = boards[2]
    for n in [7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24]:
        play_board(board, n)
    assert score_board(board) == 188


def test_is_complete_board():
    puzzle_in = read_input("input_test.txt")
    (numbers, boards) = puzzle_in
    board = boards[2]
    for n in [7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21]:
        play_board(board, n)
    assert is_complete_board(board) == False
    play_board(board, 24)
    assert is_complete_board(board) == True


def test_star1():
    puzzle_in = read_input("input_test.txt")
    score = star1(puzzle_in)
    assert score == 4512


def test_star2():
    puzzle_in = read_input("input_test.txt")
    score = star2(puzzle_in)
    assert score == 1924
