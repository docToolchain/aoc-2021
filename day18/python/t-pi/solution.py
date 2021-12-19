# see README.adoc

import math
import re
import copy
from pprint import pprint

def read_daily_input(filename):
    ''' Read lines from file with given input name
        cast to daily required type and return list
    '''
    with open(filename) as input_file:
        local_list = [item.strip() for item in input_file.readlines()]
    return local_list

def eval_string(sno):
    ''' Evaluate directly the string
    '''
    if (re.search('[a-zA-Z]', sno) != None): # limit risks with eval() below
        return sno
    while not finished:
        bracket_level = 0
        last_num_index = 0
        last_digit = ''
        finished = True
        print(sno)
        for i, c in enumerate(sno):
            if (c == '['):
                bracket_level += 1
                if (bracket_level == 5): # explode
                    close_i = sno[i:].find(']')
                    bracket = eval(sno[i, i+close_i+1])
                    print(bracket)
                    if (last_num_index != 0):
                        left_no = sno[:last_num_index] + str(int(last_digit)+int(bracket[0])) + '0'
                    else:
                        left_no = sno[:i] + '0'
                    
                    
                        
                    sno = left_no + '0' + f"[{lefty},{righty}]" + sno[i+1:]
                    finished = False
                    break
            if (c == ']'):
                bracket_level -= 1
        for i, c in enumerate(sno):
            if c.isdigit():
                if (last_num_index == i - 1): # split
                    value = int(last_digit + c)
                    lefty = str(math.floor(value/2))
                    righty = str(round(value/2))
                    sno = sno[:i-1] + f"[{lefty},{righty}]" + sno[i+1:]
                    finished = False
                    break
                last_num_index = i
                last_digit = c



def evaluate_expression(expression, verbose = True):
    ''' Evaluate single snailfish number for explode or split
        Return new expression
    '''
    def add_to(element, addition, index = -1):
        if isinstance(element, int):
            element += addition
            return element
        else:
            element[index] = add_to(element[index], addition, index)
        return element
    
    def check_splitfree(my_list, splitfree = True):
        if not splitfree:
            return my_list, splitfree
        for i, item in enumerate(my_list):
            if isinstance(item, list):
                item, splitfree = check_splitfree(item, splitfree)
                if not splitfree:
                    return my_list, splitfree
            elif item > 9:
                my_list[i] = [math.floor(float(item)/2), math.ceil(float(item)/2)]
                return my_list, False
        return my_list, splitfree

    finishable = False
    if verbose: 
        print(expression)
    while not finishable:
        explode_buffer = 0
        finishable = True
        for i1, item1 in enumerate(expression):
            if isinstance(item1,list): # [[a, b], c] 
                for i2, item2 in enumerate(item1):
                    if isinstance(item2,list): # [[[a, b], c], d]
                        for i3, item3 in enumerate(item2):
                            if isinstance(item3,list): # [[[[a, b], c], d], e]
                                for i4, item4 in enumerate(item3):
                                    if isinstance(item4, list) and finishable: # explode
                                        finishable = False
                                        if (i4 != 0):
                                            item3[i4-1] = add_to(item3[i4-1], item4[0])
                                        elif (i3 != 0):
                                            item2[i3-1] = add_to(item2[i3-1], item4[0])
                                        elif (i2 != 0):
                                            item1[i2-1] = add_to(item1[i2-1], item4[0])
                                        elif (i1 != 0):
                                            expression[i1-1] = add_to(expression[i1-1], item4[0])
                                        # print('explode1:', expression)
                                        explode_buffer = item4[1]
                                        item3[i4] = 0
                                    elif not finishable:
                                        item3[i4] = add_to(item3[i4], explode_buffer, 0)
                                        explode_buffer = 0
                                        break
                            elif not finishable:
                                item2[i3] = add_to(item2[i3], explode_buffer, 0)
                                explode_buffer = 0
                                break
                    elif not finishable:
                        item1[i2] = add_to(item1[i2], explode_buffer, 0)
                        explode_buffer = 0
                        break
            elif not finishable:
                expression[i1] = add_to(expression[i1], explode_buffer, 0)
                explode_buffer = 0
                break

        if verbose and not finishable:
            print('explode2: ', expression)
        if finishable:
            expression, finishable = check_splitfree(expression, finishable)
            # if max_value > 9:
            #     finishable = False
            if verbose: 
                print('split: ', expression)
    return expression
                                    

def get_magnitude(expression):
    ''' Calculate magnitude
    '''
    left_exp, right_exp = expression
    lefty = 3 * left_exp if isinstance(left_exp, int) else 3 * get_magnitude(left_exp)
    righty = 2 * right_exp if isinstance(right_exp, int) else 2 * get_magnitude(right_exp)
    return lefty + righty        


def main():
    calc_sheet = read_daily_input('input18.txt')
    # calc_sheet = ['[[[[2, [1,12]],4], 3], 6]']
    # calc_sheet  = ['[[[[0, [5, 8]], [[1, 7], [9, 6]]], [[4, [1, 2]], [[1, 4], 2]]],2]']
    # pprint(calc_sheet)
    calculus = []
    for sno_line in calc_sheet:
        if (re.search('[a-zA-Z]', sno_line) == None): # basic eval() protection
            expression = list(eval(sno_line))
            calculus.append(evaluate_expression(expression, False))
    result = calculus[0]
    for i, sno in enumerate(calculus[1:]):
        # print([result] + [sno])
        result = evaluate_expression([result] + [sno], False)
        # print('---------------------------\nresult: ', result)
    star1 = get_magnitude(result)
    print(f"Result (*): {star1}")
    mags = []
    span = len(calc_sheet)
    for i in range(span):
        for j in range(i+1, span):
            if (re.search('[a-zA-Z]', calc_sheet[i]) == None): # basic eval() protection
                lefty = list(eval(calc_sheet[i]))
            if (re.search('[a-zA-Z]', calc_sheet[j]) == None): # basic eval() protection
                righty = list(eval(calc_sheet[j]))
            input1 = [lefty] + [righty]
            buffer1 = evaluate_expression(input1.copy(), False)
            mags.append(get_magnitude(buffer1))

            if (re.search('[a-zA-Z]', calc_sheet[i]) == None): # basic eval() protection
                lefty = list(eval(calc_sheet[i]))
            if (re.search('[a-zA-Z]', calc_sheet[j]) == None): # basic eval() protection
                righty = list(eval(calc_sheet[j]))
            input2 = [righty] + [lefty] 
            buffer2 = evaluate_expression(input2.copy(), False)
            mags.append(get_magnitude(buffer2))
    star2 = max(mags)
    print(f"Result (**): {star2}")


if __name__ == "__main__":
    main()
