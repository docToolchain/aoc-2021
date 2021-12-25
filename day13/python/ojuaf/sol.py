
#!/usr/bin/env python3

import re

pattern = re.compile("fold along ([xy])=(\d+)")


def load_input():
    data = set()
    folds = list()
    with open('input') as fd:
        first = True
        for i, line in enumerate(fd):
            line = line.strip()
            if line and first:
                x, y = line.split(',')
                data.add((int(x), int(y)))
            elif not line and first:
                first = False
            else:
                match = pattern.search(line)
                folds.append([0 if match.group(1) == 'x' else 1, int(match.group(2))])
    return data, folds


def parts():
    # First Part
    data, folds = load_input()

    count = 0
    for fold in folds:
        count += 1
        next_data = set()
        for coords in data:
            if coords[fold[0]] > fold[1]:
                value = 2*fold[1] - coords[fold[0]]
                coords = (value, coords[1]) if fold[0] == 0 else (coords[0], value)
            next_data.add(coords)
        data = next_data
        if count == 1:
            result = len(data)
            print("Part 1:", result)

    # Second Part
    x_values = [coords[0] for coords in data]
    y_values = [coords[1] for coords in data]

    print("Part 2:")
    for y in range(max(y_values)+1):
        line = ""
        for x in range(max(x_values)+1):
            if (x, y) in data:
                line += '#'
            else:
                line += '.'
        print(line)

    return


if __name__ == '__main__':
    parts()
