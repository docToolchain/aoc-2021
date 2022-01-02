import numpy as np

file1 = open('input.txt', 'r')

lines = list()

for line in file1:
    lines.append(list(map(int,list(line.strip()))))

grid = np.zeros((10,10), dtype=int)
for y in range(10):
    row = list(map(int,list(lines[y])))
    grid[y] = np.asarray(row, dtype=np.int8)

print(grid)

def power_up_neighbours(x,y,grid,flashed_octopusses):
    for dx in (-1, 0, 1):
        for dy in (-1, 0, 1):
            x_n = x + dx
            y_n = y + dy
            if (x_n >= 0 and x_n < 10 ) and (y_n >= 0 and y_n < 10 ):
                if(not flashed_octopusses[y_n][x_n]):
                    grid[y_n][x_n] += 1

flash_count = 0
for iteration in range(0,100):
    flashed_octopusses = np.zeros((10,10), dtype=int)
    #increase engery level by one
    for y in range(0, len(grid)):
        for x in range(0, len(grid[0])):
            grid[y][x] += 1
    #let octopusses lash
    while((grid > 9).sum() != 0):
        for y in range(0, len(grid)):
            for x in range(0, len(grid[0])):
                if grid[y][x] > 9:
                    flashed_octopusses[y][x] = True
                    flash_count += 1
                    grid[y][x] = 0
                    power_up_neighbours(x,y,grid,flashed_octopusses)
    #print(grid)

print(f"Star 1: There have been {flash_count} flashes after 100 itterations")