
#!/usr/bin/env python3

import re

pattern = re.compile('(\d*),(\d*) -> (\d*),(\d*)')


def load_input():
    data = list()
    with open('input') as fd:
        for i, line in enumerate(fd):
            data.append(line)
    return data

def part1():
    # First part
    data = load_input()
    # data = ["{([(<{}[<>[]}>{[]{[(<()>"]
    pairs = {
        '(': ')',
        '[': ']',
        '<': '>',
        '{': '}'
    }
    values = {
        ')': 3,
        ']': 57,
        '}': 1197,
        '>': 25137
    }

    errors = list()
    remainings = list()
    for line in data:
        expected = list()
        opened = list()
        for i in line:
            if i in pairs:
                opened.append(i)
                expected.append(pairs[i])
            if i in pairs.values():
                expect = expected.pop()
                opened.pop()
                if i != expect:
                    errors.append(i)
                    break
        else:
            expected.reverse()
            remainings.append(expected)

    result = 0
    for error in errors:
        result += values[error]
    print("Part 1:", result)

    return remainings


def part2():
    # Second part
    values = {
        ')': 1,
        ']': 2,
        '}': 3,
        '>': 4
    }

    remainings = part1()
    results = list()

    for remaining in remainings:
        temp = 0
        for i in remaining:
            temp = 5*temp + values[i]
        else:
            results.append(temp)
    results.sort()
    print("Part 2:", results[len(results)//2])
    return

if __name__ == '__main__':
    # part1()
    part2()
