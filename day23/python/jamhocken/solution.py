def process_input(file_contents):

    grid = [0]*7
    for i in[3,5,7,9]:
        for j in [2,3]:
            grid.append(file_contents[j][i])

    return tuple(grid)

def find_neighbors_star1(current_node,goals,paths,distance):
    neighbors = list()
    neighbor_dict = dict()
    multiplier = {'A':1, 'B':10, 'C':100, 'D':1000}

    for i, position in enumerate(current_node):
        if position != 0 and i != goals[position][1] and not (i == goals[position][0] and current_node[goals[position][1]]==position):
            if i<7:
                if current_node[goals[position][0]] == 0:
                    if current_node[goals[position][1]] == position:
                       if not sum([1 if current_node[k] != 0 else 0 for k in paths[(i,goals[position][0])]]):
                           neighbor = list(current_node)
                           neighbor[goals[position][0]] = position
                           neighbor[i] = 0
                           neighbors.append(tuple(neighbor))
                           neighbor_dict[tuple(neighbor)] = distance[(i,goals[position][0])]*multiplier[position]
                           break
                    elif current_node[goals[position][1]] == 0:
                        if not sum([1 if current_node[k] != 0 else 0 for k in paths[(i,goals[position][1])]]):
                           neighbor = list(current_node)
                           neighbor[goals[position][1]] = position
                           neighbor[i] = 0
                           neighbors.append(tuple(neighbor))
                           neighbor_dict[tuple(neighbor)] = distance[(i,goals[position][1])]*multiplier[position]                        
                           break
            else:
                for j in range(7):
                    if current_node[j] == 0:
                        if not sum([1 if current_node[k] != 0 else 0 for k in paths[(j,i)]]):
                            neighbor = list(current_node)
                            neighbor[j] = position
                            neighbor[i] = 0
                            neighbors.append(tuple(neighbor))
                            neighbor_dict[tuple(neighbor)] = distance[(j,i)]*multiplier[position]

    neighbors = set(neighbors)

    return neighbors, neighbor_dict

def find_neighbors_star2(current_node,goals,paths,distance):
    neighbors = list()
    neighbor_dict = dict()
    multiplier = {'A':1, 'B':10, 'C':100, 'D':1000}

    for i, position in enumerate(current_node):
        if position != 0 and i != goals[position][3] and not (i == goals[position][2] and current_node[goals[position][3]]==position) and not (i == goals[position][1] and current_node[goals[position][2]]==position and current_node[goals[position][3]]==position) and not (i == goals[position][0] and current_node[goals[position][1]]==position  and current_node[goals[position][2]]==position and current_node[goals[position][3]]==position):
            if i<7:
                if current_node[goals[position][0]] == 0:
                    if not sum([1 if current_node[goals[position][k]] != position else 0 for k in [1,2,3]]):
                       if not sum([1 if current_node[k] != 0 else 0 for k in paths[(i,goals[position][0])]]):
                           neighbor = list(current_node)
                           neighbor[goals[position][0]] = position
                           neighbor[i] = 0
                           neighbors.append(tuple(neighbor))
                           neighbor_dict[tuple(neighbor)] = distance[(i,goals[position][0])]*multiplier[position]
                           break
                    elif current_node[goals[position][1]] == 0:
                        if not sum([1 if current_node[goals[position][k]] != position else 0 for k in [2,3]]):
                            if not sum([1 if current_node[k] != 0 else 0 for k in paths[(i,goals[position][1])]]):
                                neighbor = list(current_node)
                                neighbor[goals[position][1]] = position
                                neighbor[i] = 0
                                neighbors.append(tuple(neighbor))
                                neighbor_dict[tuple(neighbor)] = distance[(i,goals[position][1])]*multiplier[position]
                                break
                        elif current_node[goals[position][2]] == 0:
                            if not sum([1 if current_node[goals[position][k]] != position else 0 for k in [3]]):
                                if not sum([1 if current_node[k] != 0 else 0 for k in paths[(i,goals[position][2])]]):
                                    neighbor = list(current_node)
                                    neighbor[goals[position][2]] = position
                                    neighbor[i] = 0
                                    neighbors.append(tuple(neighbor))
                                    neighbor_dict[tuple(neighbor)] = distance[(i,goals[position][2])]*multiplier[position]
                                    break
                            elif current_node[goals[position][3]] == 0:
                                if not sum([1 if current_node[k] != 0 else 0 for k in paths[(i,goals[position][3])]]):
                                    neighbor = list(current_node)
                                    neighbor[goals[position][3]] = position
                                    neighbor[i] = 0
                                    neighbors.append(tuple(neighbor))
                                    neighbor_dict[tuple(neighbor)] = distance[(i,goals[position][3])]*multiplier[position]
                                    break
            else:
                for j in range(7):
                    if current_node[j] == 0:
                        if not sum([1 if current_node[k] != 0 else 0 for k in paths[(j,i)]]):
                            neighbor = list(current_node)
                            neighbor[j] = position
                            neighbor[i] = 0
                            neighbors.append(tuple(neighbor))
                            neighbor_dict[tuple(neighbor)] = distance[(j,i)]*multiplier[position]

    neighbors = set(neighbors)

    return neighbors, neighbor_dict

