#!/usr/bin/env python3

import re

pattern = re.compile('(\d*),(\d*) -> (\d*),(\d*)')


def load_input():
    data = list()
    data1 = dict()
    with open('input') as fd:
        for i, line in enumerate(fd):
            data.append(list())
            for j, height in enumerate(line.strip()):
                data[i].append(int(height))
    return data


def get_neighbours(data, point):
    neighbours = list()
    adjacents = [(1, 0), (-1, 0), (0, 1), (0, -1)]
    for adjacent in adjacents:
        y = point[0] - adjacent[0]
        x = point[1] - adjacent[1]
        if y >= 0 and x >= 0 and x < len(data[0])  and y < len(data):
            neighbours.append((y, x))
    return neighbours


def part1():
    # First part
    data = load_input()
    low_points = list()
    for i, row in enumerate(data):
        for j, height in enumerate(row):
            neighbours = get_neighbours(data, (i, j))
            neighbours_height = [data[neighbour[0]][neighbour[1]] for neighbour in neighbours]
            if min(neighbours_height) > height:
                low_points.append((i, j))
    result = 0
    for low_point in low_points:
        result += data[low_point[0]][low_point[1]] + 1
    print("Part 1:", result)
    y_size = len(data)
    x_size = len(data[0])
    return low_points, data


def part2():
    # Second part
    low_points, data = part1()
    basins = list()
    for low_point in low_points:
        basin = list()
        basin.append(low_point)
        last_neighbours = basin
        while True:
            for last_neighbour in last_neighbours:
                neighbours = get_neighbours(data, last_neighbour)
                basin_neighbour_found = False
                for neighbour in neighbours:
                    if neighbour not in basin and data[neighbour[0]][neighbour[1]] != 9:
                        basin.append(neighbour)
                        basin_neighbour_found = True
                last_neighbours = neighbours
            if basin_neighbour_found == False:
                break
        basins.append(basin)

    size_basins = [len(basin) for basin in basins]

    size_basins.sort(reverse=True)
    result = 1
    for i in range(3):
        result *= size_basins[i]

    print("Part 2:", result)
    return

if __name__ == '__main__':
    part1()
    part2()
