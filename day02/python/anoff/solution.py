from typing import List


def star1(puzzle_in):
    pos = drive_sub(puzzle_in)
    return pos[0] * pos[1]


def star2(puzzle_in):
    pass


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
