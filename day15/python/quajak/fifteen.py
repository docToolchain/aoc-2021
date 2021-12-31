f = open("fifteen.txt", "r")
lines = [x.strip() for x in f.readlines()]

risk = []
costs = []
for y in range(5):
    for line in lines:
        r = []
        co = []
        for x in range(5):
            r.extend([(int(c) + x + y) if int(c) + x + y < 10 else int(c) + x + y - 9 for c in line])
            co.extend([100000 for c in line])
        risk.append(r)
        costs.append(co)
    
costs[0][0] = 0
to_check = [(0,0)]

def check_field(x,y, base_cost):
    return x >= 0 and y >= 0 and x != len(risk[0]) and y != len(risk) and base_cost + risk[y][x] < costs[y][x]

while len(to_check) != 0:
    x,y = to_check.pop(0)
    cost = costs[y][x]
    if check_field(x-1, y, cost):
        costs[y][x-1] = cost + risk[y][x-1]
        to_check.append((x-1, y))
    if check_field(x+1, y, cost):
        costs[y][x+1] = cost + risk[y][x+1]
        to_check.append((x+1, y))
    if check_field(x, y-1, cost):
        costs[y-1][x] = cost + risk[y-1][x]
        to_check.append((x, y-1))
    if check_field(x, y+1, cost):
        costs[y+1][x] = cost + risk[y+1][x]
        to_check.append((x, y+1))
        
print(costs[-1][-1])