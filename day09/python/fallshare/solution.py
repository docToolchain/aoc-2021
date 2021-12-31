file1 = open('input.txt', 'r')

heat_map = list()

for line in file1:
    x_coordinates = list(line.strip())
    heat_map.append(list(map(int, x_coordinates)))

risk_level_sum = 0
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

print(f"Star 1: Risk lebel sum: {risk_level_sum}")
