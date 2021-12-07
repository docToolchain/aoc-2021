#!/usr/bin/env python3

def load_input():
    bins = list()
    with open('input') as fd:
        bins = [0 for _ in range(9)]
        values = [int(i) for i in fd.read().strip().split(',')]
        for value in values:
            bins[value] += 1
    return bins


def step(bins):
    temp = bins[0]
    for i in range(1, 9):
        bins[i-1]=bins[i]
    bins[6] += temp
    bins[8] = temp
    return bins


def part1():
    bins = load_input()
    days = 80
    for _ in range(days):
        bins = step(bins)
    result = sum(bins)

    print("Part 1:", result)
    return


def part2():
    bins = load_input()
    days = 256
    for _ in range(days):
        bins = step(bins)
    result = sum(bins)

    print("Part 2: ", result)
    return

if __name__ == '__main__':
    part1()
    part2()
