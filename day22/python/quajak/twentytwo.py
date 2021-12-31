f = open("twentytwo.txt", "r")
lines = [x.strip() for x in f.readlines()]

commands = []

for line in lines:
    p = line.split(" ")
    dims = [[int(val) for val in dim.split("=")[1].split("..")] for dim in p[1].split(",")]
    dims[0][1] += 1
    dims[1][1] += 1
    dims[2][1] += 1
    commands.append((0 if p[0] == "off" else 1, dims))


def intersects(range1, range2):
    return range2[0] <= range1[1] and range1[0] <= range2[1]
    
def valid(x, y, z):
    return min(x) > 0 and min(y) > 0 and min(z) > 0

def line_split(l1, l2, val1, val2):
    res = []
    if l1[0] < l2[0]:
        res.append([val1, [l1[0], l2[0]]])
    res.append([val2, [max(l1[0], l2[0]), min(l1[1], l2[1])]])
    if l1[1] > l2[1]:
        res.append([val1, [l2[1], l1[1]]])
    return res

def split(val1, x1, y1, z1, val2, x2, y2, z2):
    res = []
    x_split = line_split(x1, x2, val1, val2)
    y_split = line_split(y1, y2, val1, val2)
    z_split = line_split(z1, z2, val1, val2)
    
    for x in x_split:
        for y in y_split:
            for z in z_split:
                val = val2 if x[0] == y[0] == z[0] == val2 else val1 
                res.append([val, [x[1], y[1], z[1]]])
    return res

regions = [[0, [[-10000000, 10000001], [-10000000, 10000001], [-10000001, 10000001]]]]
for command, cube in commands:
    x = cube[0]
    y = cube[1]
    z = cube[2]
    new_regions = []
    for region in regions:
        if region[0] != command:
            if intersects(region[1][0], x) and intersects(region[1][1], y) and intersects(region[1][2], z):
                new_regions.extend(split(region[0], region[1][0], region[1][1], region[1][2], command, x, y, z))
            else:
                new_regions.append(region)
        else:
            new_regions.append(region)
    regions = new_regions

on = 0
for region in regions:
    if region[0] == 1:
        dim = region[1]
        on += (dim[0][1] - dim[0][0]) * (dim[1][1] - dim[1][0]) * (dim[2][1] - dim[2][0])
print(on)