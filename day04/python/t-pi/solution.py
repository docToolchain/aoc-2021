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
        bingo_numbers = [int(num) for num in local_list[0].split(',')]
        bingo_fields = list()
        bingo_field = list()
        for item in local_list[2:]:
            field_line = [int(num) for num in item.split()]
            if (len(field_line) == 0):
                bingo_fields.append(bingo_field)
                bingo_field = list()
            else:
                bingo_field.append(field_line)
        return bingo_numbers, bingo_fields


def transpose_list(input_list):
    ''' Transpose list for analysis
    '''
    item_length = len(input_list[0])
    transposed_list = [[] for i in range(item_length)]
    for item in input_list:
        for i in range(item_length):
            transposed_list[i].append(item[i])
    return transposed_list


def check_bingo_field(bingo_field):
    ''' Check single bingo field for full row or full column of negative values
        Returns sum of positive values (unmarked), else 0
    '''
    bingo = False
    bingo_result = 0
    bingo_columns = transpose_list(bingo_field)
    for bingo_line in bingo_field+bingo_columns:
        if (all(elem < 0 for elem in bingo_line)):
            bingo = True
    if bingo:
        for bingo_line in bingo_field:
            for elem in bingo_line:
                bingo_result += elem if elem > 0 else 0
    return bingo_result


def mark_bingo_field(bingo_field, new_number):
    ''' Marks numbers in bingo_field by subtracting 100 (max bingo value = 99)
        Returns marked bingo_field
    '''
    for i_row in range(len(bingo_field)):
        for i_col in range(len(bingo_field[i_row])):
            if (bingo_field[i_row][i_col] == new_number):
                bingo_field[i_row][i_col] -= 100
    return bingo_field


def main():
    bingo_numbers, bingo_fields = read_daily_input('input04.txt')
    pprint(bingo_numbers)
    pprint(bingo_fields)
    star1 = 0
    star2 = 0
    have_won = [False] * len(bingo_fields)
    for num in bingo_numbers:
        for i, bingo_field in enumerate(bingo_fields):
            bingo_field = mark_bingo_field(bingo_field, num)
            checksum = check_bingo_field(bingo_field)
            if (checksum > 0):
                if (star1 == 0):
                    star1 = num * check_bingo_field(bingo_field)
                    print(num, checksum, star1)
                if not have_won[i]:
                    star2 = num * check_bingo_field(bingo_field)
                    print(i, num, checksum, star2)
                have_won[i] = True
    print(f"Result (*): {star1}")
    print(f"Result (**): {star2}")


if __name__ == "__main__":
    main()
