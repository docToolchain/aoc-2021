# see README.adoc

import math
from pprint import pprint

def read_daily_input(filename):
    ''' Read lines from file with given input name
        cast to daily required type and return list
    '''
    with open(filename) as input_file:
        local_list = [item.strip() for item in input_file.readlines()]
    coordinates = [item.split(',') for item in local_list[:local_list.index('')]]
    fold_instructions = [item.split()[2] for item in local_list[local_list.index('')+1:]]
    return coordinates, fold_instructions


def preprocess_input(coordinates_list, fold_instructions):
    ''' Preprocess daily input
        Put coordinates into sheet, parse fold_instructions
        Return sheet with dots, fold lines
        ! BEWARE: 2d list 'sheet' is flipped & rotated vs. instructions
    '''
    coordinates = [[int(x), int(y)] for (x, y) in coordinates_list]
    max_x = max([x for (x, __) in coordinates])
    max_y = max([y for (__, y) in coordinates])
    sheet = [[0 for y in range(max_y+1)] for x in range(max_x+1)]
    for point in coordinates:
        sheet[point[0]][point[1]] = 1
    folds=[[axis, int(coord)] for (axis, coord) in (item.split('=') for item in fold_instructions)]
    return sheet, folds


def transpose_list(any_list):
    ''' Transpose list for analysis
    '''
    return [[row[i] for row in any_list] for i in range(len(any_list[0]))]


def fold_sheet(sheet, fold_axis, fold_coord):
    ''' "Fold" sheet along axis on line coord.
        superpose inverted second half with first half and discard second half
        Return new sheet
        ! BEWARE: 2d list 'sheet' is flipped & rotated vs. instructions
    '''
    max_row = len(sheet)
    max_col = len(sheet[0])
    if fold_axis == 'y':  # fold along col
        for row in range(max_row):
            for i, col in enumerate(range(fold_coord+1, max_col)):
                sheet[row][fold_coord-1-i] = 1 if (sheet[row][fold_coord-1-i]+sheet[row][col] > 0) else 0
        folded_sheet = [item[:fold_coord] for item in sheet]
        return folded_sheet
    else:  # fold along row
        for i, row in enumerate(range(fold_coord+1, max_row)):
            for col in range(max_col):
                sheet[fold_coord-1-i][col] = 1 if (sheet[fold_coord-1-i][col]+sheet[row][col] > 0) else 0
        folded_sheet = [item for item in sheet[:fold_coord]]
        return folded_sheet



def main():
    coordinates, fold_instructions = read_daily_input('input13.txt')
    sheet, folds = preprocess_input(coordinates, fold_instructions)
    first_folded_sheet = fold_sheet(sheet, folds[0][0], folds[0][1])
    star1 = sum([sum(row) for row in first_folded_sheet])
    print(f"Result (*): {star1}")
    fully_folded_sheet = sheet.copy()
    for fold in folds:
        fully_folded_sheet = fold_sheet(fully_folded_sheet, fold[0], fold[1])
    star2_list = transpose_list(fully_folded_sheet)
    for row in star2_list:
        print(''.join(['█' if c == 1 else ' ' for c in row]))
    star2 = 's. above'
    print(f"Result (**): {star2}")


if __name__ == "__main__":
    main()
