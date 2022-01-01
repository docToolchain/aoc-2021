
#!/usr/bin/env python3

import re
import math as m
from collections import defaultdict

pattern = re.compile("Player (\d) starting position: (\d+)")


def load_input():
    data = list()
    with open('input') as fd:
        for i, line in enumerate(fd):
            match = pattern.search(line)
            data.append(int(match.group(2))-1)
    return data


def part1():
    # First Part
    data = load_input()
    positions = data
    board = list(range(1, 11))
    scores = [0, 0]
    dice = 1
    rolls = 0
    values = 0
    run = True
    loser = None

    while run:
        for i in range(2):
            values = 0
            for j in range(3):
                values += dice
                rolls += 1
                dice += 1
                if dice > 100:
                    dice = 1
            positions[i] = (positions[i] + values)%10
            scores[i] += board[positions[i]]
            if scores[i] >= 1000:
                run = False
                loser = 0 if i == 1 else 1
                break
    
    result = rolls*scores[loser]
    print("Part 1:", result)
    return


def part2():
    # Second part
    data = load_input()
    start = data
    board = list(range(1, 11))
    
    a = defaultdict(lambda: 0)
    for i in range(1,4):
        for j in range(1,4):
            for k in range(1,4):
                a[i+j+k] += 1
    
    positions = [[defaultdict(lambda: 0) for _ in range(10)], [defaultdict(lambda: 0) for _ in range(10)]]
    positions[0][start[0]] = {0: 1}
    positions[1][start[1]] = {0: 1}
    
    finished = [defaultdict(lambda: 0), defaultdict(lambda: 0)]
    round_variants = [defaultdict(lambda: 0), defaultdict(lambda: 0)]
    for player in range(2):
        rounds = 0
        while True:
            rounds += 1
            stop = True
            next_positions = [defaultdict(lambda: 0) for _ in range(10)]
            for pos, scores in enumerate(positions[player]):
                for score, number in scores.items():
                    for i in range(3, 10):
                        next_pos = (i + pos) % 10
                        next_score = score + board[next_pos]
                        next_number = number*a[i]
                        if next_score < 21:
                            next_positions[next_pos][next_score] += next_number
                            round_variants[player][rounds] += next_number
                            stop = False
                        else:
                            finished[player][rounds] += next_number
            positions[player] = next_positions
            if stop:
                break
    
    wins_p1 = 0
    for round_p1, number_p1 in finished[0].items():
        wins_p1 += number_p1*round_variants[1][round_p1-1]

    wins_p2 = 0
    for round_p2, number_p2 in finished[1].items():
        wins_p2 += number_p2*round_variants[0][round_p2]

    result = wins_p1 if wins_p1 > wins_p2 else wins_p2

    print("Part 2:", result)
    return


if __name__ == '__main__':
    part1()
    part2()
