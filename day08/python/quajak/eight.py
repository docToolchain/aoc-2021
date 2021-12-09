f = open("eight.txt", "r")
lines = [x.strip() for x in f.readlines()]

special = 0

for line in lines:
    parts = line.split("|")
    letters = [''.join(sorted(o.strip())) for o in parts[0].split()]
    output = [''.join(sorted(o.strip())) for o in parts[1].split()]
    lengths = {}
    for word in letters:
        word = word.strip()
        length = len(word)
        if length not in lengths:
            lengths[length] = [word]
        elif not word in lengths[length]:
            lengths[length].append(word)
    
    for length in lengths.keys():
        if len(lengths[length]) == 1:
            special += output.count(lengths[length][0])
    
print(special)

sum = 0
for line in lines:
    parts = line.split("|")
    letters = [''.join(sorted(o.strip())) for o in parts[0].split()]
    output = [''.join(sorted(o.strip())) for o in parts[1].split()]
    lengths = {}
    for word in letters:
        word = word.strip()
        length = len(word)
        if length not in lengths:
            lengths[length] = [word]
        elif not word in lengths[length]:
            lengths[length].append(word)
    
    values = {}
    one = lengths[2][0]
    four = lengths[4][0]
    values[one] = 1
    values[four] = 4
    values[lengths[3][0]] = 7
    eight = lengths[7][0]
    values[eight] = 8
    nine = ""
    for val in lengths[6]:
        if all(c in val for c in four):
            nine = val
            values[val] = 9
        elif all(c in val for c in one):
            values[val] = 0
        else:
            values[val] = 6
    
    for val in lengths[5]:
        if all(c in val for c in one):
            values[val] = 3
        elif len(set(list(val)).difference(list(nine))) == 0:
            values[val] = 5
        else:
            values[val] = 2
            
    o = values[output[0]] * 1000 + values[output[1]] * 100 + values[output[2]] * 10 + values[output[3]]
    sum += o
    
print(sum)