def find_shortest_path(nodes,goals,paths,distance,star):
    if star == 1:
        end_node = (0,0,0,0,0,0,0,'A','A','B','B','C','C','D','D')
    else:
        end_node = (0,0,0,0,0,0,0,'A','A','A','A','B','B','B','B','C','C','C','C','D','D','D','D')
    visited = set()
    current_node = nodes
    energy_dict = {current_node:0}
    valued = {current_node}
    while current_node != end_node:
        if star == 1:
            (neighbors, neighbor_dict) = find_neighbors_star1(current_node,goals,paths,distance)
        else:
            (neighbors, neighbor_dict) = find_neighbors_star2(current_node,goals,paths,distance)

        valued.update(neighbors-visited)
        for neighbor in neighbors:
            if neighbor not in visited:
                if neighbor not in energy_dict.keys():
                    energy_dict[neighbor] = energy_dict[current_node] + neighbor_dict[neighbor]
                else:
                    energy_dict[neighbor] = min(energy_dict[neighbor],energy_dict[current_node]+neighbor_dict[neighbor])

        visited.add(current_node)
        valued.remove(current_node)
        energy_dict.pop(current_node)

        min_distance = min([energy_dict[key] for key in valued])
        for node in valued:
            if energy_dict[node] == min_distance:
                current_node = node
                break

    energy = min_distance

    return energy

