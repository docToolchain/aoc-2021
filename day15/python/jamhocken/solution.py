def return_first(elem):
    return elem[0]
        
def process_input_star1(file_contents):
    lines_stripped = [line.strip() for line in file_contents]
    
    nodes = list()
    line_length = len(lines_stripped[0])
    for i,line in enumerate(lines_stripped):
        for j,risk in enumerate(line):
            adjacent = set()
            index = i*line_length+j
            if i != 0:
                adjacent.add(index-line_length)
            if i != line_length-1:
                adjacent.add(index+line_length)
            if index % line_length != 0:
                adjacent.add(index-1)
            if (index+1) % (line_length) != 0:
                adjacent.add(index+1)
            if index == 0:
                distance = 0
            else:
                distance = float('Inf')
            nodes.append([index,int(risk),distance,adjacent,0])
    
    return nodes

def process_input_star2(file_contents):
    lines_stripped = [line.strip() for line in file_contents]
    
    nodes = list()
    line_length = len(lines_stripped[0])
    for k in range(5):
        for i,line in enumerate(lines_stripped):
            for m in range(5):    
                for j,risk in enumerate(line):
                    adjacent = set()
                    index = (i+m*line_length)*line_length*5+j+k*line_length
                    risk = int(risk)+m+k
                    if risk > 9:
                        risk = risk - 9
                    if index > line_length*5-1:
                        adjacent.add(index-line_length*5)
                    if index < line_length*5*line_length*5-line_length*5:
                        adjacent.add(index+line_length*5)
                    if index % (5*line_length) != 0:
                        adjacent.add(index-1)
                    if (index+1) % (5*line_length) != 0:
                        adjacent.add(index+1)
                    if index == 0:
                        distance = 0
                    else:
                        distance = float('Inf')
                    nodes.append([index,risk,distance,adjacent,0])
                    
    nodes.sort(key=return_first)
    
    return nodes

def find_shortest_path(nodes):
    visited = set()
    current_node = 0
    valued = {current_node}
    while current_node != len(nodes)-1:
        neighbors = nodes[current_node][3]
        valued.update(neighbors-visited)
        for neighbor in neighbors:
            if neighbor not in visited:
                if nodes[neighbor][2] > nodes[current_node][2]+nodes[neighbor][1]:
                    nodes[neighbor][2] = nodes[current_node][2]+nodes[neighbor][1]
                    nodes[neighbor][4] = current_node
            
        visited.add(current_node)
        valued.remove(current_node)
        min_distance = min([nodes[node][2] for node in valued])
        
        for node in valued:
            if nodes[node][2] == min_distance:
                current_node = node

    risk = nodes[current_node][2]  

    return risk
    

def main():
    with open("input.txt",'r') as chiton_file:
        chiton_lines = chiton_file.readlines()

    #star 1
    nodes = process_input_star1(chiton_lines)   

    risk = find_shortest_path(nodes)
    
    print(risk)
    
    #star 2
    nodes = process_input_star2(chiton_lines)
    
    risk = find_shortest_path(nodes)
    
    print(risk)
    

main()
