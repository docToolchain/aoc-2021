from os import path
from typing import Dict, List
import copy

f = open("twelve.txt", "r")
lines = [x.strip() for x in f.readlines()]


class Node:
    def __init__(self, name):
        self.edges = []
        self.name = name
        self.small = name[0].islower()

places = []
nodes : Dict[str, Node] = {}

for line in lines:
    parts = line.split("-")
    if parts[0] not in places:
        places.append(parts[0])
        nodes[parts[0]] = Node(parts[0])
    if parts[1] not in places:
        places.append(parts[1])
        nodes[parts[1]] = Node(parts[1])
        
    nodes[parts[0]].edges.append(parts[1])
    nodes[parts[1]].edges.append(parts[0])
    
def remove(name: str, graph: Dict[str, Node]):
        del graph[name]
        to_remove = set()
        for node in graph.keys():
            if name in graph[node].edges:
                graph[node].edges.remove(name)
                if len(graph[node].edges) == 0:
                    to_remove.add(graph[node].name)
        
        for rem in to_remove:
            remove(rem, graph)
            
def valid_paths_to_end(current: Node, graph: Dict[str, Node]) -> List[List[str]]:
    if current.name == "end":
        return [["end"]]
    if current.small:
        graph = copy.deepcopy(graph)
        remove(current.name, graph)
                
    ways = []

    for edge in current.edges:
        if edge in graph:
            ways.extend([[current.name] + path for path in valid_paths_to_end(graph[edge], graph)])
        
    return ways

print(len(valid_paths_to_end(nodes["start"], nodes)))

places = []
nodes : Dict[str, Node] = {}

for line in lines:
    parts = line.split("-")
    if parts[0] not in places:
        places.append(parts[0])
        nodes[parts[0]] = Node(parts[0])
    if parts[1] not in places:
        places.append(parts[1])
        nodes[parts[1]] = Node(parts[1])
        
    nodes[parts[0]].edges.append(parts[1])
    nodes[parts[1]].edges.append(parts[0])
    
def continue_path(path : List[str], current: Node, graph: Dict[str, Node], has_double: bool) -> List[List[str]]:
    if has_double and (current.small and current.name in path):
        return []
    if current.name == "end":
        return [path + ["end"]]
    if current.small and current.small in path or current.name == "start":
        graph = copy.deepcopy(graph)
        remove(current.name, graph)
                
    ways = []
    for edge in current.edges:
        if edge in graph:
            ways.extend(continue_path(path + [current.name], graph[edge], graph, has_double or (current.small and current.name in path)))
    return ways

print(len(continue_path([], nodes["start"], nodes, False)))
