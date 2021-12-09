f = open("five.txt", "r")
lines = [x.strip() for x in f.readlines()]

space = {}

for line in lines:
    words = line.split(" ")
    start = [int(w) for w in words[0].split(",")]
    end = [int(w) for w in words[2].split(",")]
    
    if start[0] == end[0]:
        for i in range(min(start[1], end[1]), max(start[1], end[1]) + 1):
            if not start[0] in space:
                space[start[0]] = {i: 1}
            else:
                if i not in space[start[0]]:
                    space[start[0]][i] = 1
                else:
                    space[start[0]][i] += 1
    elif start[1] == end[1]:
        for i in range(min(start[0], end[0]), max(start[0], end[0]) + 1):
            if not i in space: 
                space[i] = {start[1]: 1}
            else:
                if start[1] not in space[i]:
                    space[i][start[1]] = 1
                else:
                    space[i][start[1]] += 1
                
points = 0
for x in space.keys():
    for y in space[x].keys():
        if space[x][y] > 1:
            points += 1
            
print("Star 1", points)

for line in lines:
    words = line.split(" ")
    start = [int(w) for w in words[0].split(",")]
    end = [int(w) for w in words[2].split(",")]
    
    if start[0] == end[0]:
        pass # we already handled these cases in the first star
    elif start[1] == end[1]:
        pass 
    else:
        x_dir = 1 if start[0] < end[0] else -1
        y_dir = 1 if start[1] < end[1] else -1
        for (x,y) in zip(range(start[0], end[0] + x_dir, x_dir), range(start[1], end[1] + y_dir, y_dir)):
            if not x in space:
                space[x] = {}
            if not y in space[x]:
                space[x][y] = 1
            else:
                space[x][y] += 1
       
points = 0
for x in space.keys():
    for y in space[x].keys():
        if space[x][y] > 1:
            points += 1
            
print("Star 2", points)