# read input from file "input.txt" line by line
# and print the sum of all increasing numbers

# tag::read_input[]
with open("input.txt", "r") as f:
    input = f.read()
# end::read_input[]

# tag::sum_increasing_numbers[]
# iterate over all lines
sum = 0
prevNum = 100000000000
for line in input.splitlines():
    # convert to int
    num = int(line)
    # check if num is increasing
    if num > prevNum:
        sum += 1
    prevNum = num
print (sum)
# end::sum_increasing_numbers[]
