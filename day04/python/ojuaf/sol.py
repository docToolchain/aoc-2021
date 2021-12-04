#!/usr/bin/env python3


def load_input():
    numbers = list()
    boards = list()
    board = list()
    with open('input') as fd:
        for i, line in enumerate(fd):
            if i == 0:
                numbers = list(map(lambda x: int(x), line.strip().split(',')))
            elif (i - 1)%6 != 0:
                board.append(list(map(lambda x: int(x), line.strip().split())))
            else:
                if board:
                    boards.append(board)
                board = list()
        else:
            boards.append(board)
    return numbers, boards

def find_pos(number, board):
    for i in range(5):
        for j in range(5):
            if board[i][j] == number:
                pos = (i, j)
                return pos
    return None

def check_bingo(matches):
    for i in range(5):
        if len([match for match in matches if match[0] == i]) == 5:
            return True
        if len([match for match in matches if match[1] == i]) == 5:
            return True
    return False

def calc_result(board, matches, number):
    unmarked = 0
    for i in range(5):
        for j in range(5):
            if (i, j) not in matches:
                unmarked += board[i][j]
    return number*unmarked

def part1():
    numbers, boards = load_input()
    positions = [[] for _ in range(len(boards))]
    result = None
    for number in numbers:
        for i, board in enumerate(boards):
            pos = find_pos(number, board)
            if pos is not None:
                positions[i].append(pos)
            if check_bingo(positions[i]):
                result = calc_result(board, positions[i], number)
                break
        if result is not None:
            break

    print("Part 1: ", result)
    return

def part2():
    # Second part
    numbers, boards = load_input()
    positions = [[] for _ in range(len(boards))]
    winners = list()
    result = None
    for number in numbers:
        for i, board in enumerate(boards):
            if i not in winners:
                pos = find_pos(number, board)
                if pos is not None:
                    positions[i].append(pos)
                if check_bingo(positions[i]):
                    winners.append(i)
                    if len(winners) == len(boards):
                        result = calc_result(board, positions[i], number)
                        break
        if result is not None:
            break
    print("Part 2: ", result)
    return

if __name__ == '__main__':
    part1()
    part2()
