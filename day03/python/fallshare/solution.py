
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



