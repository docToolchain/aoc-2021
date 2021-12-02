from typing import List


class Submarine:
    aim: int = 0
    x: int = 0
    y: int = 0

    def __init__(self):
        pass

    def parse_command(self, cmd):
        (dir, value) = cmd
        if dir == "y":
            self.aim += value
        else:
            self.move(value)

    def move(self, value):
        self.x += value
        self.y += self.aim * value


def star1(puzzle_in):
    pos = drive_sub(puzzle_in)
    return pos[0] * pos[1]


def star2(puzzle_in):
    s = Submarine()
    for c in puzzle_in:
        s.parse_command(c)
    return s.x * s.y


def read_input(filepath):
    """Create input and parse it.

    Returns:
     (direction, value) tuple where direction = "x" or "y" and value is an integer
    """
    commands = list()
    with open(filepath, "r") as f:
        for line in f:
            (dir, value) = line.split(" ")
            value = int(value)
            if dir == "forward":
                commands.append(("x", value))
            elif dir == "down":
                commands.append(("y", value))
            elif dir == "up":
                commands.append(("y", -value))
            else:
                raise ValueError(f"Unknown direction encountered: {dir} in line {line}")
    return commands


def move_sub(command, start_position=(0, 0)):
    """Move the submarine for one step.

    Args:
        start_position: tuple of (x, y)

    Returns:
        position: tuple of (x, y)
    """
    (dir, value) = command
    (x, y) = start_position
    if dir == "x":
        x += value
    else:
        y += value
    return (x, y)


def drive_sub(command_list: List, start_position=(0, 0)):
    """Move the sub for multiple steps."""
    pos = start_position
    for c in command_list:
        pos = move_sub(c, pos)
    return pos


if __name__ == "__main__":
    puzzle_in = read_input("input.txt")
    print(f"Result for first star: {star1(puzzle_in)}")
    print(f"Result for first star: {star2(puzzle_in)}")
