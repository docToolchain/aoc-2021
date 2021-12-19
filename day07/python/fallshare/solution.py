file1 = open('input.txt', 'r')
string_list = file1.readline().rstrip().split(",")
crab_positions = list(map(int, string_list))

closest_crab = min(crab_positions)
furthest_crab = max(crab_positions)

max_distance = furthest_crab - closest_crab

print(f"Closest crab: {closest_crab}")
print(f"Furthest crab: {furthest_crab}")
print(f"Crab distance: {max_distance}")

min_distance = max_distance * len(crab_positions)

distance_sum = 0
best_position = max_distance
for current_position in range(closest_crab, furthest_crab):
    for crab_position in crab_positions:
        distance_sum += abs(current_position - crab_position)
    if distance_sum < min_distance:
        min_distance = distance_sum
        best_position = current_position
    distance_sum = 0

print("Start 1")
print(f"Best position is: {best_position}")
print(f"Fuel to spend: {min_distance}")

def distance_to_fuel_star2(distance):
    return (distance * (distance + 1)) / 2

min_distance = max_distance * len(crab_positions) * len(crab_positions)
distance_sum = 0
best_position = max_distance
for current_position in range(closest_crab, furthest_crab):
    for crab_position in crab_positions:
        distance_sum += distance_to_fuel_star2(abs(current_position - crab_position))

    if distance_sum < min_distance:
        min_distance = distance_sum
        best_position = current_position
    distance_sum = 0



print("Start 2")
print(f"Best position is: {best_position}")
print(f"Fuel to spend: {min_distance}")