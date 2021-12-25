
f = open("twentyfive.txt", "r")
lines = [x.strip() for x in f.readlines()]

width = len(lines[0])
height = len(lines)

left = {}
down = {}
for y in range(len(lines)):
    left[y] = []
    down[y] = []
    for x in range(len(lines[y])):
        if lines[y][x] == ">":
            left[y].append(x)
        elif lines[y][x] == "v":
            down[y].append(x)
moved = 1
step = 0
while moved != 0:
    step += 1
    moved = 0
    new_left = {}
    new_down = {}
    for y in left:
        new_left[y] = []
        for x in left[y]:
            nx = x + 1 if x != (width - 1) else 0
            if nx in left[y] or nx in down[y]:
                new_left[y].append(x)
            else:
                moved += 1
                new_left[y].append(nx)
    left = new_left
    for y in down:
        new_down[y] = []
    for y in down:
        for x in down[y]:
            ny = y + 1 if y != (height - 1) else 0
            if x in left[ny] or x in down[ny]:
                new_down[y].append(x)
            else:
                moved += 1
                new_down[ny].append(x)
    down = new_down            
print(step)