from typing import Tuple

target_area_x = [241, 273]
target_area_y = [-97, -63]

def simulate_flight(dx, dy) -> Tuple[bool, int]:
    x = 0
    y = 0
    max_y = 0
    while x <= target_area_x[1] and y >= target_area_y[0]:
        if x >= target_area_x[0] and y <= target_area_y[1]:
            return [True, max_y]
        x += dx
        y += dy
        if dy > 0:
            max_y = max(max_y, y)
        if dx != 0:
            dx += -1 * dx // abs(dx)
        dy -= 1
    if y < target_area_y[1]:
        return [False, -1]
    return [False, -1]

max_y = -1
count = 0
for x in range(274):
    for y in range(-97, 100):
        hit, m = simulate_flight(x,y)
        if hit:
            count += 1
            if max_y < m:
                max_y = m
print(max_y)
print(count)