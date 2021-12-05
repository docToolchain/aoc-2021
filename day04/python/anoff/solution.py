from typing import List, Tuple

Board = List[List[int]]


def read_input(filepath) -> Tuple[List[int], List[Board]]:
    """Create input and parse it.

    Returns:
     (numbers, boards) tuple of bingo numbers and bingo boards
    """
    with open(filepath, "r") as f:
        lines = f.readlines()
    numbers = [int(n) for n in lines.pop(0).split(",")]

    lines_per_board = 6
    n_boards = int(len(lines) / lines_per_board)
    boards = list()
    for n in range(n_boards):
        board = lines[n * lines_per_board + 1 : (n + 1) * lines_per_board]
        for row_ix in range(len(board)):
            row = [s.strip() for s in board[row_ix].split(" ")]
            row = [int(s) for s in row if len(s)]
            board[row_ix] = row
        boards.append(board)
    return (numbers, boards)


def play_board(board: Board, number: int) -> Board:
    """Play a single number on a board. Modifies board."""
    n_rows = len(board)
    n_cols = len(board[0])

    for c_row in range(n_rows):
        for c_col in range(n_cols):
            if board[c_row][c_col] == number:
                board[c_row][c_col] = str(number)
    return board


def score_board(board: Board) -> int:
    """Calculate board score."""
    score = 0
    for row in board:
        row_sum = sum([n for n in row if type(n) == int])
        score += row_sum
    return score


def is_complete_board(board: Board) -> bool:
    """Check if a board is compelete."""
    for row in board:
        int_elements = [n for n in row if type(n) == int]
        if len(int_elements) == 0:
            return True
    for col_ix in range(len(board[0])):
        col = [row[col_ix] for row in board]
        int_elements = [n for n in col if type(n) == int]
        if len(int_elements) == 0:
            return True
    return False


# tag::star1[]
def star1(puzzle_in):
    (numbers, boards) = puzzle_in
    for number in numbers:
        for board in boards:
            play_board(board, number)
            if is_complete_board(board):
                return score_board(board) * number
    return None


# end::star1[]

# tag::star2[]
def star2(puzzle_in):
    (numbers, boards) = puzzle_in
    last_score = 0
    for number in numbers:
        for board in boards:
            play_board(board, number)
        # loop twice because otherwise removing leads to broken loop
        for board in boards:
            if is_complete_board(board):
                last_score = score_board(board) * number
                boards.remove(board)
    return last_score


# end::star2[]

if __name__ == "__main__":
    puzzle_in = read_input("input.txt")
    print(f"Result for first star: {star1(puzzle_in)}")
    puzzle_in = read_input("input.txt")
    print(f"Result for first star: {star2(puzzle_in)}")
