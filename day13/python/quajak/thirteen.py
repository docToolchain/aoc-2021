f = open("thirteen.txt", "r")
lines = [x.strip() for x in f.readlines()]

dots = []

for line in lines:
    if line == "":
        break
    p = line.split(",")
    dots.append((int(p[0]), int(p[1])))
    
folds = [line.split()[2] for line in lines if line.startswith("fold along ")]

fold = folds[0]
dir = fold[0]
line = int(fold[2:])
new_dots = []
for x,y in dots:
    if dir == "y":
        if y > line:
            new_dots.append((x, y - 2 * (y - line)))
        else:
            new_dots.append((x, y))
    elif dir == "x":
        if x > line:
            new_dots.append((x- 2 * (x - line), y))
        else:
            new_dots.append((x, y))
            
print(len(set(new_dots)))


for fold in folds:
    dir = fold[0]
    line = int(fold[2:])
    new_dots = []
    for x,y in dots:
        if dir == "y":
            if y > line:
                new_dots.append((x, y - 2 * (y - line)))
            else:
                new_dots.append((x, y))
        elif dir == "x":
            if x > line:
                new_dots.append((x- 2 * (x - line), y))
            else:
                new_dots.append((x, y))
    dots = list(set(new_dots))

max_x = max([x for x,y in dots]) + 1
max_y = max([y for x,y in dots]) + 1
print(dots)
screen = []
for y in range(max_y):
    screen.append([" "] * max_x)
    
for x,y in dots:
    screen[y][x] = "#"
    
for s in screen:
    print("".join(s))