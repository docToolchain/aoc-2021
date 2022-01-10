import regex as re

def process_input(file_contents):
    input_regex = re.compile('(-?\d+)')
    area = re.findall(input_regex,file_contents[0])
    area = [int(i) for i in area]
    
    return area

def landed(x,y,area):
    
    pos1 = 0
    pos2 = 0
    hit = 0
    while pos1 < area[1]+1 and pos2 > area[2]-1:
        pos1 += x
        pos2 += y
        if pos1 in list(range(area[0],area[1]+1)) and pos2 in list(range(area[2],area[3]+1)):
            hit=1
        if x != 0:
            x -= abs(x)//x
        y -= 1
        
    return hit

def main():
    with open("input.txt",'r') as trajectory_file:
        trajectory_lines = trajectory_file.readlines()

    area = process_input(trajectory_lines)
    # star 1
    print(max(-1*area[2],-1*area[3])*(max(-1*area[2],-1*area[3])-1)//2)
    
    hits = 0
    for x in range(1,area[1]+1):
        for y in range(area[2],-1*area[2]-1+1):
            hits += landed(x,y,area)
    print(hits)
    
main()
