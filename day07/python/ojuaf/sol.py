#!/usr/bin/env python3

import re

pattern = re.compile('(\d*),(\d*) -> (\d*),(\d*)')


def load_input():
    data = list()
    with open('input') as fd:
        data =[int(i) for i in fd.read().strip().split(',')]
    return data


def part1():
    data = load_input()
    min_pos = min(data)
    max_pos = max(data)
    fuels_list = list()
    for pos in range(min_pos, max_pos + 1):
        fuels = 0
        for crab_pos in data:
            fuels += abs(crab_pos - pos)
        fuels_list.append(fuels)
    least_fuel = min(fuels_list)
    result = least_fuel
    print("Part 1: ", least_fuel)
    return

def part2():
    # Second part
    data = load_input()
    min_pos = min(data)
    max_pos = max(data)
    fuels_list = list()
    for pos in range(min_pos, max_pos + 1):
        fuels = 0
        for crab_pos in data:
            dist = abs(crab_pos - pos)
            fuels += dist*(dist+1)/2
        fuels_list.append(fuels)
    least_fuel = min(fuels_list)
    result = least_fuel
    print("Part 2: ", result)
    return

if __name__ == '__main__':
    part1()
    part2()
