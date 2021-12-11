# see description.adoc

import io
from pprint import pprint

def read_daily_input(filename):
    ''' Read lines from file with given input name
        cast to daily required type and return list
    '''
    with open(filename) as input_file:
        local_list = [item.strip() for item in input_file.readlines()]
        return_list = [[int(digit) for digit in line] for line in local_list]
        return return_list


def do_single_step(energy_levels):
    ''' Iterate energy levels through single step
        Return new energy_levels, number of flashes (energy levels >9), bool if flash count equals all squids
    '''
    flashes = 0
    depth = len(energy_levels)
    width = len(energy_levels[0])
    energy_levels = [[energy+1 for energy in line] for line in energy_levels]
    old_flashes = -1
    while (flashes > old_flashes):
        old_flashes = flashes
        for x in range(depth):
            for y in range(width):
                if (energy_levels[x][y]>9):
                    flashes +=1
                    energy_levels[x][y] = 0
                    adjacents = [(x-1,y+1),(x,y+1),(x+1,y+1),(x-1,y),(x+1,y),(x-1,y-1),(x,y-1),(x+1,y-1)]
                    for adj in adjacents:
                        if ((0 <= adj[0] < depth) and (0 <= adj[1] < width)):
                            if (energy_levels[adj[0]][adj[1]] != 0):
                                energy_levels[adj[0]][adj[1]] += 1
    return energy_levels, flashes, flashes == depth*width


def main():
    daily_list = read_daily_input('input11.txt')
    sum_flashes = 0
    star2 = 0
    for i in range(100):
        daily_list, flash_count, is_sync = do_single_step(daily_list)
        sum_flashes += flash_count
        if (is_sync and (star2 == 0)):
            star2 = i+1
    star1 = sum_flashes
    print(f"Result (*): {star1}")
    if (star2 == 0):
        i = 100
        is_sync = False
        while not is_sync:
            daily_list, flash_count, is_sync = do_single_step(daily_list)
            i += 1
        star2 = i 
    print(f"Result (**): {star2}")


if __name__ == "__main__":
    main()
