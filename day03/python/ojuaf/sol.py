#!/usr/bin/env python3

def load_input():
    data = list()
    with open('input') as fd:
        for line in fd:
            value = line.strip()
            data.append(value)
    return data


def find_value(data, compare):
    length = len(data[0])
    bins = data
    for i in range(length):
        least = [value for value in bins if value[i] == '0']
        most = [value for value in bins if value[i] == '1']
        if compare(len(most), len(least)):
            bins = most
        else:
            bins = least
        if len(bins) == 1:
            break
    return int(bins[0], 2)


def part1():
    data = load_input()
    # First part
    gamma = str()
    epsilon = str()
    total = len(data)
    length = len(data[0])
    for i in range(length):
        count = 0
        for value in data:
            count += int(value[i])
        if count >= total/2:
            gamma += '1'
            epsilon += '0'
        else:
            gamma += '0'
            epsilon += '1'
    result = int(gamma, 2) * int(epsilon, 2)
    print("Part 1: ", result)
    return


def part2():
    # Second part
    data = load_input()
    oxygen = find_value(data, lambda x, y: x >= y)    
    co2 = find_value(data, lambda x, y: x < y)
    result = oxygen * co2
    print("Part 2: ", result)
    return

if __name__ == '__main__':
    part1()
    part2()
