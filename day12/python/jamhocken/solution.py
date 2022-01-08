def process_input(file_contents):
    lines_stripped = [line.strip() for line in file_contents]
    
    paths_dict = {}
    for line in lines_stripped:
        vertices = line.split("-")
        if vertices[0] in paths_dict:
            temp = paths_dict[vertices[0]]
            temp.add(vertices[1])
            paths_dict[vertices[0]] = temp
        else:
            paths_dict[vertices[0]] = {vertices[1]}
        if vertices[1] in paths_dict:
            temp = paths_dict[vertices[1]]
            temp.add(vertices[0])
            paths_dict[vertices[1]] = temp
        else:
            paths_dict[vertices[1]] = {vertices[0]}
            
    return paths_dict

def traverse_graph_star1(paths,start,end,path,doubles,found_paths):
    path.append(start)
    if start[0]>"Z":
        doubles.add(start)
    if start == end:
        found_paths.append(path.copy())
    else:
        for child in paths[start].difference(doubles):
            traverse_graph_star1(paths,child,end,path,doubles,found_paths)
    path.pop()
    if start[0]>"Z":
        doubles.remove(start)     
    
    return

def traverse_graph_star2(paths,start,end,path,doubles,found_paths):
    flag = 0
    if start=="end" or start=="start":
        doubles.add(start)
        flag = 1
    elif start[0]>"Z" and len(doubles)>2:
        doubles.add(start)
        flag = 1
    elif start[0]>"Z" and start in path:
        doubles.add(start)
        flag = 1
    path.append(start)
    if start == end:
        found_paths.append(path.copy())
    else:
        for child in paths[start].difference(doubles):
            if len(doubles)<2 or child[0]<"a" or child not in path:
                traverse_graph_star2(paths,child,end,path,doubles,found_paths)
    path.pop()

    if flag == 1:
        doubles.remove(start)

    return

def main():
    with open("input.txt",'r') as paths_file:
        path_lines = paths_file.readlines()

    paths = process_input(path_lines)
    
    #star 1
    start = "start"
    end = "end"
    path = []
    doubles = set()
    found_paths = []
    
    traverse_graph_star1(paths,start,end,path,doubles,found_paths)
    
    print(len(found_paths))
    
    #star 2
    path = []
    doubles = set()
    found_paths = []

    traverse_graph_star2(paths,start,end,path,doubles,found_paths)
    
    print(len(found_paths))

main()
