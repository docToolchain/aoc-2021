import math

input = list()

with open("input.txt", "r") as f:
    for line in f:
        current_line = list(map(int, line.rstrip() ))
        input.append(current_line)


def list_to_int(input_list):
    binary_list = input_list[:]
    number = 0
    binary_list.reverse()
    for i in range(len(binary_list)):
        number += binary_list[i] * (2 ** i)
    return number


msb = list()
lsb = list()
for column in range(len(input[0])):
    one_amount = 0
    for row in range(len(input)):
        if input[row][column] == 1:
            one_amount += 1
    if one_amount >= round(len(input) / 2):
        msb.append(1)
        lsb.append(0)
    else:
        msb.append(0)
        lsb.append(1)

print("Solution Star 1:")
print(f"Gamma rate: {list_to_int(msb)}")
print(f"Epsilon rate: {list_to_int(lsb)}")
print(f"Power consumption: {list_to_int(msb) * list_to_int(lsb)}")
print("---------------------------")
print("Solution Star 2:")

remaining_numbers = input.copy()
for column in range(len(input[0])):
    #determine msb of renaming numbers at position column
    most_commonvalue = 0
    least_commonvalue = 0

    one_amount = 0
    for row in range(len(remaining_numbers)):
        if remaining_numbers[row][column] == 1:
            one_amount += 1
    if one_amount >= round(len(remaining_numbers) / 2):
        most_commonvalue = 1
        least_commonvalue = 0
    else:
        most_commonvalue = 0
        least_commonvalue = 1
     
    #add numbers that match bit criteria to remaining numbers
    matching_numbers = list()
    for row in range(len(remaining_numbers)):
        if remaining_numbers[row][column] == most_commonvalue:
            matching_numbers.append(remaining_numbers[row])
    
    remaining_numbers = matching_numbers

oxygen_gen_rate = list_to_int(remaining_numbers[0])




remaining_numbers = input.copy()
for column in range(len(input[0])):
    #determine msb of renaming numbers at position column
    most_commonvalue = 0
    least_commonvalue = 0
    one_amount = 0
    for row in range(len(remaining_numbers)):
        if remaining_numbers[row][column] == 1:
            one_amount += 1
    print(f"{one_amount} - {math.ceil(len(remaining_numbers) / 2)}")
    if one_amount >= math.ceil(len(remaining_numbers) / 2):
        most_commonvalue = 1
        least_commonvalue = 0
    else:
        most_commonvalue = 0
        least_commonvalue = 1
    print(remaining_numbers)
    print(least_commonvalue)
    #add numbers that match bit criteria to remaining numbers
    matching_numbers = list()
    for row in range(len(remaining_numbers)):
        if remaining_numbers[row][column] == least_commonvalue:
            matching_numbers.append(remaining_numbers[row])
    
    remaining_numbers = matching_numbers

    if len(remaining_numbers) == 1:
        break
scrubbing_rate = list_to_int(remaining_numbers[0])

print(f"Oxygen generator rate: {oxygen_gen_rate}")
print(f"CO2 scrubber rate: {scrubbing_rate}")
print(f"Life support rate: {oxygen_gen_rate * scrubbing_rate}")

