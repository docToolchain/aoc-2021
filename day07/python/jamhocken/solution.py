import statistics

def process_input(file_contents):
    lines_stripped = [line.strip() for line in file_contents]
    
    crabs = [int(i) for i in lines_stripped[0].split(",")]

    return crabs

def main():
    with open("input.txt",'r') as crab_file:
        crab_lines = crab_file.readlines()

    crabs = process_input(crab_lines)

    # The optimal position is the median for star 1
    center = int(statistics.median(crabs))

    fuel = 0    
    for crab in crabs:
        fuel += abs(crab-center)
    print(fuel)
    
    # The median is a good starting point for star 2
    fuel = 0    
    for crab in crabs:
        for i in range(abs(crab-center)):
            fuel += i+1

    # Is the optimal answer more or less than the median?
    fuel_old = fuel
    fuel = 0
    center_temp = center - 1
    for crab in crabs:
        for i in range(abs(crab-center_temp)):
            fuel += i+1
    if fuel < fuel_old:
        iterator = -1
    else:
        iterator = 1  

    # Iterate until the values start to increase
    fuel = fuel_old
    while fuel<=fuel_old:
        fuel_old = fuel
        fuel = 0
        center = center + iterator
        for crab in crabs:
            for i in range(abs(crab-center)):
                fuel += i+1
    
    print(fuel_old)
    
main()
