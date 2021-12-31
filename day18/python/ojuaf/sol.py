
#!/usr/bin/env python3

import re
import math as m

stop_value = False

# from binarytree import tree, Node

class Node(object):
    def __init__(self):
        self.left = None
        self.right = None
        self.parent = None
        self.value = None

pattern = re.compile("target area: x=(\d+)..(\d+), y=(-\d+)..(-\d+)")

def load_input():
    data = list()
    with open('input') as fd:
        for line in fd:
            line = line.strip()
            data.append(eval(line))
    return data

def _build_tree(root, data):
    root.right = Node()
    root.left = Node()
    root.left.parent = root
    root.right.parent = root
    if isinstance(data[0], int):
        root.left.value = data[0]
    else:
        _build_tree(root.left, data[0])
    if isinstance(data[1], int):
        root.right.value = data[1]
    else:
        _build_tree(root.right, data[1])

def create_tree(data):
    root = Node()
    _build_tree(root, data)
    return root

def get_tree_str(root):
    ret_val = "["
    if root.left.value is not None:
        ret_val += str(root.left.value)
    else:
        ret_val += get_tree_str(root.left)
    ret_val += ','
    if root.right.value is not None:
        ret_val += str(root.right.value)
    else:
        ret_val += get_tree_str(root.right)
    ret_val += ']'
    return ret_val

def find_right_leave(nd):
    if nd.value is not None:
        return nd
    else:
        return find_right_leave(nd.right)

def find_left_node(nd):
    ret_val = None
    if nd.parent is not None:
        if nd.parent.left is nd:
            ret_val = find_left_node(nd.parent)
        else:
            ret_val = find_right_leave(nd.parent.left)
    return ret_val

def find_left_leave(nd):
    if nd.value is not None:
        return nd
    else:
        return find_left_leave(nd.left)

def find_right_node(nd):
    ret_val = None
    if nd.parent is not None:
        if nd.parent.right is nd:
            ret_val = find_right_node(nd.parent)
        else:
            ret_val = find_left_leave(nd.parent.right)
    return ret_val

def explode(root, layer):
    if root.left is not None and root.left.value is not None and root.right.value is not None and layer >= 4:
        right_value = root.right.value
        left_value = root.left.value

        left_node = find_left_node(root)
        if left_node is not None:
            left_node.value += left_value

        right_node = find_right_node(root)
        if right_node is not None:
            right_node.value += right_value

        root.value = 0
        root.left = None
        root.right = None
        return True
    return False

def split(root, layer):
    if root.value is not None and root.value > 9:
        left_value = root.value//2
        right_value = int(m.ceil(root.value/2))

        left_node = Node()
        left_node.value = left_value
        left_node.parent = root

        right_node = Node()
        right_node.value = right_value
        right_node.parent = root

        root.left = left_node
        root.right = right_node
        root.value = None
        return True
    return False
    
def apply_action(root, layer, func):
    global stop_value

    if stop_value == True:
       return

    stop_value = func(root, layer)

    if not stop_value:
        if root.left is not None:
            apply_action(root.left, layer+1, func)

        if root.right is not None:
            apply_action(root.right, layer+1, func)
    return

def calc_magnitude(root):
    result = 0
    if root.left is not None and root.left.value is not None:
        result += root.left.value*3
    else:
        result += 3*calc_magnitude(root.left)
    if root.right is not None and root.right.value is not None:
        result += 2*root.right.value
    else:
        result += 2*calc_magnitude(root.right)
    return result
    
def add_list(left_node, right_node):
    root = Node()
    root.left = left_node
    root.left.parent = root
    root.right = right_node
    root.right.parent = root
    return root

def part1():
    # First Part
    global stop_value
    data = load_input()

    left_node = create_tree(data[0])

    for i in range(1, len(data)):
        right_node = create_tree(data[i])
        root = add_list(left_node, right_node)
        while True:
            stop_value = False
            apply_action(root, 0, explode)
            if stop_value:
                continue
            apply_action(root, 0, split)
            if not stop_value:
                break
        left_node = root
    result = calc_magnitude(root)
    print("Part 1:", result)
    return


def part2():
    # Second part
    global stop_value
    data = load_input()

    results = list()
    for i in range(0, len(data)):
        for j in range(0, len(data)):
            if i != j:
                left_node = create_tree(data[i])
                right_node = create_tree(data[j])
                root = add_list(left_node, right_node)
                while True:
                    stop_value = False
                    apply_action(root, 0, explode)
                    if stop_value:
                        continue
                    apply_action(root, 0, split)
                    if not stop_value:
                        break
                results.append(calc_magnitude(root))
    result = max(results)
    print("Part 2:", result)
    return


if __name__ == '__main__':
    part1()
    part2()
