def load_input():
    data = list()
    with open('input') as fd:
        for line in fd:
            value = line.strip().split()
            data.append((value[0], int(value[1])))
    return data


def part1():
    data = load_input()
    # First part
    x = 0
    y = 0
    for cmd, value in data:
        if cmd == "forward":
            x += value
        elif cmd == "down":
            y += value
        elif cmd == "up":
            y -= value
        else:
            pass
    result = x*y
    print("Part 1: ", result)


def part2():
    # Second part
    data = load_input()
    x = 0
    y = 0
    aim = 0
    for cmd, value in data:
        if cmd == "forward":
            x += value
            y += aim*value
        elif cmd == "down":
            aim += value
        elif cmd == "up":
            aim -= value
        else:
            pass

    result = x*y
    print("Part 2: ", result)
            
    return 0

if __name__ == '__main__':
    part1()
    part2()
