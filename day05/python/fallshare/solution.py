import numpy as np

file1 = open('input.txt', 'r')

lines = list()
for line in file1:
    string_coordinates = line.rstrip().replace(" -> ", ",").split(",")
    coordinates = list(map(int, string_coordinates))
    lines.append(coordinates)

biggest_x = 0
for i in range(len(lines)):
    if lines[i][0] > biggest_x:
        biggest_x = lines[i][0]
    if lines[i][2] > biggest_x:
        biggest_x = lines[i][2]
print("Biggest X: {}".format( biggest_x))

biggest_y = 0
for i in range(len(lines)):
    if lines[i][1] > biggest_y:
        biggest_y = lines[i][1]
    if lines[i][3] > biggest_y:
        biggest_y = lines[i][3]
print("Biggest Y: {}".format( biggest_y))

vent_map = np.zeros((biggest_x + 1,biggest_y + 1), dtype=int)
for i in range(len(lines)):
    
    x_start = lines[i][0]
    x_end = lines[i][2]
    y_start = lines[i][1]
    y_end = lines[i][3]
    
    if x_start == x_end : #horizontal line?
        if y_start > y_end:
            y_start, y_end = y_end, y_start        
        for y in range(y_start, y_end + 1):
            vent_map[y][x_start] += 1
    elif y_start == y_end: #vertical line?
        if x_start > x_end:
            x_start, x_end = x_end, x_start
        for x in range(x_start, x_end + 1):
            vent_map[y_start][x] += 1

count = np.count_nonzero(vent_map > 1)
print("Solution Start 1:")
print(f"Points where at least two lines cross: {count}")


# Star 2
vent_map = np.zeros((biggest_x + 1,biggest_y + 1), dtype=int)
for i in range(len(lines)):
    x_start = lines[i][0]
    x_end = lines[i][2]
    y_start = lines[i][1]
    y_end = lines[i][3]
    
    if x_start == x_end : #horizontal line?
        if y_start > y_end:
            y_start, y_end = y_end, y_start        
        for y in range(y_start, y_end + 1):
            vent_map[y][x_start] += 1
    elif y_start == y_end: #vertical line?
        if x_start > x_end:
            x_start, x_end = x_end, x_start
        for x in range(x_start, x_end + 1):
            vent_map[y_start][x] += 1
    else:
        if x_start > x_end:
            x_start, x_end = x_end, x_start
            y_start, y_end = y_end, y_start       

        y_direction = 1
        if y_start > y_end:
            y_direction = -1            

        y = y_start

        for x in range(x_start, x_end + 1):

            vent_map[y][x] += 1
            y += y_direction

count = np.count_nonzero(vent_map > 1)
print("Solution Start 2:")
print(f"Points where at least two lines cross: {count}")