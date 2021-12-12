file1 = open('input.txt', 'r')
string_list = file1.readline().rstrip().split(",")
lantern_fishes = list(map(int, string_list))


max_days = 80
for day in range(0, max_days):
    for fish in range(len(lantern_fishes)):
        lantern_fishes[fish] -= 1
        if lantern_fishes[fish] == -1:
            lantern_fishes[fish] = 6
            lantern_fishes.append(8)
print("Star 1")
print(f"After 80 days {len(lantern_fishes)} fishes are found")

#Star 2
# Exponential growth
file1 = open('input.txt', 'r')
string_list = file1.readline().rstrip().split(",")
lantern_fishes = list(map(int, string_list))

fish_counter = [0] * 9
for fish in range(len(lantern_fishes)):
    fish_age = lantern_fishes[fish]
    fish_counter[fish_age] += 1
print(fish_counter)    
max_days = 256
for day in range(0, max_days):
    new_fish_counter = [0] * 9
    for age in reversed(range(9)):
        if age == 0:
           new_fish_counter[8] = fish_counter[age]
           new_fish_counter[6] += fish_counter[age]  
        else:
           new_fish_counter[age - 1] = fish_counter[age] 
    fish_counter = new_fish_counter

print("Star 2")
print(f"After {max_days} days {sum(fish_counter)} fishes are found")


