
#!/usr/bin/env python3

import re
import math as m
from collections import defaultdict
import itertools as it

pattern = re.compile("target area: x=(\d+)..(\d+), y=(-\d+)..(-\d+)")


def load_input():
    data = list()
    with open('input') as fd:
        for line in fd:
            line = line.strip().split()
            data.append(line)
    return data


def prepare(data):
    kn = list()
    ln = list()
    for i, line in enumerate(data):
        j = i%18
        if j == 5:
            kn.append(int(line[2]))
        elif j == 15:
            ln.append(int(line[2]))
            
    return kn, ln

def largest_number(kn, ln):
    zn = [0]*15
    number = [9]*14
    count = 0
    while count < len(number):
        if kn[count] > 9:
            zn[count+1] = zn[count]*26 + number[count] + ln[count]
            count += 1
        else:
            temp = zn[count]%26 + kn[count]
            if 0 < temp < 10:
                number[count] = temp
                zn[count+1] = zn[count]//26
                count += 1
            else:
                while True:
                    count -= 1
                    if kn[count] > 9:
                        number[count] -= 1
                        if number[count] == 0:
                            number[count] = 9
                        else:
                            break

    result = ""
    for i in number:
        result += str(i)

    return int(result)

def smallest_number(kn, ln):
    zn = [0]*15
    number = [1]*14
    count = 0
    while count < len(number):
        if kn[count] > 9:
            zn[count+1] = zn[count]*26 + number[count] + ln[count]
            count += 1
        else:
            temp = zn[count]%26 + kn[count]
            if 0 < temp < 10:
                number[count] = temp
                zn[count+1] = zn[count]//26
                count += 1
            else:
                while True:
                    count -= 1
                    if kn[count] > 9:
                        number[count] += 1
                        if number[count] == 10:
                            number[count] = 1
                        else:
                            break

    result = ""
    for i in number:
        result += str(i)

    return int(result)

def part1():
    # First Part
    data = load_input()

    kn, ln = prepare(data)
    result = largest_number(kn, ln)
    print("Part 1:", result)
    
    return


def part2():
    # Second part
    data = load_input()

    kn, ln = prepare(data)
    result = smallest_number(kn, ln)
    print("Part 2:", result)
    return


if __name__ == '__main__':
    part1()
    part2()
