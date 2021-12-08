from typing import List, Dict


def fuel_cost(subs: Dict, position: int):
    cost = 0
    for sub_pos, sub_count in subs.items():
        cost += abs(sub_pos - position) * sub_count
    return cost


# tag::star1[]
def star1(subs: Dict):
    possible_positions = subs.keys()
    min_cost = 1e9
    min_pos = 0
    for p in possible_positions:
        cost = fuel_cost(subs, p)
        if cost < min_cost:
            min_cost = cost
            min_pos = p
    return min_cost


# end::star1[]


def fuel_cost2(subs: Dict, position: int):
    cost = 0
    for sub_pos, sub_count in subs.items():
        delta = abs(sub_pos - position)

        cost += delta * (delta + 1) / 2 * sub_count
    return int(cost)


# tag::star2[]
def star2(subs: Dict):
    possible_positions = range(max(subs.keys()))
    min_cost = 1e9
    min_pos = 0
    for p in possible_positions:
        cost = fuel_cost2(subs, p)
        if cost < min_cost:
            min_cost = cost
            min_pos = p
    print("Best position:", min_pos)
    return min_cost


# end::star2[]


def read_input(filepath):
    """Create input and parse it.

    Returns:
        dictionary with key->position, value->count of subs
    """
    d = dict()
    with open(filepath, "r") as f:
        entries = f.readline()
    entries = [int(s) for s in entries.split(",")]
    for n in entries:
        if n in d:
            d[n] += 1
        else:
            d[n] = 1
    return d


if __name__ == "__main__":
    puzzle_in = read_input("input.txt")
    print(f"Result for first star: {star1(puzzle_in)}")
    puzzle_in = read_input("input.txt")
    print(f"Result for second star: {star2(puzzle_in)}")
