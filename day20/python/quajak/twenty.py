
f = open("twenty.txt", "r")
lines = [x.strip() for x in f.readlines()]

color_table = lines[0].replace(".","0").replace("#", "1")

lines = lines[2:]

image = {}

y = 0
for line in lines:
    line = line.replace(".","0").replace("#", "1")
    x = 0
    for ch in line:
        image[(x,y)] = ch
        x += 1
    y += 1
    
background = "0"
for i in range(50):
    min_x = min([x for x,y in image.keys()]) - 2
    max_x = max([x for x,y in image.keys()]) + 2
    min_y = min([y for x,y in image.keys()]) - 2
    max_y = max([y for x,y in image.keys()]) + 2
    
    new_image = {}
    for x in range(min_x, max_x + 1):
        for y in range(min_y, max_y + 1):
            val = image.get((x-1, y-1), background) + image.get((x, y-1), background) + image.get((x+1, y-1), background) + image.get((x-1, y), background) + image.get((x, y), background) +image.get((x+1, y), background) \
                + image.get((x-1, y+1), background) + image.get((x, y+1), background) + image.get((x+1, y+1), background)
            val = int(val, 2)
            new_image[(x,y)] = color_table[val]
    image = new_image
    background = color_table[int(background * 9, 2)]
      
min_x = min([x for x,y in image.keys()]) - 1
max_x = max([x for x,y in image.keys()]) + 1
min_y = min([y for x,y in image.keys()]) - 1
max_y = max([y for x,y in image.keys()]) + 1
for y in range(min_y, max_y + 1):
    line = ""
    for x in range(min_x, max_x + 1):
        line += image.get((x,y), "0")
    print(line)

print(sum([1 if pix == "1" else 0 for pos, pix in image.items()]))    