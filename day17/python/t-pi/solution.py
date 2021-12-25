# see README.adoc

import math
from pprint import pprint

def read_daily_input(filename):
    ''' Read lines from file with given input name
        cast to daily required type and return list
    '''
    with open(filename) as input_file:
        local_list = [item.strip() for item in input_file.readlines()]
    target_area_range = [int(num.strip(', ')) for item in local_list[0].split()[2:4] for num in item.split("=")[1].split('..')]
    x = target_area_range[:2]
    y = target_area_range[2:]
    return x, y


def check_target(pos, target_x, target_y):
    ''' Check if pos is in target_range
        Return bool
    ''' 
    x, y = pos
    return ((target_x[0] <= x <= target_x[1]) and (target_y[0] <= y <= target_y[1]))


def shoot_probe(velocity, target_x, target_y):
    ''' Shoot probe with initial velocity.
        Return whether target region reached as max height or not as 0
        Rules:        
            The probe's x position increases by its x velocity.
            The probe's y position increases by its y velocity.
            Due to drag, the probe's x velocity changes by 1 toward the value 0; that is, it decreases by 1 if it is greater than 0, increases by 1 if it is less than 0, or does not change if it is already 0.
            Due to gravity, the probe's y velocity decreases by 1.
    '''
    x, y = (0, 0)
    v_x, v_y = velocity
    max_height = 0
    while x <= max(target_x) and y >= min(target_y):
        x += v_x
        y += v_y
        if (y > max_height):
            max_height = y
        if check_target((x, y), target_x, target_y):
            return True, max_height
        v_x = max(0, v_x - 1)
        v_y -= 1
    return False, 0

def main():
    target_x, target_y = read_daily_input('input17.txt')
    # target_x, target_y = read_daily_input('input_test.txt')
    print('Target area: ', target_x, target_y)
    n = 0
    sum_num = 0
    while sum_num < target_x[0]:
        n += 1
        sum_num = (n * (n + 1)) / 2
    results = []
    for v_x in range(n, max(target_x)+1):
        for v_y in range((-20*n), 20*n):
            mark, result = shoot_probe((v_x, v_y), target_x, target_y)
            if (mark):
                results.append(result)
    star1 = max(results)
    print(f"Result (*): {star1}")
    star2 = len(results)
    print(f"Result (**): {star2}")


if __name__ == "__main__":
    main()
