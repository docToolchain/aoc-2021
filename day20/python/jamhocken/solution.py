def process_input(file_contents):
    lines_stripped = [line.strip() for line in file_contents]
    
    enhancer = lines_stripped[0]
    trench = dict()
    for j,line in enumerate(lines_stripped[2:]):
        for i,character in enumerate(line):
            trench[(i,j)] = character
        
    return enhancer, trench

def main():
    with open("input.txt",'r') as trench_file:
        trench_lines = trench_file.readlines()

    (enhancer, trench) = process_input(trench_lines)
    size_trench = max(trench.keys())[0]+1
    
    neighbors = [(-1,-1),(0,-1),(1,-1),(-1,0),(0,0),(1,0),(-1,1),(0,1),(1,1)]
    
    for i in range(50):
        for j in range(-1-i,size_trench+i):
            if i%2 == 0:
                trench[(j,-1-i)] = "."
                trench[(j,size_trench+i)] = "."
                trench[(-1-i,j)] = "."
                trench[(size_trench+i,j)] = "."
                trench[(size_trench+i,size_trench+i)] = "."
            else:
                trench[(j,-1-i)] = enhancer[0]
                trench[(j,size_trench+i)] = enhancer[0]
                trench[(-1-i,j)] = enhancer[0]
                trench[(size_trench+i,j)] = enhancer[0]
                trench[(size_trench+i,size_trench+i)] = enhancer[0]

        trench_temp = trench.copy()
        for key,value in trench.items():
            grid = [tuple(map(sum,zip(key,neighbor))) for neighbor in neighbors]
            code = ''.join([trench[grid_item] if grid_item in trench else '.' if i%2 == 0 else enhancer[0] for grid_item in grid])            
            code = ''.join(["1" if character == "#" else "0" for character in code])
            trench_temp[key] = enhancer[int(code,2)]
        trench = trench_temp
        if i == 1:
            print(sum([1 if i == "#" else 0 for key,i in trench.items()]))
                
    print(sum([1 if i == "#" else 0 for key,i in trench.items()]))
main()
