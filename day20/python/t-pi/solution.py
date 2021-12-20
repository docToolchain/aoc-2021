# see README.adoc

import math
from pprint import pprint

def read_daily_input(filename):
    ''' Read lines from file with given input name
        cast to daily required type and return list
    '''
    with open(filename) as input_file:
        local_list = [item.strip() for item in input_file.readlines()]
    image_processor = list()
    dots = set()
    read_dots = False
    for i, line in enumerate(local_list):
        if line == '':
            read_dots = True
            line_shift = i + round((len(local_list) - i)/2)
        if not read_dots:
            image_processor.extend([1 if c == '#' else 0 for c in line])
        else:
            col_shift = round(len(line)/2)
            dots.update({(i-line_shift, j-col_shift) for j, c in enumerate(line) if c == '#'})
    return image_processor, dots

def process_raw(dots, image_processor, border, verbose = True):
    ''' Process one iteration of image
        Returns new dots
    '''
    def get_niner(dots, x, y, verbose):
        ''' Get value of niner-block as bits
        '''
        niner = ['1' if (dx, dy) in dots else '0' for dx in range(x-1, x+2) for dy in range(y-1, y+2)]
        if verbose:
            print(''.join(niner), int(''.join(niner),2), image_processor[int(''.join(niner),2)])
        return int(''.join(niner),2)

    min_x = min([x for x, y in dots])
    max_x = max([x for x, y in dots])
    min_y = min([y for x, y in dots])
    max_y = max([y for x, y in dots])
    proc_dots = set()
    if image_processor[get_niner(dots, 0, 0, verbose)] == 1:
        proc_dots.add((0, 0))
    for x in range(min_x-border, max_x+border+1):
        for y in range(min_y-border, max_y+border+1):
                if image_processor[get_niner(dots, x, y, verbose)] == 1:
                    proc_dots.add((x, y))
    ### Trying to fix catch by starting in center... no difference
    # for dx in range(1, max_x+border):
    #     for dy in range(1, max_y+border):
    #         for x in range(-dx, dx):
    #             if image_processor[get_niner(dots, x, dy, verbose)] == 1:
    #                 proc_dots.add((x, dy))
    #             if image_processor[get_niner(dots, x, -dy, verbose)] == 1:
    #                 proc_dots.add((x, -dy))
    #         for y in range(-dy, dy):
    #             if image_processor[get_niner(dots, dx, y, verbose)] == 1:
    #                 proc_dots.add((dx, y))
    #             if image_processor[get_niner(dots, -dx, y, verbose)] == 1:
    #                 proc_dots.add((-dx, y))
    if verbose:
        plot_dots(proc_dots)
    return proc_dots


def plot_dots(dots):
    ''' Print dots matrix
    '''
    print()
    min_x = min([x for x, y in dots])
    max_x = max([x for x, y in dots])
    min_y = min([y for x, y in dots])
    max_y = max([y for x, y in dots])
    for x in range(min_x-1, max_x+2):
        line = ['#' if (x, y) in dots else '.' for y in range(min_y-1, max_y+2)]
        print(''.join(line))
    print()


def main():
    image_processor, dots = read_daily_input('input20.txt')
    # image_processor, dots = read_daily_input('input_test.txt')
    plot_dots(dots)
    print('\n\n...processing...\n\n')
    dots = process_raw(dots, image_processor, 8, False)
    dots = process_raw(dots, image_processor, -4, False)
    plot_dots(dots)
    star1 = len(dots)
    print(f"Result (*): {star1}")
    for i in range(24):
        dots = process_raw(dots, image_processor, 8, False)
        dots = process_raw(dots, image_processor, -4, False)
    plot_dots(dots)
    star2 = len(dots)
    print(f"Result (*): {star1}")
    print(f"Result (**): {star2}")


if __name__ == "__main__":
    main()
