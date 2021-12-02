def process_input(file_contents):
    lines_stripped = [line.strip() for line in file_contents]

    lines_split = []
    for lines in lines_stripped:
        line = lines.split()
        lines_split.append((line[0], int(line[1])))
    
    return lines_split

def main():
    with open("input.txt",'r') as course_file:
        course_lines = course_file.readlines()

    course = process_input(course_lines)
    
    horizontal = 0
    depth = 0
    
    for directions in course:
        if directions[0] == 'forward':
            horizontal += directions[1]
        elif directions[0] == 'up':
            depth -= directions[1]
        else:
            depth += directions[1]

    print(horizontal, depth, depth*horizontal)
    
    horizontal = 0
    aim = 0
    depth = 0
    
    for directions in course:
        if directions[0] == 'forward':
            horizontal += directions[1]
            depth += aim*directions[1]
        elif directions[0] == 'up':
            aim -= directions[1]
        else:
            aim += directions[1]
            
    print(horizontal, depth, depth*horizontal)
    
main()
