def find_winning_board(boards):
    n_rows = len(boards[0])
    n_cols = len(boards[0][0])
    winners = list()
    for (index, board) in enumerate(boards):
        for row in board:
            if len([1 for n in row if type(n) == int]) == 0:
                # all elements in row are "marked" as string
                winners.append(index)

        for c_cols in range(n_cols):
            col = [row[c_cols] for row in board]
            if len([1 for n in col if type(n) == int]) == 0:
                # all elements in column are "marked" as string
                winners.append(index)
    return winners


def star1(puzzle_in):
    (numbers, boards) = puzzle_in
    n_boards = len(boards)
    n_rows = len(boards[0])
    n_cols = len(boards[0][0])

    for number in numbers:
        c_boards = 0
        while c_boards < n_boards:
            c_rows = 0
            while c_rows < n_rows:
                c_cols = 0
                while c_cols < n_cols:
                    if boards[c_boards][c_rows][c_cols] == number:
                        boards[c_boards][c_rows][c_cols] = str(number)
                    c_cols += 1
                c_rows += 1
            c_boards += 1

        winning_board_ix = find_winning_board(boards)
        if len(winning_board_ix) > 0:
            winning_board = boards[winning_board_ix[0]]
            score = 0
            for row in winning_board:
                score += sum([n for n in row if type(n) == int])
            return number * score
    return None


def star2(puzzle_in):
    (numbers, boards) = puzzle_in
    n_rows = len(boards[0])
    n_cols = len(boards[0][0])

    last_score = 4
    for number in numbers:
        n_boards = len(boards)
        c_boards = 0
        while c_boards < n_boards:
            c_rows = 0
            while c_rows < n_rows:
                c_cols = 0
                while c_cols < n_cols:
                    if boards[c_boards][c_rows][c_cols] == number:
                        boards[c_boards][c_rows][c_cols] = str(number)
                    c_cols += 1
                c_rows += 1
            c_boards += 1

        winning_board_ix = find_winning_board(boards)
        if len(winning_board_ix) > 0:
            winning_board = boards[winning_board_ix[-1]]
            score = 0
            for row in winning_board:
                score += sum([n for n in row if type(n) == int])
            last_score = number * score
            winning_board_ix.reverse()
            for ix in winning_board_ix:
                del boards[ix]
        if len(boards) == 0:
            break
    return last_score


def read_input(filepath):
    """Create input and parse it.

    Returns:
     (numbers, boards) tuple of bingo numbers and bingo boards
    """
    numbers = list()
    boards = list()
    with open(filepath, "r") as f:
        line_count = 0
        board = list()
        for line in f:
            line = line.strip()
            if line_count == 0:
                nums = line.split(",")
                numbers = [int(n) for n in nums]
            elif (line_count - 1) % 6 == 0:
                if line_count > 2:
                    boards.append(board)
                board = list()
            else:
                nums = line.split(" ")
                board.append([int(n) for n in nums if len(n)])
            line_count += 1
        boards.append(board)
    return (numbers, boards)


if __name__ == "__main__":
    puzzle_in = read_input("input.txt")
    print(f"Result for first star: {star1(puzzle_in)}")
    print(f"Result for first star: {star2(puzzle_in)}")
