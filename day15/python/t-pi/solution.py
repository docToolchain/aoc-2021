# see README.adoc

import math
from pprint import pprint
from queue import PriorityQueue

def read_daily_input(filename):
    ''' Read lines from file with given input name
        cast to daily required type and return list
    '''
    with open(filename) as input_file:
        local_list = [item.strip() for item in input_file.readlines()]
    return_list = [[int(c) for c in line] for line in local_list]
    return return_list


def transpose_list(any_list):
    ''' Transpose list for analysis
    '''
    return [[row[i] for row in any_list] for i in range(len(any_list[0]))]


def eval_nextline(risk_layer, firstline = False):
    ''' Evaluate risk for all positions in second line
        Return new risklayer
        Does not work if optimal path has to go up a line...
    '''
    less_risk_layer = [row[:] for row in risk_layer]
    if (len(less_risk_layer) == 1):
        return less_risk_layer
    if firstline:
        for i in range(len(risk_layer[0])):
            risk_factors = [sum(risk_layer[0][:j+1])+sum(risk_layer[1][j:i+1])-risk_layer[0][0] for j in range(i+1)]
            # print(i, risk_factors)
            less_risk_layer[1][i] = min(risk_factors)
    else:
        min_risks = [sum(risk_layer[0]) for __ in range(len(risk_layer[0]))]
        for start in range(len(risk_layer[0])):
            risk_factors = [0 for __ in range(len(risk_layer[0]))]
            for i in range(len(risk_layer[0])):
                if (i<start):
                    risk_factors[i] = risk_layer[0][start] + sum(risk_layer[1][i:start])
                else:
                    risk_factors[i] = min([sum(risk_layer[0][start:j+1])+sum(risk_layer[1][j:i+1]) for j in range(start, i+1)])
                if (risk_factors[i] < min_risks[i]):
                    min_risks[i] = risk_factors[i]
                # print(i, risk_factors)
        less_risk_layer[1] = min_risks
    return less_risk_layer[1:]


def get_path(risk_layer):
    ''' First Djikstra approach: Create graph (dict) of risk layer and push connections to PriorityQueue.
        Return lower right value
    '''
    risk_graph = {(x, y): r for x, line in enumerate(risk_layer) for y, r in enumerate(line)}
    max_x = len(risk_layer)
    max_y = len(risk_layer[0])
    q = PriorityQueue()
    q.put((0, (0, 0)))
    visited = {(0, 0): 0}
    while not q.empty():
        _r, (_x, _y) = q.get()
        for adj in {(-1, 0), (1, 0), (0, -1), (0, 1)}:
            _nx = _x + adj[0]
            _ny = _y + adj[1]
            if ((_nx < 0) or (_nx >= max_x) or (_ny < 0) or (_ny >= max_y)):
                continue
            _nr = _r + risk_graph[(_nx, _ny)]
            if (_nx, _ny) == (max_x - 1, max_y - 1):
                return _nr
            if (((_nx, _ny) in visited.keys()) and (visited[(_nx, _ny)] <= _nr)):
                continue
            visited[(_nx, _ny)] = _nr
            q.put((_nr, (_nx, _ny)))
        # print(visited)


def expand_risklayer(risk_layer):
    ''' Expand map 4x to the right and to the bottom for star2
        Increase risk level on each expansion by 1
        Returns expanded risklayer
    '''    
    large_risklayer = list(risk_layer).copy()
    for x_add in range(4):
        chunk = [[((r+x_add)%9+1) for r in line] for line in risk_layer]
        for j in range(len(large_risklayer.copy())):
            large_risklayer[j] = large_risklayer[j] + chunk[j]
    
    basic_risklayer = large_risklayer.copy()
    for y_add in range(4):
        for j, line in enumerate(basic_risklayer):
            new_line = [((r+y_add)%9+1) for r in line]
            large_risklayer.append(new_line)
        # print("-------------------")
        # for line in new_risklayer:
        #     print('.'.join(str(i) for i in line))
    return large_risklayer


def main():
    risk_layer = read_daily_input('input15.txt')
    # risk_layer = read_daily_input('input_test.txt')
    
    # for line in risk_layer:
    #     print('.'.join(str(i) for i in line))
    star1 = get_path(risk_layer)
    print(f"Result (*): {star1}")
    huge_risklayer = expand_risklayer(risk_layer)
    star2 = get_path(huge_risklayer)
    print(f"Result (**): {star2}")


if __name__ == "__main__":
    main()
