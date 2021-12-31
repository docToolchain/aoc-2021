
#!/usr/bin/env python3

import re
import math as m
from collections import deque

pattern = re.compile("target area: x=(\d+)..(\d+), y=(-\d+)..(-\d+)")


def load_input():
    data = list()
    with open('input_test') as fd:
        temp = list()
        for line in fd:
            line = line.strip()
            if line:
                if "scanner" not in line:
                    temp.append(tuple(map(int, line.split(','))))
            else:
                data.append(temp)
                temp = list()
        else:
            data.append(temp)
    return data

def get_orientations(data):
    orientations = list()
    orientations.append((data[0], data[1], data[2]))
    orientations.append((data[1], -data[0], data[2]))
    orientations.append((-data[0], -data[1], data[2]))
    orientations.append((-data[1], data[0], data[2]))
    orientations.append((-data[2], data[1], data[0]))
    orientations.append((data[2], data[1], -data[0]))
    
    all_orientations = list()
    for orientation in orientations:
        all_orientations.extend(get_ups(orientation))
        
    return all_orientations

def get_ups(data):
    orientations = list()
    orientations.append((data[0], data[1], data[2]))
    orientations.append((data[0], -data[2], data[1]))
    orientations.append((data[0], -data[1], -data[2]))
    orientations.append((data[0], data[2], -data[1]))
    return orientations

def part1():
    # First Part
    data = load_input()
    scans = list()
    beacons = list()
    for l in data:
        scans.append(sorted(l, key=lambda x: x[0]))
    beacons.append(scans[0])
    
    scanners = [(0,0,0)]

    print(beacons)
    del scans[0]
    number_beacons = 1
    while True:
        last_beacons = number_beacons
        number_beacons = 0
        print("length:", len(scans))
        if not scans:
            break
        for l in range(len(beacons)-last_beacons, len(beacons)):
            for i in range(len(beacons[l])):
                remove = list()
                ref_beacons = set([(beacon[0]-beacons[l][i][0], beacon[1]-beacons[l][i][1], beacon[2]-beacons[l][i][2]) for beacon in beacons[l]])
                for idx, scan in enumerate(scans):
                    orientations = list()
                    for beacon in scan:
                        orientations.append(get_orientations(beacon))
                    for k in range(24):
                        orientation = [orientation[k] for orientation in orientations]
                        for j in range(len(scan)):
                            sbeacons = set([(beacon[0]-orientation[j][0], beacon[1]-orientation[j][1], beacon[2]-orientation[j][2]) for beacon in orientation])
                            intersects = ref_beacons.intersection(sbeacons)
                            if len(intersects) > 11:
                                sbeacons = [(position[0]+beacons[l][i][0], position[1]+beacons[l][i][1], position[2]+beacons[l][i][2]) for position in sbeacons]
                                beacons.append(sbeacons)
                                scanners.append((-orientation[j][0]+beacons[l][i][0], -orientation[j][1]+beacons[l][i][1], -orientation[j][2]+beacons[l][i][2]))
                                number_beacons += 1
                                del scans[idx]
                        
        last_beacons = number_beacons

    print(scanners)
    result = set()
    for l in beacons:
        result = result.union(l)
    print("Part 1:", len(result))
    
    result = list()
    for i in range(len(scanners)-1):
        for j in range(i+1, len(scanners)-1):
            distance = 0
            for k in range(3):
                distance += abs(scanners[i][k] - scanners[j+1][k])
            result.append(distance)
    print("Part 2:", max(result))
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
    # part2()
