
#!/usr/bin/env python3

import re
import math as m
from collections import deque

pattern = re.compile("target area: x=(\d+)..(\d+), y=(-\d+)..(-\d+)")


def load_input():
    with open('input') as fd:
        match = pattern.search(fd.read())
        x = (int(match.group(1)), int(match.group(2)))
        y = (int(match.group(3)), int(match.group(4)))
    return x, y


def part1():
    # First Part
    target_x, target_y = load_input()
    v_ymax = abs(min(target_y)) - 1
    n_ymax = v_ymax
    y_max = v_ymax*(n_ymax + 1) - (n_ymax*(n_ymax +1)/2)

    result = int(y_max)
    print("Part 1:", result)
    return


def part2():
    # Second part
    target_x, target_y = load_input()

    v_xmax = max(target_x)
    v_xmin = m.ceil(m.sqrt(min(target_x)*2 - 0.25) - 0.5)
    
    v_ymax = abs(min(target_y)) - 1
    v_ymin = min(target_y)
    
    n_max = 2*v_ymax+2
    
    target_x = [x for x in range(min(target_x), max(target_x)+1)]
    target_y = [y for y in range(min(target_y), max(target_y)+1)]
    
    velocities = set()

    for n in range(1, n_max+1):
        for v_x in range(v_xmin, v_xmax+1):
            slow = n*(n-1)//2
            if n > v_x:
                x_pos = v_x*(v_x+1)//2
            else:
                x_pos = v_x*n - n*(n-1)//2
            if x_pos in target_x:
                for v_y in range(v_ymin, v_ymax+1):
                    y_pos = v_y*n - n*(n-1)//2
                    if y_pos in target_y:
                        velocities.add((v_x, v_y))
    
    result = len(velocities)
    print("Part 2:", result)
    return


if __name__ == '__main__':
    part1()
    part2()
