
#!/usr/bin/env python3

from networkx import Graph
import string

lower_letters = string.ascii_lowercase
upper_letters = string.ascii_uppercase

def load_input():
    g = Graph()
    with open('input') as fd:
        for i, line in enumerate(fd):
            a, b = line.strip().split('-')
            g.add_edge(a, b)
    return g

def find_paths2(g, path, visited_twice):
    last = path[len(path)-1]
    paths = list()
    neighbors = g.neighbors(last)
    for neighbor in neighbors:
        found_paths = list()
        if neighbor == 'start':
            continue

        if neighbor == 'end':
            paths.append([*path, neighbor])
        else:
            if neighbor[0] in lower_letters and neighbor in path:
                if visited_twice:
                    continue
                else:
                    found_paths = find_paths2(g, [*path, neighbor], True)
            else:
                found_paths = find_paths2(g, [*path, neighbor], visited_twice)
        if found_paths:
            paths.extend(found_paths)
    return paths

def find_paths(g, path):
    last = path[len(path)-1]
    paths = list()
    neighbors = g.neighbors(last)
    for neighbor in neighbors:
        if neighbor[0] in lower_letters and neighbor in path:
            continue
        if neighbor == 'end':
            paths.append([*path, neighbor])
        else:
            found_paths = find_paths(g, [*path, neighbor])
            if found_paths:
                paths.extend(found_paths)
    return paths


def part1():
    # First Part
    g = load_input()
    paths = list()
    paths = find_paths(g, ['start'])
    result = len(paths)
    print("Part 1:", result)
    return

def part2():
    # Second part
    g = load_input()
    paths = list()
    paths = find_paths2(g, ['start'], False)
    result = len(paths)
    print("Part 2:", result)
    return

if __name__ == '__main__':
    part1()
    part2()
