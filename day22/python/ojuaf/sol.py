
#!/usr/bin/env python3

import re
import math as m
from collections import defaultdict

pattern = re.compile("(\w+) x=([^\.]+)\.\.([^\.]+),y=([^\.]+)\.\.([^\.]+),z=([^\.]+)\.\.([^\.]+)")


def load_input():
    data = list()
    with open('input') as fd:
        for line in fd:
            line = line.strip()
            match = pattern.search(line)
            operator = match.group(1)
            x_min = int(match.group(2))
            x_max = int(match.group(3))+1
            y_min = int(match.group(4))
            y_max = int(match.group(5))+1
            z_min = int(match.group(6))
            z_max = int(match.group(7))+1
            data.append((operator, x_min, x_max, y_min, y_max, z_min, z_max))
    return data


def part1():
    # First Part
    data = load_input()
    coords = set()
    for operator, x_min, x_max, y_min, y_max, z_min, z_max in data:
        xt_min = x_min if x_min>-50 else -50
        xt_max = x_max if x_max<50 else 50
        yt_min = y_min if y_min>-50 else -50
        yt_max = y_max if y_max<50 else 50
        zt_min = z_min if z_min>-50 else -50
        zt_max = z_max if z_max<50 else 50
        
        cube = [(x, y, z) for x in range(xt_min, xt_max) for y in range(yt_min, yt_max) for z in range(zt_min, zt_max)]
        if operator == 'on':
            coords.update(cube)
        else:
            coords.difference_update(cube)
    result = len(coords)
    print("Part 1:", result)
    return

def calc_volume(data):
    xs = set()
    ys = set()
    zs = set()
    for operator, x_min, x_max, y_min, y_max, z_min, z_max in data:
        xs.add(x_min)
        xs.add(x_max)
        ys.add(y_min)
        ys.add(y_max)
        zs.add(z_min)
        zs.add(z_max)
    xs = sorted(list(xs))
    ys = sorted(list(ys))
    zs = sorted(list(zs))
    
    grid = dict()
    for operator, x_min, x_max, y_min, y_max, z_min, z_max in data:
        x_start = xs.index(x_min)
        y_start = ys.index(y_min)
        z_start = zs.index(z_min)
        for i in range(x_start, len(xs)-1):
            if xs[i] >= x_max:
                break
            for j in range(y_start, len(ys)-1):
                if ys[j] >= y_max:
                    break
                for k in range(z_start, len(zs)-1):
                    if zs[k] >= z_max:
                        break
                    if operator == "on":
                        grid[(xs[i], xs[i+1], ys[j], ys[j+1], zs[k], zs[k+1])] = True
                    else:
                        if (xs[i], xs[i+1], ys[j], ys[j+1], zs[k], zs[k+1]) in grid:
                            del grid[(xs[i], xs[i+1], ys[j], ys[j+1], zs[k], zs[k+1])]

    volume = 0
    for cube in grid:
        volume += (cube[1]-cube[0])*(cube[3]-cube[2])*(cube[5]-cube[4])

    return volume
    
def part2():
    # Second part
    data = load_input()
    result = 0
    volume = 0
    for i in range(len(data)):
        cubes = list()
        operator, x_min, x_max, y_min, y_max, z_min, z_max = data[i]
        if operator == 'on':
            volume += (x_max-x_min)*(y_max-y_min)*(z_max-z_min)
        for j in range(i):
            operatorj, xj_min, xj_max, yj_min, yj_max, zj_min, zj_max = data[j]
            xd_min = max(xj_min, x_min)
            xd_max = min(xj_max, x_max)
            yd_min = max(yj_min, y_min)
            yd_max = min(yj_max, y_max)
            zd_min = max(zj_min, z_min)
            zd_max = min(zj_max, z_max)
            if xd_min < xd_max and yd_min < yd_max and zd_min < zd_max:
                cubes.append((operatorj, xd_min, xd_max, yd_min, yd_max, zd_min, zd_max))
        volume -= calc_volume(cubes)
    result = volume

    print("Part 2:", result)
    return


if __name__ == '__main__':
    part1()
    part2()
