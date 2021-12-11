
#!/usr/bin/env python3

import numpy as np


def load_input():
    data = list()
    with open('input') as fd:
        for i, line in enumerate(fd):
            data.append([int(i) for i in line.strip()])
    data = np.asarray(data)
    return data

def part2():
    # Second part
    data = load_input()
    increases = np.ones((3, 3), np.int64)
    count = 0
    while True:
        count += 1
        total_flashes = 0
        data = data + 1
        ready_for_flash = np.full((data.shape), True, dtype=bool)
        while True:
            flashes = (data > 9) == ready_for_flash
            number_flashes = flashes.sum()
            total_flashes += number_flashes
            if number_flashes == 0:
                data[data>9] = 0
                break
            adjacents = np.zeros((data.shape[0] + 2, data.shape[1] + 2), np.int64)
            it = np.nditer(flashes, flags=['multi_index'])
            for x in it:
                i, j = it.multi_index
                if flashes[i,j]:
                    ready_for_flash[i,j] = False
                    adjacents[i:i+3,j:j+3] += increases
            data += adjacents[1:-1,1:-1]
        if total_flashes == data.size:
            break
    result = count
    print("Part 2:", result)
    return


def part1():
    # First Part
    data = load_input()
    steps = 100
    increases = np.ones((3, 3), np.int64)
    total_flashes = 0
    for step in range(steps):
        data = data + 1
        ready_for_flash = np.full((data.shape), True, dtype=bool)
        while True:
            flashes = (data > 9) == ready_for_flash
            number_flashes = flashes.sum()
            total_flashes += number_flashes
            if number_flashes == 0:
                data[data>9] = 0
                break
            adjacents = np.zeros((data.shape[0] + 2, data.shape[1] + 2), np.int64)
            it = np.nditer(flashes, flags=['multi_index'])
            for x in it:
                i, j = it.multi_index
                if flashes[i,j]:
                    ready_for_flash[i,j] = False
                    adjacents[i:i+3,j:j+3] += increases
            data += adjacents[1:-1,1:-1]
    result = total_flashes
    print("Part 1:", result)
    return

if __name__ == '__main__':
    part1()
    part2()
