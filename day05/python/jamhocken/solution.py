import re

def process_input(file_contents):
    lines_stripped = [line.strip() for line in file_contents]
    
    regex_vectorline = re.compile('(\d+),(\d+)\s\S+\s(\d+),(\d+)')
    
    vectors = sum([[[int(i) for i in j] for j in re.findall(regex_vectorline,lines)] for lines in lines_stripped],[])

    return vectors

def star1(vectors):
    vents = {}
    for vector in vectors:
        if vector[0] == vector[2]:
            if vector[1]>vector[3]:
                y_coordinates = range(vector[3],vector[1]+1)
            else:
                y_coordinates = range(vector[1],vector[3]+1)
            for y in y_coordinates:
                if (vector[0],y) in vents:
                    vents[(vector[0],y)] += 1
                else:
                    vents[(vector[0],y)] = 1
        elif vector[1] == vector[3]:
            if vector[0]>vector[2]:
                x_coordinates = range(vector[2],vector[0]+1)
            else:
                x_coordinates = range(vector[0],vector[2]+1)
            for x in x_coordinates:
                if (x,vector[1]) in vents:
                    vents[x,(vector[1])] += 1
                else:
                    vents[x,(vector[1])] = 1
    return vents

def star2(vectors):
    vents = {}
    for vector in vectors:
        if vector[0] == vector[2]:
            if vector[1]>vector[3]:
                y_coordinates = range(vector[3],vector[1]+1)
            else:
                y_coordinates = range(vector[1],vector[3]+1)
            for y in y_coordinates:
                if (vector[0],y) in vents:
                    vents[(vector[0],y)] += 1
                else:
                    vents[(vector[0],y)] = 1
        elif vector[1] == vector[3]:
            if vector[0]>vector[2]:
                x_coordinates = range(vector[2],vector[0]+1)
            else:
                x_coordinates = range(vector[0],vector[2]+1)
            for x in x_coordinates:
                if (x,vector[1]) in vents:
                    vents[x,(vector[1])] += 1
                else:
                    vents[x,(vector[1])] = 1
        else:
            if vector[0]>vector[2]:
                x_coordinates = range(vector[2],vector[0]+1)
            else:
                x_coordinates = range(vector[2],vector[0]-1,-1)
            if vector[1]>vector[3]:
                y_coordinates = range(vector[3],vector[1]+1)
            else:
                y_coordinates = range(vector[3],vector[1]-1,-1)
            for i in range(len(x_coordinates)):
                if (x_coordinates[i],y_coordinates[i]) in vents:
                    vents[x_coordinates[i],y_coordinates[i]] += 1
                else:
                    vents[x_coordinates[i],y_coordinates[i]] = 1

    return vents

def main():
    with open("input.txt",'r') as vector_file:
        vector_lines = vector_file.readlines()

    vectors = process_input(vector_lines)
    
    vents = star1(vectors)
    critical = sum([1 if value>1 else 0 for value in vents.values()])
    print(critical)

    vents = star2(vectors)
    critical = sum([1 if value>1 else 0 for value in vents.values()])
    print(critical)
            
main()
