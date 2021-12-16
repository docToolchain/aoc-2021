
#!/usr/bin/env python3

import re
from collections import deque
import math as m            


pattern = re.compile("(\w\w) -> (\w)")


def load_input():
    data = ""
    with open('input') as fd:
        temp = fd.read().strip()
        # to bin
        for char in temp:
            data += bin(int(char, 16))[2:].zfill(4)
    return data


def parse_header(value):
    version = int(value[0:3], 2)
    type_id = int(value[3:6], 2)
    return version, type_id, 6

def parse_literal(value):
    count = 0
    last = False
    result = ""
    while True:
        if value[count] == '0':
            last = True
        result += value[count+1:count+5]
        count += 5
        if last:
            break
    return int(result, 2), count

def parse_operator(data):
    count = 0
    size = 0
    length = None
    number_subpackages = None
    if data[0] == '0':
        length = int(data[1:16], 2)
        size = 16
    elif data[0] == '1':
        number_subpackages = int(data[1:12], 2)
        size = 12
    else:
        raise RuntimeError("Invalid Bit")
    return length, number_subpackages, size


def apply_operator(type_id, inputs):
    if type_id == 0:
        value = sum(inputs)
    elif type_id == 1:
        value = m.prod(inputs)
    elif type_id == 2:
        value = min(inputs)
    elif type_id == 3:
        value = max(inputs)
    elif type_id == 5:
        value = 1 if inputs[0] > inputs[1] else 0
    elif type_id == 6:
        value = 1 if inputs[0] < inputs[1] else 0
    elif type_id == 7:
        value = 1 if inputs[0] == inputs[1] else 0
    else:
        raise RuntimeError("No proper type")
    return value


def process(data, number_packages=None):
    version_sum = 0
    count = 0
    package_count = 0
    values = list()
    while True:
        package_count += 1
        version, type_id, size = parse_header(data[count:])
        version_sum += version
        count += size
        if type_id == 4:
            lit_val, size = parse_literal(data[count:])
            values.append(lit_val)
        else:
            length, number_subpackages, size = parse_operator(data[count:])
            count += size
            inputs = list()
            if length is not None:
                size, v_sum, inputs = process(data[count:count+length])
            elif number_subpackages is not None:
                size, v_sum, inputs = process(data[count:], number_subpackages)

            version_sum += v_sum
            value = apply_operator(type_id, inputs)
            values.append(value)
        
        count += size

        if len(data[count:]) < 10:            
            break
        if number_packages is not None and package_count >= number_packages:
            break

    return count, version_sum, values

def part1():
    # First Part
    data = load_input()

    count, version_sum, values = process(data)
    
    result = version_sum
    print("Part 1:", result)
    print("Part 2", values[0])
    return


if __name__ == '__main__':
    part1()
    # part2()
