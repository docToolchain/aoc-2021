from typing import List


def sunrise(fish):
    new_fish = list()
    for i, f in enumerate(fish):
        if f > 0:
            fish[i] -= 1
        else:
            new_fish.append(8)
            fish[i] = 6
    fish += new_fish


# tag::star1[]
def star1(puzzle_in):
    for n in range(80):
        sunrise(puzzle_in)
    return len(puzzle_in)


# end::star1[]


def fish2dict(puzzle_in):
    count = dict()
    for n in range(9):
        count[n] = len([f for f in puzzle_in if f == n])
    return count


def sunset(fish):
    new = fish[0]
    for n in list(range(8)):
        fish[n] = fish[n + 1]
    fish[8] = new
    fish[6] += new


# tag::star2[]
def star2(puzzle_in):
    fish = fish2dict(puzzle_in)
    for n in range(256):
        sunset(fish)
    return sum(fish.values())


# end::star2[]


def read_input(filepath):
    """Create input and parse it.

    Returns:
        list of integers
    """
    with open(filepath, "r") as f:
        entries = f.readline()
    entries = [int(s) for s in entries.split(",")]
    return entries


if __name__ == "__main__":
    puzzle_in = read_input("input.txt")
    print(f"Result for first star: {star1(puzzle_in)}")
    puzzle_in = read_input("input.txt")
    print(f"Result for second star: {star2(puzzle_in)}")
