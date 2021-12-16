
#!/usr/bin/env python3

import re
from collections import deque

pattern = re.compile("(\w\w) -> (\w)")


def load_input():
    start = ""
    rules = dict()
    with open('input') as fd:
        first = True
        for i, line in enumerate(fd):
            line = line.strip()
            if line and first:
                start = deque(line)
            elif not line and first:
                first = False
            else:
                match = pattern.search(line)
                rules[match.group(1)] = match.group(2)
    return start, rules


def part1():
    # First Part
    start, rules = load_input()
    dq = start
    chars = set(rules.values())
    steps = 10
    for _ in range(steps):
        for _ in range(len(dq)-1):
            key = start[0] + start[1]
            value = rules[key]
            dq.rotate(-1)
            dq.append(value)
        else:
            dq.rotate(-1)

    occurrences = [dq.count(char) for char in chars]
    result = max(occurrences) - min(occurrences)
    print("Part 1:", result)
    return


def part2():
    # Second part
    start, rules = load_input()

    chars = set(rules.values())
    counts = {k:start.count(k) for k in chars}
    pairs = {k:0 for k in rules.keys()}

    for i in range(len(start)-1):
        pair = start[i] + start[i+1]
        pairs[pair] += 1

    steps = 40
    for _ in range(steps):
        next_pairs = {k:0 for k in rules.keys()}
        for pair in pairs.keys():
            counts[rules[pair]] += pairs[pair]
            new_pair0 = pair[0] + rules[pair]
            new_pair1 = rules[pair] + pair[1]
            next_pairs[new_pair0] += pairs[pair]
            next_pairs[new_pair1] += pairs[pair]
        pairs = next_pairs

    result = max(counts.values()) - min(counts.values())
    print("Part 2:", result)
    return


if __name__ == '__main__':
    part1()
    part2()
