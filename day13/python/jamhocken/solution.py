def process_input(file_contents):
    lines_stripped = [line.strip() for line in file_contents]

    i = 0
    coordinates = set()
    instructions = []
    while lines_stripped[i]:
        coord = lines_stripped[i].split(",")
        coordinates.add((int(coord[0]),int(coord[1])))
        i += 1
    
    i += 1
    while i < len(lines_stripped):
        parts = lines_stripped[i].split()
        instructions.append((parts[2][0],int(parts[2][2:])))
        i += 1
            
    return (coordinates,instructions)

def main():
    with open("input.txt",'r') as paper_file:
        paper_lines = paper_file.readlines()

    (coordinates,instructions) = process_input(paper_lines)

    # star 1    
    folded_coordinates = set()
    for coord in coordinates:
        if instructions[0][0] == 'y':
            if coord[1]<instructions[0][1]:
                folded_coordinates.add(coord)
            else:
                folded_coordinates.add((coord[0],2*instructions[0][1]-coord[1]))
        else:
            if coord[0]<instructions[0][1]:
                folded_coordinates.add(coord)                
            else:
                folded_coordinates.add((2*instructions[0][1]-coord[0],coord[1]))

    print(len(folded_coordinates))
    
    # star 2
    for inst in instructions:
        folded_coordinates = set()
        for coord in coordinates:
            if inst[0] == 'y':
                if coord[1]<inst[1]:
                    folded_coordinates.add(coord)
                else:
                    folded_coordinates.add((coord[0],2*inst[1]-coord[1]))
            else:
                if coord[0]<inst[1]:
                    folded_coordinates.add(coord)                
                else:
                    folded_coordinates.add((2*inst[1]-coord[0],coord[1]))
        coordinates = folded_coordinates
    
    max_x = max(set([coordinate[0] for coordinate in coordinates]))
    max_y = max(set([coordinate[1] for coordinate in coordinates]))
    
    for y in range(max_y+1):
        for x in range(max_x+1):
            if (x,y) in coordinates:
                print("#",end='')
            else:
                print(".",end='')
        print()

main()
