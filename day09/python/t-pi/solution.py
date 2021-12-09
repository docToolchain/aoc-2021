# see description.adoc

from pprint import pprint

def read_daily_input(filename):
    ''' Read lines from file with given input name
        cast to daily required type and return list
    '''
    with open(filename) as input_file:
        local_list = [item.strip() for item in input_file.readlines()]
        return_list = [[int(digit) for digit in list(line)] for line in local_list]
        return return_list


def check_adjacent_for_minimum(height_map, x, y):
    ''' Checks adjacent positions for lower values, consider boundary
        x: row, y: col
        Returns 0 if no minimum, else height value + 1
    '''
    if (x>=len(height_map)):
        return 0
    if (y>=len(height_map[0])):
        return 0
    min_value = True
    nominal_value = height_map[x][y]
    if (x!=0):
        if (height_map[x-1][y] <= nominal_value):
            min_value = False
    if (x!=len(height_map)-1):
        if (height_map[x+1][y] <= nominal_value):
            min_value = False
    if (y!=0):
        if (height_map[x][y-1] <= nominal_value):
            min_value = False
    if (y!=len(height_map[0])-1):
        if (height_map[x][y+1] <= nominal_value):
            min_value = False
    return (nominal_value+1) if min_value else 0


def fill_basin_recursive(height_map, min_x, min_y):
    ''' Get basin size for a minimum coordinate
        Returns basin size (number of places)
    '''
    dx = len(height_map)
    dy = len(height_map[0])
    basin_size = 0
    def fill(x,y):
        if (height_map[x][y] == 9): # 9 = limit of basin
            return
        if (height_map[x][y] == -1): # -1 = already visited
            return
        nonlocal basin_size
        basin_size += 1
        height_map[x][y] = -1
        if (x!=0):
            fill(x-1, y)
        if (x!=len(height_map)-1):
            fill(x+1, y)
        if (y!=0):
            fill(x, y-1)
        if (y!=len(height_map[0])-1):
            fill(x, y+1)
    fill(min_x,min_y) 
    return basin_size


def main():
    daily_list = read_daily_input('input09.txt')
    sum_mins = 0
    basins = []
    for x in range(len(daily_list)):
        for y in range(len(daily_list[0])):
            valley_value = check_adjacent_for_minimum(daily_list, x, y)
            if (valley_value > 0):
                sum_mins += valley_value
                basins.append(fill_basin_recursive(daily_list, x, y))
    star1 = sum_mins
    print(f"Result (*): {star1}")
    print(basins)
    prod_basins = 1
    for i in range(3):
        max_basin = max(basins)
        prod_basins *= max_basin
        basins.remove(max_basin)
    star2 = prod_basins
    print(f"Result (**): {star2}")


if __name__ == "__main__":
    main()
