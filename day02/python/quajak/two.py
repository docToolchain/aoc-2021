f = open("./two.txt", "r")
lines = [x.strip() for x in f.readlines()]

x = 0
y = 0
for line in lines:
    value = int(line.split(" ")[1])
    if "forward" in line:
        x += value
    elif "down" in line:
        y += value
    else:
        y -= value
        
print("Star 1", x * y)

aim = 0
x = 0
y = 0
for line in lines:
    value = int(line.split(" ")[1])
    if "forward" in line:
        x += value
        y += value * aim
    elif "down" in line:
        aim += value
    else:
        aim -= value
        
print("Star 2", x * y)