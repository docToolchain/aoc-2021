f = open("six.txt", "r")
lines = [x.strip() for x in f.readlines()]

fishes = [int(num) for num in lines[0].split(",")]

ages = {}

for i in range(9):
    ages[i] = 0

for fish in fishes:
    ages[fish] += 1
    
# change range() for first star
for i in range(256):
    n_ages = {}
    for age in range(1, 9):
        n_ages[age - 1] = ages[age]
    n_ages[6] += ages[0]
    n_ages[8] = ages[0]
    ages = n_ages
    
print(sum(ages.values()))