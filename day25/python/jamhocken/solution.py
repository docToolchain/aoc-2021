import copy

def process_input(file_contents):
    lines_stripped = [line.strip() for line in file_contents]

    seafloor = [list(line) for line in lines_stripped]    

    return seafloor

def main():
    with open("input.txt",'r') as cuc_file:
        cuc_lines = cuc_file.readlines()

    seafloor = process_input(cuc_lines)
    
    flag = 1
    rounds = 0
    while flag != 0:
        flag = 0
        new_floor = copy.deepcopy(seafloor)
        for i,row in enumerate(seafloor):
            for j,position in enumerate(row):
                if position == ">":
                    if j < len(row)-1:
                        if row[j+1] == ".":
                            new_floor[i][j] = "."
                            new_floor[i][j+1] = ">"
                            flag = 1
                    else:
                        if row[0] == ".":
                            new_floor[i][j] = "."
                            new_floor[i][0] = ">"
                            flag = 1
        seafloor = copy.deepcopy(new_floor)
        for i,row in enumerate(seafloor):
            for j,position in enumerate(row):
                if position == "v":
                    if i < len(seafloor)-1:
                        if seafloor[i+1][j] == ".":
                            new_floor[i][j] = "."
                            new_floor[i+1][j] = "v"
                            flag = 1
                    else:
                        if seafloor[0][j] == ".":
                            new_floor[i][j] = "."
                            new_floor[0][j] = "v"
                            flag = 1
        seafloor = copy.deepcopy(new_floor)

        rounds += 1

    print(rounds)
main()
