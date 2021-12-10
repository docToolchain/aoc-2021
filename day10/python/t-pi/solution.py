# see description.adoc

import io
from pprint import pprint

def read_daily_input(filename):
    ''' Read lines from file with given input name
        cast to daily required type and return list
    '''
    with open(filename) as input_file:
        local_list = [item.strip() for item in input_file.readlines()]
        return local_list


def parse_brackets(command_line):
    ''' parse bracket types and throw return upon incorrect closing type
        Return values:        
        ): 3 points.
        ]: 57 points.
        }: 1197 points.
        >: 25137 points.
        else 0
    '''
    old_len = len(command_line)+1
    while old_len > len(command_line):
        old_len = len(command_line)
        command_line = command_line.replace("()", "")
        command_line = command_line.replace("[]", "")
        command_line = command_line.replace("{}", "")
        command_line = command_line.replace("<>", "")
    bracket_pos = [command_line.find(c) for c in [')',']','}','>']]
    bracket_pos = [999 if item == -1 else item for item in bracket_pos]
    min_pos = min(bracket_pos)
    if (all(elem == min_pos for elem in bracket_pos)):
        return 0, command_line
    if (bracket_pos.index(min_pos) == 0):
        return 3, command_line
    elif (bracket_pos.index(min_pos) == 1):
        return 57, command_line
    elif (bracket_pos.index(min_pos) == 2):
        return 1197, command_line
    elif (bracket_pos.index(min_pos) == 3):
        return 25137, command_line
    return 0, command_line


def count_brackets(command_line):
    ''' count bracket types from back to forth and calculate score
        ): 1 points.
        ]: 2 points.
        }: 3 points.
        >: 4 points.
    '''
    score = 0
    for i in range(len(command_line)-1, -1, -1):
        c = command_line[i]
        if (c == '('):
            score = score*5 + 1
        elif (c == '['):
            score = score*5 + 2
        elif (c == '{'):
            score = score*5 + 3
        elif (c == '<'):
            score = score*5 + 4
    return score
    

def main():
    daily_list = read_daily_input('input10.txt')
    syntax_sum = 0
    incomplete_lines = []
    for line in daily_list:
        syntax_value, parsed_line = parse_brackets(line)
        if (syntax_value == 0):
            incomplete_lines.append(parsed_line)
        else:
            syntax_sum += syntax_value
    star1 = syntax_sum
    print(f"Result (*): {star1}")
    scores = []
    for line in incomplete_lines:
        scores.append(count_brackets(line))
    scores.sort()
    star2 = scores[round(len(scores)/2)-1]
    print(f"Result (**): {star2}")


if __name__ == "__main__":
    main()
