file1 = open('input.txt', 'r')
import sys
print(sys.getrecursionlimit())
sys.setrecursionlimit(10000)
heat_map = list()

for line in file1:
    x_coordinates = list(line.strip())
    heat_map.append(list(map(int, x_coordinates)))

risk_level_sum = 0
low_points = list()
for y in range(len(heat_map)):
    for x in range(len(heat_map[0])):
        #print(f"x:{x} , y:{y}, value: {heat_map[y][x]}")
        smallest_number = True
        #hoch
        if (x > 0) and (heat_map[y][x] >= heat_map[y][x - 1]):
            smallest_number = False
        #runter
        if (x < (len(heat_map[0])-1)) and (heat_map[y][x] >= heat_map[y][x + 1]):
            smallest_number = False
        #links
        if (y > 0) and (heat_map[y][x] >= heat_map[y - 1][x]):
            smallest_number = False
        #rechts
        if (y < (len(heat_map)-1)) and (heat_map[y][x] >= heat_map[y + 1][x]):
            smallest_number = False 

        if smallest_number:
            print(f"Low point found at: {x}-{y}")
            risk_level_sum += (heat_map[y][x] + 1)
            low_points.append((x,y))

print(f"Star 1: Risk level sum: {risk_level_sum}")


def basin_size(x, y, heat_map):
    locations = {(x, y)}
    #links
    if (x > 0) and (heat_map[y][x] <= heat_map[y][x - 1]) and (heat_map[y][x - 1] != 9):
        locations.update(basin_size((x - 1), y, heat_map))
    #rechts
    if (x < (len(heat_map[0])-1)) and (heat_map[y][x] <= heat_map[y][x + 1]) and (heat_map[y][x + 1] != 9):
        locations.update(basin_size((x + 1), y, heat_map))        
    #hoch
    if (y > 0) and (heat_map[y][x] <= heat_map[y - 1][x]) and ( heat_map[y - 1][x] != 9):
        locations.update(basin_size(x, (y - 1), heat_map))
    #runter
    if (y < (len(heat_map)-1)) and (heat_map[y][x] <= heat_map[y + 1][x])  and (heat_map[y + 1][x] != 9):
        locations.update(basin_size(x, (y + 1), heat_map))    
    return locations


basin_sizes = list()
for low_point in low_points: 
    basin_size_cnt = len(basin_size(low_point[0],low_point[1],heat_map))
    basin_sizes.append(basin_size_cnt)

basin_sizes.sort()    
largest_basins = basin_sizes[-3:]

basin_product = 1
for basin in largest_basins:
    basin_product *= basin

print(f"Star 2: product of largest basins is: {basin_product}")