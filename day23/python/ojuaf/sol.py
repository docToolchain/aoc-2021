
#!/usr/bin/env python3

import re
import math as m
from networkx import Graph
from networkx import shortest_path
from networkx import shortest_path_length
import copy

g_min_energy = m.inf

hallways = [(1,1), (2,1), (4,1), (6,1), (8,1), (10, 1), (11, 1)]
weights = {'A': 1, 'B': 10, 'C': 100, 'D': 1000}
chars = ('A', 'B', 'C', 'D', '.')

def load_input(path):
    amphipods = list()
    with open(path) as fd:
        data = list()
        g = Graph()
        for i, line in enumerate(fd):
            data.append(line)
            for j, ch in enumerate(line):
                if ch in weights:
                    amphipods.append([ch, (j,i)])
    for i in range(0, len(data)-1):
        for j in range(0, len(data[i])-1):
            if data[i][j] in chars and data[i+1][j] in chars:
                g.add_edge((j,i), (j,i+1))
            if data[i][j] in chars and data[i][j+1] in chars:
                g.add_edge((j,i), (j+1,i))
    return amphipods, g

def is_free(path, amphipods):
    positions = set([pos for _, pos in amphipods])
    path = set(path)
    prev_len = len(path)
    path.difference_update(positions)
    
    if prev_len == len(path):
        return True
    return False

def min_energy_remaining(g, amphipods, rooms):
    min_energy = 0
    for amphipod, pos in amphipods:
        min_energy += shortest_path_length(g, rooms[amphipod][-1], pos)*weights[amphipod]
        rooms[amphipod].pop()
    return min_energy

def process(g, amphipods, rooms, energy,level):
    level += 1
    global g_min_energy
    if g_min_energy != m.inf and g_min_energy <= energy+min_energy_remaining(g, amphipods, copy.deepcopy(rooms)):
        return

    finished = True
    for i, item in enumerate(amphipods):
        amphipod, pos = item
        finished = False
        room = rooms[amphipod][-1]
        path = shortest_path(g, room, pos)[:-1]
        if is_free(path, amphipods):
            next_amphipods = copy.deepcopy(amphipods)
            del next_amphipods[i]
            next_rooms = copy.deepcopy(rooms)
            next_rooms[amphipod].pop()
            process(g, next_amphipods, next_rooms, energy+len(path)*weights[amphipod], level)
        else:
            if pos not in hallways:
                for hallway in hallways:
                    path = shortest_path(g, hallway, pos)[:-1]
                    if is_free(path, amphipods):
                        next_amphipods = copy.deepcopy(amphipods)
                        next_amphipods[i][1] = hallway
                        process(g, next_amphipods, rooms, energy+len(path)*weights[amphipod], level)

    if finished:
        print(energy)
        g_min_energy = energy
    return


def part1():
    # First Part
    amphipods, g = load_input('input')
    rooms = {'A': [(3, i) for i in range(2, 4)], 'B': [(5, i) for i in range(2, 4)], 'C': [(7, i) for i in range(2, 4)], 'D': [(9, i) for i in range(2, 4)]}
    process(g, amphipods, rooms, 0, 0)        
                
    print("Part 1:", g_min_energy)
    return


def part2():
    # Second part
    global g_min_energy
    g_min_energy = m.inf
    amphipods, g = load_input('input_b')
    rooms = {'A': [(3, i) for i in range(2, 6)], 'B': [(5, i) for i in range(2, 6)], 'C': [(7, i) for i in range(2, 6)], 'D': [(9, i) for i in range(2, 6)]}
    process(g, amphipods, rooms, 0, 0)        
                
    result = g_min_energy
    print("Part 2:", result)
    return


if __name__ == '__main__':
    part1()
    # part2()
