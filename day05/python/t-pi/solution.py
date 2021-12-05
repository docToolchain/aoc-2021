# see description.adoc

from os import linesep
from pprint import pprint


def read_daily_input(filename):
    ''' Read lines from file with given input name
        cast to daily required type and return list
    '''
    with open(filename) as input_file:
        local_list = input_file.readlines()
        local_list = [item.strip() for item in local_list]
        return_list = list()
        for item in local_list:
            start_str, stop_str = item.split("->")
            start = tuple(int(elem) for elem in start_str.split(','))
            stop = tuple(int(elem) for elem in stop_str.split(','))
            return_list.append([start, stop])
        return return_list


def transpose_list(input_list):
    ''' Transpose list for analysis
    '''
    item_length = len(input_list[0])
    transposed_list = [[] for i in range(item_length)]
    for item in input_list:
        for i in range(item_length):
            transposed_list[i].append(item[i])
    return transposed_list

def generate_field(line_list):
    ''' Generate 2D field from coordinate list
        star1: only horizontal & vertical lines are considered
    '''
    my_field = [[0 for i in range(1000)] for j in range(1000)]
    mine_count1 = 0
    mine_count2 = 0
    for line in line_list:
        x1, y1 = line[0]
        x2, y2 = line[1]
        if ((x1 == x2) | (y1 == y2)):
            for x in range(min(x1, x2), max(x1, x2)+1):
                for y in range(min(y1, y2), max(y1, y2)+1):
                    my_field[x][y] += 1
                    if (my_field[x][y] == 2):
                            mine_count1 += 1
    mine_count2 = mine_count1
    for line in line_list:
        x1, y1 = line[0]
        x2, y2 = line[1]
        if ((x1 == x2) | (y1 == y2)):
            continue
        x = x1
        dx = -1 if x1>x2 else 1
        y = y1
        dy = -1 if y1>y2 else 1
        while(x!=(x2+dx)):
            my_field[x][y] += 1
            if (my_field[x][y] == 2):
                mine_count2 += 1
            x += dx
            y += dy
    return my_field, mine_count1, mine_count2


def main():
    daily_list = read_daily_input('input05.txt')
    line_field, star1, star2 = generate_field(daily_list)    
    print(f"Result (*): {star1}")
    print(f"Result (**): {star2}")


if __name__ == "__main__":
    main()
