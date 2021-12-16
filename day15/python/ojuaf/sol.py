
#!/usr/bin/env python3

import re
import numpy as np

from networkx import DiGraph
from networkx import shortest_path

pattern = re.compile("(\w\w) -> (\w)")


def load_input():
    data = list()
    with open('input') as fd:
        first = True
        for i, line in enumerate(fd):
            line = line.strip()
            data.append(list(map(int, line)))
    return data


def part1():
    # First Part
    data = load_input()
    g = DiGraph()
    neighbours = [(-1, 0), (0, -1), (1, 0), (0, 1)]
    max_y_pos = len(data)-1
    max_x_pos = len(data[0])-1
    for j in range(max_y_pos+1):
        for i in range(max_x_pos+1):
            for neighbour in neighbours:
                y_pos = j + neighbour[0]
                x_pos = i + neighbour[1]
                if x_pos >= 0 and y_pos >= 0 and x_pos <= max_x_pos and y_pos <= max_y_pos:
                    g.add_edge((j, i), (y_pos, x_pos), weight=data[y_pos][x_pos])

    shortest = shortest_path(g, (0,0), (max_y_pos, max_x_pos), weight="weight")
    result = 0
    for i in range(len(shortest)-1):
        result += g[shortest[i]][shortest[i+1]]["weight"]
    print("Part 1:", result)
    return


def part2():
    # Second part
    data = load_input()
    g = DiGraph()
    neighbours = [(-1, 0), (0, -1), (1, 0), (0, 1)]
    y_size = len(data)
    x_size = len(data[0])
    max_y_pos = y_size*5-1
    max_x_pos = x_size*5-1

    risks = np.zeros((y_size*5, x_size*5))
    field = np.asarray(data)

    for j in range(5):
        for i in range(5):
            risks[j*y_size:(j+1)*y_size, i*x_size:(i+1)*x_size] = (field + i + j)
            indices = np.where(risks>9)
            risks[indices] = risks[indices] - 9

    for j in range(risks.shape[0]):
        for i in range(risks.shape[1]):
            for neighbour in neighbours:
                y_pos = j + neighbour[0]
                x_pos = i + neighbour[1]
                if x_pos >= 0 and y_pos >= 0 and x_pos <= max_x_pos and y_pos <= max_y_pos:
                    g.add_edge((j, i), (y_pos, x_pos), weight=risks[y_pos][x_pos])

    shortest = shortest_path(g, (0,0), (max_y_pos, max_x_pos), weight="weight")
    result = 0
    for i in range(len(shortest)-1):
        result += int(g[shortest[i]][shortest[i+1]]["weight"])
    print("Part 2:", result)
    return


if __name__ == '__main__':
    part1()
    part2()
