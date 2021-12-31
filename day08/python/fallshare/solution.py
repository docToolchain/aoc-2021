file1 = open('input.txt', 'r')

# Star 1
unique_digits = 0
for line in file1:
    output_part = line.split("|")[1]
    output_digits = output_part.split()

    for digit in output_digits:
        segment_cnt = len(digit)
        if (segment_cnt == 2) or (segment_cnt == 4) or (segment_cnt == 3) or (segment_cnt == 7):
            unique_digits += 1

print(f"Star 1: {unique_digits} unique digits")

# Star 2
file1 = open('input.txt', 'r')
output_value_sum = 0
for line in file1:

    signals = line.split("|")
    signal_pattern = signals[0].split()
    output_pattern = signals[1].split()

    digits = dict()

    # Find Digit 1
    for signal in signal_pattern:
        if len(signal) == 2:
            digits[1] = set(signal)
            break
    # Find Digit 4
    for signal in signal_pattern:
        if len(signal) == 4:
            digits[4] = set(signal)
            break
    # Find Digit 7
    for signal in signal_pattern:
        if len(signal) == 3:
            digits[7] = set(signal)
            break
    # Find Digit 8
    for signal in signal_pattern:
        if len(signal) == 7:
            digits[8] = set(signal)
            break             
    # Find digit 5
    overlap = digits[4] - digits[1]
    for signal in signal_pattern:
        if (len(signal) == 5) and (overlap.issubset(set(signal)) ):
            digits[5] = set(signal)

    # Find digit 2
    overlap = digits[8] - digits[5] # 2 ist the oposit of 5. So 2 must contain remaining elemnts that are not in 5
    for signal in signal_pattern:
        if (len(signal) == 5) and (overlap.issubset(set(signal)) ):
            digits[2] = set(signal)

    # Find digit 3
    for signal in signal_pattern:
        if (len(signal) == 5) and not (set(signal).issubset(digits[2])) and not (set(signal).issubset(digits[5])):
            digits[3] = set(signal)

    # Find digit 9
    for signal in signal_pattern:
        if (len(signal) == 6) and (digits[4].issubset((set(signal)))): # 4 fits only into 9 (of the digits with 6 characters)
            digits[9] = set(signal)

    # Find digit 6
    for signal in signal_pattern:
        if (len(signal) == 6) and not(digits[1].issubset((set(signal)))): 
            digits[6] = set(signal)

    # Find digit 0
    for signal in signal_pattern:
        if (len(signal) == 6) and not (set(signal).issubset(digits[6])) and not (set(signal).issubset(digits[9])):
            digits[0] = set(signal)

    number_string = ""
    for digit in output_pattern:
        digit_string = list(digits.keys())[list(digits.values()).index(set(digit))]
        number_string = number_string + str(digit_string)
        
    output_value_sum += int(number_string)


print(f"Star 2: Total sum is {output_value_sum}")