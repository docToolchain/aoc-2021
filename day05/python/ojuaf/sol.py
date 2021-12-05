#!/usr/bin/env python3

import re

pattern = re.compile('(\d*),(\d*) -> (\d*),(\d*)')


def load_input():
    data = list()
    with open('input') as fd:
        for line in fd:
            match = pattern.search(line)
            data.append([(int(match.group(1)), int(match.group(2))), (int(match.group(3)), int(match.group(4)))])
    return data

def find_hor_and_vert(data, task):
    coords = list()
    kind = None
    for el in data:
        dist = None
        if el[0][0] == el[1][0]:
            dist = range(min(el[0][1], el[1][1]), max(el[0][1], el[1][1]) + 1)
            for i in dist:
                coords.append((el[0][0], i))
        if el[0][1] == el[1][1]:
            dist = range(min(el[0][0], el[1][0]), max(el[0][0], el[1][0]) + 1)
            for i in dist:
                coords.append((i, el[0][1]))
        if task == 2:
            if abs(el[0][0] - el[1][0]) == abs(el[0][1] - el[1][1]):
                a = None
                b = None
                if el[0][0] < el[1][0]:
                    a = range(el[0][0], el[1][0] + 1)
                else:
                    a = range(el[0][0], el[1][0] - 1, -1)
                if el[0][1] < el[1][1]:
                    b = range(el[0][1], el[1][1] + 1)
                else:
                    b = range(el[0][1], el[1][1]-1, -1)
                for i in zip(a, b):
                    coords.append(i)                    
    return coords


def calc_number(coords):
    # coords.sort()
    i = 0
    count = 0
    overlaps = dict()
    for coord in coords:
        if coord not in overlaps:
            overlaps[coord] = 1
        else:
            overlaps[coord] += 1
    for value in overlaps.values():
        if value > 1:
            count += 1
    return count



def part1():
    data = load_input()
    coords = find_hor_and_vert(data, 1)
    result = calc_number(coords)
    print("Part 1: ", result)
    return

def part2():
    # Second part
    data = load_input()
    coords = find_hor_and_vert(data, 2)
    result = calc_number(coords)
    print("Part 2: ", result)
    return

if __name__ == '__main__':
    part1()
    part2()
