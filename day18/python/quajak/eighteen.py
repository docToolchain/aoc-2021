from typing import Tuple
from copy import deepcopy

f = open("eighteen.txt", "r")
lines = [x.strip() for x in f.readlines()]

class Number:
    def __init__(self, x = None, y = None):
        self.x = x
        self.y = y
    
    def __str__(self):
        return f"[{self.x}, {self.y}]"
    
    def __repr__(self):
        return str(self)
    
    def magnitude(self) -> int:
        sum = 0
        if isinstance(self.x, Number):
            sum += 3 * self.x.magnitude()
        else:
            sum += 3 * self.x
            
        if isinstance(self.y, Number):
            sum += 2 * self.y.magnitude()
        else:
            sum += 2 * self.y
        return sum
        
def parse_line(string) -> Number:
    level = 0
    current = ""
    values = []
    for char in string:
        if char == "[":
            if level != 0:
                current += "["
            level += 1
        elif char == "]":
            level -= 1
            if level != 0:
                current += "]"
        elif char == ",":
            if level == 1:
                values.append(current)
                current = ""
            else:
                current += ","
        else:
            current += char
    values.append(current)
    if len(values) != 2:
        raise Exception
    num = Number()
    if values[0].isnumeric():
        num.x = int(values[0])
    else:
        num.x = parse_line(values[0])
    if values[1].isnumeric():
        num.y = int(values[1])
    else:
        num.y = parse_line(values[1])
    return num
            
numbers = []
for line in lines:
    numbers.append(parse_line(line))
    
def do_explode(level: int, number: Number) -> Tuple[bool, bool, int, int]:
    if level == 4:
        return (True, True, number.x, number.y)
    else:
        if isinstance(number.x, Number):
            changed, cleaned, dx, dy = do_explode(level + 1, number.x)
            if changed:
                if cleaned:
                    number.x = 0
                if dy != 0:
                    cur = number
                    if isinstance(cur.y, Number):
                        cur = cur.y
                        while isinstance(cur.x, Number):
                            cur = cur.x
                        cur.x += dy
                    else:
                        cur.y += dy
                return (True, False, dx, 0)
        if isinstance(number.y, Number):
            changed, should_clean, dx, dy = do_explode(level + 1, number.y)
            if changed:
                if should_clean:
                    number.y = 0
                if dx != 0:
                    cur = number
                    if isinstance(cur.x, Number):
                        cur = cur.x
                        while isinstance(cur.y, Number):
                            cur = cur.y
                        cur.y += dx
                    else:
                        cur.x += dx
                return (True, False, 0, dy)
            else:
                return (False, False, 0, 0)
        else:
            return (False, False, 0, 0)
            
def do_split(number: Number) -> bool:
    if isinstance(number.x, Number):
        hit = do_split(number.x)
        if hit:
            return True
    else:
        if number.x >= 10:
            number.x = Number(number.x // 2, number.x - number.x // 2)
            return True
    
    if isinstance(number.y, Number):
        hit = do_split(number.y)
        if hit:
            return True
    else:
        if number.y >= 10:
            number.y = Number(number.y // 2, number.y - number.y // 2)
            return True
        
    return False
    
            
def add(vals) -> int:
    sum = vals[0]
    vals = vals[1:]
    for num in vals:
        old_sum = sum
        sum = Number()
        sum.x = old_sum
        sum.y = num
        
        changed = True
        while changed:
            changd = False
            changed, _, _, _ = do_explode(0, sum)
            if not changed:
                changed = do_split(sum)
    return sum.magnitude()
        
cp = deepcopy(numbers)
print(add(cp))  

heightest = 0

for x in numbers:
    for y in numbers:
        if x != y:
            val = add(deepcopy([x,y]))
            if val > heightest:
                heightest = val
                
print(heightest)