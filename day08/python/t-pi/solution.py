# see description.adoc

import io
from pprint import pprint

def read_daily_input(filename):
    ''' Read lines from file with given input name
        cast to daily required type and return list
    '''
    with open(filename) as input_file:
        local_list = [item.strip() for item in input_file.readlines()]
        digit_list = [digit for digit in (elem.split() for elem in local_list)]
        return digit_list


def identify_output_number(io_line):
    ''' Parse digits to identify the numbers
        Use 'one' and ('four' minus 'one') as distinguishing feature for 5 and 6 active segments
        Return output number as int
    '''
    len_line = [len(digit) for digit in io_line]
    digit_presence = [len_line.count(i) for i in range(7)]
    # print(digit_presence)
    one = list(next(obj for obj in io_line if len(obj) == 2))
    four = list(next(obj for obj in io_line if len(obj) == 4))
    crochet = [seg for seg in four if seg not in one]
    # print(one, four, crochet)
    output = False
    result = ''
    for digit in io_line:
        brightness = len(digit)
        if (brightness == 1):
            output = True
            continue
        if output:
            if (brightness == 2):
                result += '1'
            if (brightness == 3):
                result += '7'
            if (brightness == 4):
                result += '4'
            if (brightness == 7):
                result += '8'
            if (brightness == 5): # 2, 3 or 5
                if (all(seg in digit for seg in one)):
                    result += '3'
                elif (all(seg in digit for seg in crochet)):
                    result += '5'
                else:
                    result += '2'
            if (brightness == 6): # 0, 6 or 9
                if (not all(seg in digit for seg in crochet)):
                    result += '0'
                elif (all(seg in digit for seg in one)):
                    result += '9'
                else:
                    result += '6'
    return int(result)


def main():
    daily_list = read_daily_input('input08.txt')
    easy_digits = 0
    for line in daily_list:
        output = False
        for digit in line:
            brightness = len(digit)
            if (brightness == 1):
                output = True
            if output:
                if ((brightness == 2) | (brightness == 3) | (brightness == 4) | (brightness == 7)):
                    easy_digits += 1
    star1 = easy_digits
    print(f"Result (*): {star1}")
    sum_output = 0
    for line in daily_list:
        sum_output += identify_output_number(line)
    star2 = sum_output
    print(f"Result (**): {star2}")


if __name__ == "__main__":
    main()
