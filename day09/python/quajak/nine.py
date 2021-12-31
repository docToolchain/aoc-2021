f = open("nine.txt", "r")
lines = [x.strip() for x in f.readlines()]

height = len(lines)
width = len(lines[0])

map = []

for line in lines:
    cur = [int(ch) for ch in list(line)]
    map.append(cur)
    
risk = 0

for y in range(height):
    for x in range(width):
        val = map[y][x]
        if y !=0 and map[y-1][x] <= val:
            continue
        if y != height - 1 and map[y+1][x] <= val:
            continue
        if x != 0 and map[y][x-1] <= val:
            continue
        if x != width - 1 and map[y][x+1] <= val:
            continue
        risk += map[y][x] + 1
        
print(risk)

height = len(lines)
width = len(lines[0])

map = []

for line in lines:
    cur = [int(ch) for ch in list(line)]
    map.append(cur)
    
basins = []
low_points = []

for y in range(height):
    for x in range(width):
        val = map[y][x]
        if y !=0 and map[y-1][x] <= val:
            continue
        if y != height - 1 and map[y+1][x] <= val:
            continue
        if x != 0 and map[y][x-1] <= val:
            continue
        if x != width - 1 and map[y][x+1] <= val:
            continue
        low_points.append((x,y))

for (x,y) in low_points:    
    size = 0
    # low point grow outwards
    to_check = [(x,y)]
    while len(to_check) != 0:
        pos = to_check.pop(0)
        x,y = pos
        val = map[y][x]
        if val == 9:
            continue
        size += 1
        
        if y !=0 and map[y-1][x] > val and map[y-1][x] != 9:
            to_check.append((x,y-1))
        if y != height - 1 and map[y+1][x] > val and map[y+1][x] != 9:
            to_check.append((x, y + 1))
        if x != 0 and map[y][x-1] > val and map[y][x-1] != 9:
            to_check.append((x-1, y))
        if x != width - 1 and map[y][x+1] > val and map[y][x+1] != 9:
            to_check.append((x+1,y))
        map[y][x] = 9
        
        
    basins.append(size)
            
basins = sorted(basins, reverse=True)
print(basins[0] * basins[1] * basins[2])
        