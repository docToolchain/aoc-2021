def process_input(file_contents):
    lines_stripped = [line.strip() for line in file_contents]
    
    floor_plan = [[int(i) for i in list(line)] for line in lines_stripped]

    return floor_plan

def extend_basin(point,map_floor,basin):
    if point[0] != 0:
        if map_floor[point[0]-1][point[1]] != 9:
            if (point[0]-1,point[1]) not in basin:
                basin.add((point[0]-1,point[1]))
                basin = extend_basin((point[0]-1,point[1]),map_floor,basin)
    if point[0] != len(map_floor)-1:
        if map_floor[point[0]+1][point[1]] != 9:
            if (point[0]+1,point[1]) not in basin:
                basin.add((point[0]+1,point[1]))
                basin = extend_basin((point[0]+1,point[1]),map_floor,basin)
    if point[1] != 0:
        if map_floor[point[0]][point[1]-1] != 9:
            if (point[0],point[1]-1) not in basin:
                basin.add((point[0],point[1]-1))
                basin = extend_basin((point[0],point[1]-1),map_floor,basin)
    if point[1] != len(map_floor[0])-1:
        if map_floor[point[0]][point[1]+1] != 9:
            if (point[0],point[1]+1) not in basin:
                basin.add((point[0],point[1]+1))
                basin = extend_basin((point[0],point[1]+1),map_floor,basin)

    return basin

def main():
    with open("input.txt",'r') as floor_file:
        floor_lines = floor_file.readlines()

    map_floor = process_input(floor_lines)

    #star 1
    low_points = []
    sum_risk = 0
    for i in range(len(map_floor)):
        for j in range(len(map_floor[i])):
            if i == 0:
                if j == 0:
                    if map_floor[i][j]<map_floor[i][j+1] and map_floor[i][j]<map_floor[i+1][j]:
                        sum_risk += 1+map_floor[i][j]
                        low_points.append((i,j))
                elif j == len(map_floor[i])-1:
                    if map_floor[i][j]<map_floor[i][j-1] and map_floor[i][j]<map_floor[i+1][j]:
                        sum_risk += 1+map_floor[i][j]
                        low_points.append((i,j))
                elif map_floor[i][j]<map_floor[i][j+1] and map_floor[i][j]<map_floor[i+1][j] and map_floor[i][j]<map_floor[i][j-1]:
                    sum_risk += 1+map_floor[i][j]
                    low_points.append((i,j))
            elif i == len(map_floor)-1:
                if j == 0:
                    if map_floor[i][j]<map_floor[i][j+1] and map_floor[i][j]<map_floor[i-1][j]:
                        sum_risk += 1+map_floor[i][j]
                        low_points.append((i,j))
                elif j == len(map_floor[i])-1:
                    if map_floor[i][j]<map_floor[i][j-1] and map_floor[i][j]<map_floor[i-1][j]:
                        sum_risk += 1+map_floor[i][j]
                        low_points.append((i,j))
                elif map_floor[i][j]<map_floor[i][j+1] and map_floor[i][j]<map_floor[i-1][j] and map_floor[i][j]<map_floor[i][j-1]:
                    sum_risk += 1+map_floor[i][j]
                    low_points.append((i,j))
            elif j == 0:
                if map_floor[i][j]<map_floor[i][j+1] and map_floor[i][j]<map_floor[i+1][j] and map_floor[i][j]<map_floor[i-1][j]:
                    sum_risk += 1+map_floor[i][j]
                    low_points.append((i,j))
            elif j == len(map_floor[i])-1:
                if map_floor[i][j]<map_floor[i][j-1] and map_floor[i][j]<map_floor[i-1][j] and map_floor[i][j]<map_floor[i+1][j]:
                    sum_risk += 1+map_floor[i][j]
                    low_points.append((i,j))
            elif map_floor[i][j]<map_floor[i][j+1] and map_floor[i][j]<map_floor[i+1][j] and map_floor[i][j]<map_floor[i-1][j] and map_floor[i][j]<map_floor[i][j-1]:
                    sum_risk += 1+map_floor[i][j]
                    low_points.append((i,j))
                        
    print(sum_risk)
 
    #star 2
    size_basin = []
    for i in low_points:
        basin = set()
        basin.add(i)
        basin = extend_basin(i,map_floor,basin)
        size_basin.append(len(basin))
    size_basin.sort(reverse=True)
    
    print(size_basin[0]*size_basin[1]*size_basin[2])
        
main()
