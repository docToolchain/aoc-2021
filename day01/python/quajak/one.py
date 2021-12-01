f = open("./one.txt", "r")
lines = [int(x.strip()) for x in f.readlines()]

change = 0
last = lines[0]
for line in lines:
    change += last < line
    last = line
    
print("Star 1", change)

change = 0
buf = lines[:3]
last = sum(buf)
for line in lines[3:]:
    buf = buf[1:] + [line]
    new = sum(buf)
    change += last < new
    last = new

print("Star 2", change)