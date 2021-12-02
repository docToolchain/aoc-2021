def process_input(file_contents):
    lines_stripped = [int(line.strip()) for line in file_contents]

    return lines_stripped

def main():
    with open("input.txt",'r') as depths_file:
        file_lines = depths_file.readlines()

    depths = process_input(file_lines)
    
    count = 0

    for i in range(len(depths)-1):
        if depths[i+1]>depths[i]:
            count += 1
    
    print(count)
    
    window = []
    for i in range(len(depths)-2):
        window.append(depths[i]+depths[i+1]+depths[i+2])
    
    count = 0
    for i in range(len(window)-1):
        if window[i+1]>window[i]:
            count += 1
            
    print(count)
    
main()
