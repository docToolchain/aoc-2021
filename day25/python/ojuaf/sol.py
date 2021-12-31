
#!/usr/bin/env python3

def load_input():
    
    with open('input') as fd:
        rights = set()
        downs = set()
        x_max = None
        y_max = None
        for i, line in enumerate(fd):
            for j, ch in enumerate(line.strip()):
                if ch == '>':
                    rights.add((j, i))
                elif ch == 'v':
                    downs.add((j, i))
        else:
            x_max = j
            y_max = i
    return rights, downs, x_max, y_max

def get_next_right(right, x_max):
    return ((right[0]+1)%(x_max+1), right[1])

def get_next_down(down, y_max):
    return (down[0], (down[1]+1)%(y_max+1))


def process(rights, downs, x_max, y_max):
    next_rights = set()
    next_downs = set()
    change = False
    for right in rights:
        next_right = get_next_right(right, x_max)
        if next_right in downs or next_right in rights:
            next_rights.add(right)
        else:
            next_rights.add(next_right)
            change = True
    for down in downs:
        next_down = get_next_down(down, y_max)
        if next_down in downs or next_down in next_rights:
            next_downs.add(down)
        else:
            next_downs.add(next_down)
            change = True
    return next_rights, next_downs, change
        

def part1():
    # First Part
    rights, downs, x_max, y_max = load_input()
    count = 0
    change = True
    while change:
        rights, downs, change = process(rights, downs, x_max, y_max)
        count += 1
        
    result = count
    print("Part 1:", result)
    return

if __name__ == '__main__':
    part1()
