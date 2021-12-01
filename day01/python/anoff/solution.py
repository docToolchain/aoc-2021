def calc_deltas(numbers) -> list:
    """Calculate the delta between each point and the next."""
    deltas = list()
    for n in range(len(numbers) - 1):
        d = numbers[n + 1] - numbers[n]
        deltas.append(d)
    return deltas


def windowed_sum(numbers, window_size=3):
    sums = list()
    assert window_size % 2 == 1, "window_size should be an uneven number"
    for n in range(len(numbers) - window_size + 1):
        s = sum(numbers[n : n + window_size])
        sums.append(s)
    return sums


def star1(puzzle_in):
    deltas = calc_deltas(puzzle_in)
    return sum([1 for d in deltas if d > 0])


def star2(puzzle_in):
    sums = windowed_sum(puzzle_in)
    deltas = calc_deltas(sums)
    return sum([1 for d in deltas if d > 0])


def read_input(filepath):
    numbers = list()
    with open(filepath, "r") as f:
        for line in f:
            numbers.append(int(line))
    return numbers


if __name__ == "__main__":
    puzzle_in = read_input("input.txt")
    print(f"Result for first star: {star1(puzzle_in)}")
    print(f"Result for first star: {star2(puzzle_in)}")
