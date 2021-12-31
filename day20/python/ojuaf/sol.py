
#!/usr/bin/env python3

import re
import math as m
from collections import deque

pattern = re.compile("target area: x=(\d+)..(\d+), y=(-\d+)..(-\d+)")


def load_input():
    rule = ""
    data = set()
    j = 0
    first = True
    with open('input') as fd:
        for line in fd:
            line = line.strip()
            if line and first:
                rule += line
            elif not line:
                first = False
                # rule.replace('.', '0')
                # rule.replace('#', '1')
            else:
                for i, ch in enumerate(line):
                    if ch == '#':
                        data.add((i, j))
                j += 1
    return data, rule

def print_data(data):
    xs = [t[0] for t in data]
    ys = [t[1] for t in data]
    for i in range(min(ys)-2, max(ys)+3):
        line = ""
        for j in range(min(xs)-2, max(xs)+3):
            if (j,i) in data:
                line += '#'
            else:
                line += '.'
        print(line)

def get_number(data, pos):
    number = ""
    for i in range(-1, 2):
        for j in range(-1, 2):
            t = (pos[0]+j, pos[1]+i)
            if t in data:
                number += '1'
            else:
                number += '0'
    return int(number, 2)

def fill_border(step, y_min, y_max, x_min, x_max, data):
    if step%2 != 0:
        for i in range(y_min-2, y_max+3):
            for j in range(x_min-2, x_max+3):
                if i < y_min or i > y_max or j < x_min or j > x_max:
                    data.add((j,i))
    return


def part1():
    # First Part
    data, rule = load_input()
    steps = 50

    for step in range(steps):
        next_data = set()
        xs = [t[0] for t in data]
        x_min = min(xs)
        x_max = max(xs)
        ys = [t[1] for t in data]
        y_min = min(ys)
        y_max = max(ys)
        fill_border(step, y_min, y_max, x_min, x_max, data)

        for i in range(y_min-1, y_max+2):
            for j in range(x_min-1, x_max+2):
                number = get_number(data, (j, i))
                if rule[number] == '#':
                    next_data.add((j,i))

        data = next_data
        if step == 1:
            result = len(data)

    print("Part 1:", result)
    result = len(data)
    print("Part 2:", result)
    return


def part2():
    # Second part
    result = 0
    print("Part 2:", result)
    return


if __name__ == '__main__':
    part1()
    # part2()
