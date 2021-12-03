def star1(puzzle_in):
    digit_count = len(puzzle_in[0])
    line_count = len(puzzle_in)
    # sum up all digits per column
    sum = [0] * digit_count
    for digits in puzzle_in:
        for n in range(digit_count):
            sum[n] += digits[n]
    # if "1" is the most common number in one position -> the sum should be > half line count
    more_than_half = [d >= (line_count / 2) for d in sum]
    gamma_binary_str = "".join(["1" if d else "0" for d in more_than_half])
    gamma = int(gamma_binary_str, 2)

    epsilon_binary_str = "".join(["0" if d else "1" for d in more_than_half])
    epsilon = int(epsilon_binary_str, 2)
    return epsilon * gamma


def star2(puzzle_in):
    digit_count = len(puzzle_in[0])
    line_count = len(puzzle_in)

    def filter_by_most_common(lines, position, invert=False):
        total_sum = sum([l[position] for l in lines])
        most_common = "1" if total_sum >= (len(lines) / 2) else "0"
        if invert:
            most_common = "0" if most_common == "1" else "1"
        filtered = [l for l in lines if l[position] == int(most_common)]
        return filtered

    lines = puzzle_in
    pos = 0
    while len(lines) > 1 and pos < digit_count:
        lines = filter_by_most_common(lines, pos)
        pos += 1
    oxygen = int("".join([str(d) for d in lines[0]]), 2)

    lines = puzzle_in
    pos = 0
    while len(lines) > 1 and pos < digit_count:
        lines = filter_by_most_common(lines, pos, True)
        pos += 1
    co2 = int("".join([str(d) for d in lines[0]]), 2)

    return co2 * oxygen


def read_input(filepath):
    """Create input and parse it."""
    values = list()
    with open(filepath, "r") as f:
        for line in f:
            digits = list(line.strip())
            digits = [int(d) for d in digits]
            values.append(digits)
    return values


if __name__ == "__main__":
    puzzle_in = read_input("input.txt")
    print(f"Result for first star: {star1(puzzle_in)}")
    print(f"Result for first star: {star2(puzzle_in)}")