def main():
    with open("input.txt",'r') as amph_file:
        amph_lines = amph_file.readlines()

    grid = process_input(amph_lines)

    goals = {"A":(7,8),"B":(9,10),"C":(11,12),"D":(13,14)}
    paths = {(0,7):{1},(0,8):{1,7},(0,9):{1,2},(0,10):{1,2,9},(0,11):{1,2,3},(0,12):{1,2,3,11},(0,13):{1,2,3,4},(0,14):{1,2,3,4,13}}
    paths.update({(1,7):{},(1,8):{7},(1,9):{2},(1,10):{2,9},(1,11):{2,3},(1,12):{2,3,11},(1,13):{2,3,4},(1,14):{2,3,4,13}})
    paths.update({(2,7):{},(2,8):{7},(2,9):{},(2,10):{9},(2,11):{3},(2,12):{3,11},(2,13):{3,4},(2,14):{3,4,13}})
    paths.update({(3,7):{2},(3,8):{2,7},(3,9):{},(3,10):{9},(3,11):{},(3,12):{11},(3,13):{4},(3,14):{4,13}})
    paths.update({(4,7):{2,3},(4,8):{2,3,7},(4,9):{3},(4,10):{3,9},(4,11):{},(4,12):{11},(4,13):{},(4,14):{13}})
    paths.update({(5,7):{2,3,4},(5,8):{2,3,4,7},(5,9):{3,4},(5,10):{3,4,9},(5,11):{4},(5,12):{4,11},(5,13):{},(5,14):{13}})
    paths.update({(6,7):{2,3,4,5},(6,8):{2,3,4,5,7},(6,9):{3,4,5},(6,10):{3,4,5,9},(6,11):{4,5},(6,12):{4,5,11},(6,13):{5},(6,14):{5,13}})
    distance = {(0,7):3,(0,8):4,(0,9):5,(0,10):6,(0,11):7,(0,12):8,(0,13):9,(0,14):10}
    distance.update({(1,7):2,(1,8):3,(1,9):4,(1,10):5,(1,11):6,(1,12):7,(1,13):8,(1,14):9})
    distance.update({(2,7):2,(2,8):3,(2,9):2,(2,10):3,(2,11):4,(2,12):5,(2,13):6,(2,14):7})
    distance.update({(3,7):4,(3,8):5,(3,9):2,(3,10):3,(3,11):2,(3,12):3,(3,13):4,(3,14):5})
    distance.update({(4,7):6,(4,8):7,(4,9):4,(4,10):5,(4,11):2,(4,12):3,(4,13):2,(4,14):3})
    distance.update({(5,7):8,(5,8):9,(5,9):6,(5,10):7,(5,11):4,(5,12):5,(5,13):2,(5,14):3})
    distance.update({(6,7):9,(6,8):10,(6,9):7,(6,10):8,(6,11):5,(6,12):6,(6,13):3,(6,14):4})

    energy = find_shortest_path(grid,goals,paths,distance,1)
    print(energy)
    
    grid_temp = [0]*23
    for i in range(8):
        grid_temp[i] = grid[i]
    grid_temp[8] = "D"
    grid_temp[9] = "D"
    grid_temp[10] = grid[8]
    grid_temp[11] = grid[9]
    grid_temp[12] = "C"
    grid_temp[13] = "B"
    grid_temp[14] = grid[10]
    grid_temp[15] = grid[11]
    grid_temp[16] = "B"
    grid_temp[17] = "A"
    grid_temp[18] = grid[12]
    grid_temp[19] = grid[13]
    grid_temp[20] = "A"
    grid_temp[21] = "C"
    grid_temp[22] = grid[14]
    grid = tuple(grid_temp)
    
    goals = {"A":(7,8,9,10),"B":(11,12,13,14),"C":(15,16,17,18),"D":(19,20,21,22)}

    paths = {(0,7):{1},(0,8):{1,7},(0,9):{1,7,8},(0,10):{1,7,8,9},(0,11):{1,2},(0,12):{1,2,11},(0,13):{1,2,11,12},(0,14):{1,2,11,12,13},(0,15):{1,2,3},(0,16):{1,2,3,15},(0,17):{1,2,3,15,16},(0,18):{1,2,3,15,16,17},(0,19):{1,2,3,4},(0,20):{1,2,3,4,19},(0,21):{1,2,3,4,19,20},(0,22):{1,2,3,4,19,20,21}}
    distance = {(0,7):3,(0,8):4,(0,9):5,(0,10):6,(0,11):5,(0,12):6,(0,13):7,(0,14):8,(0,15):7,(0,16):8,(0,17):9,(0,18):10,(0,19):9,(0,20):10,(0,21):11,(0,22):12}
    for k in range(7,23):
        paths[(1,k)] = paths[(0,k)] - {1}
        distance[(1,k)] = distance[(0,k)] - 1
    for k in range(7,11):
        paths[(2,k)] = paths[(1,k)]
        paths[(3,k)] = paths[(2,k)] | {2}
        paths[(4,k)] = paths[(3,k)] | {3}
        paths[(5,k)] = paths[(4,k)] | {4}
        paths[(6,k)] = paths[(5,k)] | {5}
        distance[(2,k)] = distance[(1,k)]
        distance[(3,k)] = distance[(2,k)] + 2
        distance[(4,k)] = distance[(3,k)] + 2
        distance[(5,k)] = distance[(4,k)] + 2
        distance[(6,k)] = distance[(5,k)] + 1
    for k in range(11,15):
        paths[(2,k)] = paths[(1,k)] - {2}
        paths[(3,k)] = paths[(2,k)]
        paths[(4,k)] = paths[(3,k)] | {3}
        paths[(5,k)] = paths[(4,k)] | {4}
        paths[(6,k)] = paths[(5,k)] | {5}
        distance[(2,k)] = distance[(1,k)] - 2
        distance[(3,k)] = distance[(2,k)]
        distance[(4,k)] = distance[(3,k)] + 2
        distance[(5,k)] = distance[(4,k)] + 2
        distance[(6,k)] = distance[(5,k)] + 1
    for k in range(15,19):
        paths[(2,k)] = paths[(1,k)] - {2}
        paths[(3,k)] = paths[(2,k)] - {3}
        paths[(4,k)] = paths[(3,k)]
        paths[(5,k)] = paths[(4,k)] | {4}
        paths[(6,k)] = paths[(5,k)] | {5}
        distance[(2,k)] = distance[(1,k)] - 2
        distance[(3,k)] = distance[(2,k)] - 2
        distance[(4,k)] = distance[(3,k)]
        distance[(5,k)] = distance[(4,k)] + 2
        distance[(6,k)] = distance[(5,k)] + 1
    for k in range(19,23):
        paths[(2,k)] = paths[(1,k)] - {2}
        paths[(3,k)] = paths[(2,k)] - {3}
        paths[(4,k)] = paths[(3,k)] - {4}
        paths[(5,k)] = paths[(4,k)]
        paths[(6,k)] = paths[(5,k)] | {5}
        distance[(2,k)] = distance[(1,k)] - 2
        distance[(3,k)] = distance[(2,k)] - 2
        distance[(4,k)] = distance[(3,k)] - 2
        distance[(5,k)] = distance[(4,k)]
        distance[(6,k)] = distance[(5,k)] + 1

    energy = find_shortest_path(grid,goals,paths,distance,2)
    print(energy)
     
main()
