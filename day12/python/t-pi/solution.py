# see README.adoc

import io
from pprint import pprint

def read_daily_input(filename):
    ''' Read lines from file with given input name
        cast to daily required type and return list
    '''
    with open(filename) as input_file:
        local_list = [item.strip() for item in input_file.readlines()]
        connection_list = [item.split('-') for item in local_list]
        caves = set([item for sublist in connection_list for item in sublist])
        connectors = dict()
        for cave in caves:
            for connection in connection_list:
                if cave == 'end':
                    continue
                if cave in connection:
                    if not (cave in connectors.keys()):
                        connectors[cave] = []
                    for cv in connection:
                        if ((cv != cave) and (cv != 'start')):
                            connectors[cave].append(cv)
        return caves, connectors


def find_paths(caves, links, star = 1):
    ''' find number of paths from 'start' to 'end' by recursively iterating
        star1: visit small caves (lowercase) only once
        star2: visit single small cave twice
    '''
    paths = list()

    def go_step(path, new_cave):
        path.append(new_cave)
        if new_cave == 'end':
            return
        for cave in links[new_cave]:
            if (all(c.islower() for c in cave) and (cave in path)):
                if (star == 1):
                    continue
                lowies = [cv for cv in path if all(c.islower() for c in cv)] 
                if (any(path.count(cv)==2 for cv in lowies)):
                    continue
            paths.append(path.copy())
            go_step(paths[-1], cave)
    
    paths.append([])
    go_step(paths[-1], 'start')
    paths = [p for p in paths if p[-1] == 'end']
    paths = [p for i,p in enumerate(paths) if p not in paths[i+1:]]
    # pprint(paths)
    return paths


def main():
    caves, links = read_daily_input('input12.txt')
    star1 = len(find_paths(caves, links))
    print(f"Result (*): {star1}")
    print("Path finding for Star2 will take some time...")
    star2 = len(find_paths(caves, links, 2))
    print(f"Result (**): {star2}")


if __name__ == "__main__":
    main()
