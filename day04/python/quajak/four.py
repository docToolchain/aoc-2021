f = open(".\day02\python\quajak\\four.txt", "r")
lines = [x.strip() for x in f.readlines()]

class Board:
    def __init__(self, raw):
        self.lines = []
        self.values = []
        self.won = False
        
        # horizontal
        for l in raw:
            self.lines.append((5, l))
            
        # vertical
        lines = [[], [], [], [], []]
        for l in raw:
            for i in range(len(l)):
                lines[i].append(l[i])
                
        for l in lines:
            self.lines.append((5, l))
        
        for r in raw:
            self.values += r
        
    def add(self, val):
        if val in self.values and not self.won:
            self.values.remove(val)
        else:
            return 0
        for i in range(len(self.lines)):
            count, vals = self.lines[i]
            if val in vals:
                count -= 1
            self.lines[i] = (count, vals)
            if count == 0:
                self.won = True
                return sum(self.values) * val
        return 0

drawn = []
draws = [int(ch) for ch in lines[0].split(",")]

boards = []

b_index = 0
index = 1
while len(lines) > index:
    index += 1
    raw = []
    for i in range(5):
        raw.append([int(c) for c in lines[index].split()])
        index += 1
    cur = Board(raw)
    boards.append(cur)
        
first = False
last = 0
for d in draws:
    for b in boards:
        val = b.add(d)
        if val != 0:
            if not first:
                print("Star 1", val)
                first = True
            last = val
            
print("Star 2", last)