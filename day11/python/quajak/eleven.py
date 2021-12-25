f = open("eleven.txt", "r")
lines = [x.strip() for x in f.readlines()]

field = []
for line in lines:
    field.append([0] + [int(ch) for ch in line] + [0])
    
field.insert(0, [0] * len(field[0]))
field.append([0] * len(field[0]))


width = len(field[0])
height = len(field)

flashes = 0

def flash(ax, ay):
    global flashes
    if (ax, ay) in flashed:
        return
    if ax == 0 or ax == width - 1:
        return
    if ay == 0 or ay == height - 1:
        return
    flashed.append((ax,ay))
    flashes += 1
    for dy in [-1, 0, 1]:
        for dx in [-1, 0, 1]:
            if dy == dx == 0:
                continue
            field[ay + dy][ax + dx] += 1
            if field[ay + dy][ax + dx] > 9:
                flash(ax + dx, ay + dy)

for _ in range(100):
    flashed = []
    for y in range(1, height - 1):
        for x in range(1, width - 1):
            field[y][x] += 1
            if field[y][x] > 9:
                flash(x,y)
                
    for x,y in flashed:
        field[y][x] = 0
        
print("Star 1", flashes)    

    
turn = 0
while True:
    flashed = []
    for y in range(1, height - 1):
        for x in range(1, width - 1):
            field[y][x] += 1
            if field[y][x] > 9:
                flash(x,y)
                
    for x,y in flashed:
        field[y][x] = 0
        
    turn += 1
    if len(flashed) == (width - 2) * (height - 2):
        break

print("Star 2", turn)