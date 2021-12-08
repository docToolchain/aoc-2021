# see description.adoc

from pprint import pprint


def read_daily_input(filename):
    ''' Read lines from file with given input name
        cast to daily required type and return list
    '''
    with open(filename) as input_file:
        local_list = [item.strip() for item in input_file.readlines()]
        numbers_list = [int(elem) for elem in local_list[0].split(',')]
        return_list = [numbers_list.count(i) for i in range(10)]
        return return_list


def iterate_one_day(fishcount_list):
    ''' Decrease counters by 1, spawn new 8's by 0
        Return new list
    '''
    fishcount_list[9] = fishcount_list[0]
    fishcount_list[7] += fishcount_list[0]
    for i in range(1, len(fishcount_list)):
        fishcount_list[i-1] = fishcount_list[i]
    fishcount_list[9] = 0
    return fishcount_list


def main():
    daily_list = read_daily_input('input06.txt')
    pprint(daily_list)
    for i in range(80):
        daily_list = iterate_one_day(daily_list)
        print(i+1, daily_list)
    star1 = sum(daily_list)
    print(f"Result (*): {star1}")
    for i in range(256-80):
        daily_list = iterate_one_day(daily_list)
        print(i+1, daily_list)
    star2 = sum(daily_list)
    print(f"Result (**): {star2}")


if __name__ == "__main__":
    main()
