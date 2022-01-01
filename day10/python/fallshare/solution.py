from collections import Counter

file1 = open('input.txt', 'r')

lines = list()
for line in file1:
    lines.append(list(line.strip()))

illegal_lines = list()
for line in lines:
    counts = (Counter(line))
    print(line)
    print(counts)
    if (counts["<"] + counts["["] + counts["{"] + counts["("] ) == (counts[">"] + counts["]"] + counts["}"] + counts[")"]):
        illegal_lines.append(line)
        print("added")
    print("--")



for line in illegal_lines:
    print(line)

