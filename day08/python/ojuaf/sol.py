#!/usr/bin/env python3


def get_unique_digits(data, digits):
    digit_sizes = [6, 2, 5, 5, 4, 5, 6, 3, 7, 6]
    for digit in data:
        if len(digit) == digit_sizes[1]:
            digits[1] = digit
        if len(digit) == digit_sizes[4]:
            digits[4] = digit
        if len(digit) == digit_sizes[7]:
            digits[7] = digit
        if len(digit) == digit_sizes[8]:
            digits[8] = digit
        

def load_input():
    data = list()
    with open('input') as fd:
        for line in fd:
            observation, output = line.strip().split('|')
            data.append([observation.strip().split(), output.strip().split()])
    return data


def part1():
    data = load_input()
    digit_sizes = [6, 2, 5, 5, 4, 5, 6, 3, 7, 6]
    
    count = 0
    for observation in data:
        for digit in observation[1]:
            if len(digit) == digit_sizes[1]:
                count += 1
            if len(digit) == digit_sizes[4]:
                count += 1
            if len(digit) == digit_sizes[7]:
                count += 1
            if len(digit) == digit_sizes[8]:
                count += 1
    
    result = count
    print("Part 1: ", result)
    return

def part2():
    # Second part: Just ugly
    data = load_input()
    digits = dict()
    count = 0
    for observation in data:
        known_digits = list()
        get_unique_digits(observation[0], digits)
        get_unique_digits(observation[1], digits)
        
        six_size_digits = [digit for digit in observation[0] if len(digit) == 6]
        for digit in six_size_digits:
            temp = set(digits[4])
            temp.difference_update(set(digit))
            if not temp:
                digits[9] = digit
        six_size_digits.remove(digits[9])
        
        for digit in six_size_digits:
            temp = set(digits[1])
            temp.difference_update(set(digit))
            if not temp:
                digits[0] = digit
        six_size_digits.remove(digits[0])
        
        digits[6] = six_size_digits[0]
        
        five_size_digits = [digit for digit in observation[0] if len(digit) == 5]
        for digit in five_size_digits:
            temp = set(digits[1])
            temp.difference_update(set(digit))
            if not temp:
                digits[3] = digit
        five_size_digits.remove(digits[3])

        for digit in five_size_digits:
            temp = set(digits[6])
            temp.difference_update(set(digit))
            if len(temp) == 1:
                digits[5] = digit
        five_size_digits.remove(digits[5])
        
        digits[2] = five_size_digits[0]
        
        rdigits = dict()
        for i, digit in digits.items():
            rdigits["".join(sorted(digit))] = i
        
        value = str()
        for digit in observation[1]:
            value += str(rdigits["".join(sorted(digit))])
        
        count += int(value)
    
    result = count
    print("Part 2: ", result)
    return

if __name__ == '__main__':
    part1()
    part2()
