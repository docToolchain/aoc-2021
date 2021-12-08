f = open("./three.txt", "r")
lines = [x.strip() for x in f.readlines()]


gamma = ""
for index in range(len(lines[0])):
    bits = [line[index] for line in lines]
    zeros = list(bits).count('0')
    ones = list(bits).count('1')
    if zeros > ones:
        gamma += "0"
    else:
        gamma += "1"

epsilon = ""
for index in range(len(lines[0])):
    bits = [line[index] for line in lines]
    zeros = list(bits).count('0')
    ones = list(bits).count('1')
    if zeros < ones:
        epsilon += "0"
    else:
        epsilon += "1"

print("Star 1", int(epsilon, 2) * int(gamma, 2))

oxygen_valid = list(lines)

index = 0
while len(oxygen_valid) > 1:
    bits = [line[index] for line in oxygen_valid]
    zeros = list(bits).count('0')
    ones = list(bits).count('1')
    if zeros > ones:
        oxygen_valid = [line for line in oxygen_valid if line[index] == "0"]
    else:
        oxygen_valid = [line for line in oxygen_valid if line[index] == "1"]
    index += 1
        
        
co = list(lines)

index = 0
while len(co) > 1:
    bits = [line[index] for line in co]
    zeros = list(bits).count('0')
    ones = list(bits).count('1')
    if zeros <= ones:
        co = [line for line in co if line[index] == "0"]
    else:
        co = [line for line in co if line[index] == "1"]
    index += 1
        
print("Star 2", int(oxygen_valid[0], 2) * int(co[0], 2))