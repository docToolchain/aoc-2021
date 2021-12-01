with open('input.txt') as input_file:
    input = []
    for line in input_file:
        input.append(int(line.strip('\n')))

assert int(input[0]) == 195

def getDepthIncrement_Star1(input):
    previousDepth = input[0]
    increment = 0

    for depth in input:        
        if depth > previousDepth:
            increment += 1
        previousDepth = depth

    return increment

def getDepthIncrementSlidingWindow_Star2(input):
    previousDepthWindow = input[0] + input[1] + input [2]
    increment = 0
    for i in range(1, len(input) - 2 , 1):
        currentDepthWindow = input[i] + input[i+1] + input [i+2]     
        if currentDepthWindow > previousDepthWindow:
            increment += 1
        previousDepthWindow = currentDepthWindow
        print(str(i))
    return increment



print("Star 1: Depth increase is " + str(getDepthIncrement_Star1(input)))
print("Star 1: Depth increase is " + str(getDepthIncrementSlidingWindow_Star2(input)))