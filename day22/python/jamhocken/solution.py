import regex as re

def process_input(file_contents):
    lines_stripped = [line.strip() for line in file_contents]
    
    cube = list()
    pattern = re.compile("(-?\d+)")
    for line in lines_stripped:
        numbers = re.findall(pattern,line)
        cube.append((line[0:2],int(numbers[0]),int(numbers[1])+1,int(numbers[2]),int(numbers[3])+1,int(numbers[4]),int(numbers[5])+1))
                
    return cube

def cut_cube(cube_in, cube_cut):
    cubes = list()
    cubes.append((min(cube_in[1],cube_cut[1]),cube_in[1],cube_in[2],cube_in[3],cube_in[4],cube_in[5]))
    cubes.append((cube_in[0],max(cube_cut[0],cube_in[0]),cube_in[2],cube_in[3],cube_in[4],cube_in[5]))
    cubes.append((max(cube_in[0],cube_cut[0]),min(cube_in[1],cube_cut[1]),min(cube_in[3],cube_cut[3]),cube_in[3],cube_in[4],cube_in[5]))
    cubes.append((max(cube_in[0],cube_cut[0]),min(cube_in[1],cube_cut[1]),cube_in[2],max(cube_in[2],cube_cut[2]),cube_in[4],cube_in[5]))
    cubes.append((max(cube_in[0],cube_cut[0]),min(cube_in[1],cube_cut[1]),max(cube_in[2],cube_cut[2]),min(cube_in[3],cube_cut[3]),min(cube_in[5],cube_cut[5]),cube_in[5]))
    cubes.append((max(cube_in[0],cube_cut[0]),min(cube_in[1],cube_cut[1]),max(cube_in[2],cube_cut[2]),min(cube_in[3],cube_cut[3]),cube_in[4],max(cube_cut[4],cube_in[4])))

    return cubes

def main():
    with open("input.txt",'r') as player_file:
        player_lines = player_file.readlines()

    insts = process_input(player_lines)

    cubes = 0
    cubes_star1 = 0
    for i,inst in enumerate(insts):
        if inst[0] != "of":
            space = [inst[1:]]
            for inst1 in insts[i+1:]:
                space_temp = list()
                for cube in space:
                    if inst1[2] >= cube[0] and inst1[4]>= cube[2] and inst1[6] >= cube[4] and inst1[1] <= cube[1] and inst1[3] <= cube[3] and inst1[5]<=cube[5]:
                        if inst1[1]<=cube[0] and inst1[2]>=cube[1] and inst1[3]<=cube[2] and inst1[4]>=cube[3] and inst1[5]<=cube[4] and inst1[6]>=cube[5]:
                            pass
                        else:
                            [space_temp.append(i) for i in cut_cube(cube,inst1[1:])]
                    else:
                        space_temp.append(cube)
                space = list(set(space_temp))
            cubes += sum([(spa[1]-spa[0])*(spa[3]-spa[2])*(spa[5]-spa[4]) for spa in space])
            if inst[2] >= -50 and inst[4]>= -50 and inst[6] >= -50 and inst[1] <= 50 and inst[3] <= 50 and inst[5]<=50:
                cubes_star1 += sum([(spa[1]-spa[0])*(spa[3]-spa[2])*(spa[5]-spa[4]) for spa in space])
    
    print(cubes_star1)
    print(cubes)
    
main()
