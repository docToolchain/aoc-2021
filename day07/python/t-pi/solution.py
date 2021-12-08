# see description.adoc

from pprint import pprint

def read_daily_input(filename):
    ''' Read lines from file with given input name
        cast to daily required type and return list
    '''
    with open(filename) as input_file:
        local_list = [item.strip() for item in input_file.readlines()]
        return_list = [int(elem) for elem in local_list[0].split(',')]
        return return_list


def align_on_position_lin_fuel(crab_positions, align_position):
    ''' Compute fuel needed to align crab's positions on align_position,
        star1: linear fuel need with distance
        Return amount of fuel (1 per position in-/decrement)
    '''
    max_pos = max(crab_positions)
    if (align_position > max_pos):
        return 0
    fuel_needed = [abs(crab_positions[i]-align_position) for i in range(len(crab_positions))]
    return sum(fuel_needed)


def align_on_position_inc_fuel(crab_positions, align_position):
    ''' Compute fuel needed to align crab's positions on align_position, 
        star2: growing fuel need per distance with higher distance (1:1 + 2:2 + 3:3 + ...)
        Return amount of fuel (1 per position in-/decrement)
    '''
    max_pos = max(crab_positions)
    if (align_position > max_pos):
        return 0
    fuel_needed = [sum(range(abs(crab_positions[i]-align_position)+1)) for i in range(len(crab_positions))]
    return sum(fuel_needed)


def main():
    daily_list = read_daily_input('input07.txt')
    max_pos = max(daily_list)
    min_fuel = sum(daily_list)
    for i in range(max_pos):
        pos_fuel = align_on_position_lin_fuel(daily_list, i)
        if (pos_fuel == 0):
            continue
        if (pos_fuel < min_fuel):
            min_fuel = pos_fuel
            print(i, min_fuel)
            star1 = min_fuel
    print(f"Result (*): {star1}")
    min_fuel = sum(daily_list)*sum(daily_list)
    for i in range(max_pos):
        pos_fuel = align_on_position_inc_fuel(daily_list, i)
        if (pos_fuel == 0):
            continue
        if (pos_fuel < min_fuel):
            min_fuel = pos_fuel
            print(i, min_fuel)
            star2 = min_fuel
    print(f"Result (*): {star1}")
    print(f"Result (**): {star2}")


if __name__ == "__main__":
    main()
