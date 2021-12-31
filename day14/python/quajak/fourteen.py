f = open("fourteen.txt", "r")
lines = [x.strip() for x in f.readlines()]

poly = lines[0]
polymer = lines[0]

lines = lines[2:]

rules = {}

for line in lines:
    s = line.split(" -> ")
    rules[s[0]] = s[1]


for i in range(10):
    new = ""
    for pos in range(len(polymer)):
        pair = polymer[pos:pos+2]
        if pair in rules:
            new += pair[0] + rules[pair]
        else:
            new += pair[0]
    polymer = new

chars = {}
for c in polymer:
    if c not in chars:
        chars[c] = polymer.count(c)
    
print(max(chars.values()) - min(chars.values()))
    
polymer = poly
pairs = {}

for pos in range(len(polymer)):
    pair = polymer[pos:pos+2]
    if pair not in pairs:
        pairs[pair] = 1
    else:
        pairs[pair] += 1
        
        
for i in range(40):
    #print(pairs)
    d = {}
    for key, value in pairs.items():
        if key in rules:
            d[key[0] + rules[key]] = d.get(key[0] + rules[key], 0) + value
            d[rules[key] + key[1]] = d.get(rules[key] + key[1], 0) + value
        else:
            d[key] = d.get(key, 0) + value
    pairs = d
    
chars = {}
for pair,count in pairs.items():
    if pair[0] not in chars:
        chars[pair[0]] = count
    else:
        chars[pair[0]] += count
    # if len(pair) == 2:
    #     if pair[1] not in chars:
    #         chars[pair[1]] = count
    #     else:
    #         chars[pair[1]] += count
        
print(max(chars.values()) - min(chars.values()))