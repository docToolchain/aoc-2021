from typing import List
import math

Map = List[List[int]]


class P2D:
    """2-dimensional point."""

    x: int = 0
    y: int = 0

    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y

    def __eq__(self, other):
        return self.x == other.x and self.y == other.y

    def points_to(self, other):
        """Return a list of all points between (including) self and another point."""
        dist = self.distance(other)
        if dist == 0:
            return [self]
        delta = (other.x - self.x, other.y - self.y)
        points = list()
        dx = delta[0] / abs(delta[0]) if delta[0] != 0 else 0
        dy = delta[1] / abs(delta[1]) if delta[1] != 0 else 0
        dx = int(dx)
        dy = int(dy)
        for d in range(dist + 1):
            p = P2D(self.x + d * dx, self.y + d * dy)
            points.append(p)
        return points

    def distance(self, other) -> int:
        """Calculate distance to another point."""
        dx = other.x - self.x
        dy = other.y - self.y
        distance = max(abs(dx), abs(dy))
        return distance

    def is_diagonal(self, other):
        return self.x != other.x and self.y != other.y

    def __repr__(self):
        return f"(x={self.x}, y={self.y})"


Entries = List[List[P2D]]


def remove_diagonal_entries(entries):
    """Modifies list in place. Removes all diagonal lines."""
    ptr = len(entries) - 1
    while ptr:
        entry = entries[ptr]
        if entry[0].is_diagonal(entry[1]):
            del entries[ptr]
        ptr -= 1


def get_board_size(entries: Entries) -> P2D:
    """Find maximum x/y position."""
    max_x = 0
    max_y = 0
    for e in entries:
        max_x = max(max_x, e[0].x, e[1].x)
        max_y = max(max_y, e[0].y, e[1].y)
    return P2D(max_x, max_y)


def create_map(x_max: int, y_max: int) -> Map:
    """Create a map of given dimensions.

    Each element starts with 0.
    """
    n_rows = y_max + 1
    n_cols = x_max + 1
    output = [0] * n_rows
    output = [[0] * n_cols for row in output]
    return output


def mark_spots(spots: Map, entry: List[P2D]):
    """Create entries in spot list where provided entry produces a vent.

    Modifies spots argument in place.
    """
    points = entry[0].points_to(entry[1])
    for point in points:
        spots[point.y][point.x] += 1


def print_map(spots: Map):
    for row in spots:
        s = "".join([str(n) for n in row]).replace("0", ".")
        print(s)


def star1(puzzle_in):
    remove_diagonal_entries(puzzle_in)
    p = get_board_size(puzzle_in)
    spots = create_map(p.x, p.y)
    for entry in puzzle_in:
        mark_spots(spots, entry)
    # print_map(spots)
    spots_with_two_vents = 0
    for row in spots:
        for point in row:
            if point >= 2:
                spots_with_two_vents += 1
    return spots_with_two_vents


def star2(puzzle_in):
    p = get_board_size(puzzle_in)
    spots = create_map(p.x, p.y)
    for entry in puzzle_in:
        mark_spots(spots, entry)
    # print_map(spots)
    spots_with_two_vents = 0
    for row in spots:
        for point in row:
            if point >= 2:
                spots_with_two_vents += 1
    return spots_with_two_vents


def read_input(filepath):
    """Create input and parse it.

    Returns:
        list of tuples (P2D, P2D)
    """
    entries = list()
    with open(filepath, "r") as f:
        for line in f:
            points = line.split(" -> ")
            entry = list()
            for point in points:
                x, y = [int(s) for s in point.split(",")]
                entry.append(P2D(x, y))
            entries.append(entry)
    return entries


if __name__ == "__main__":
    puzzle_in = read_input("input.txt")
    print(f"Result for first star: {star1(puzzle_in)}")
    puzzle_in = read_input("input.txt")
    print(f"Result for first star: {star2(puzzle_in)}")
