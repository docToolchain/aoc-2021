def process_input(file_contents):
    lines_stripped = [line.strip() for line in file_contents]
    
    inputs = [lines.split("|")[0].split() for lines in lines_stripped]
    outputs = [lines.split("|")[1].split() for lines in lines_stripped]

    return inputs, outputs

def main():
    with open("input.txt",'r') as segment_file:
        segment_lines = segment_file.readlines()

    inputs,outputs = process_input(segment_lines)

    #star 1
    uniques = 0
    flat_list = [item for sublist in outputs for item in sublist]
    for item in flat_list:
        if len(item) in [2,3,4,7]:
            uniques += 1 
    print(uniques)
    
    #star 2
    inputs = [sorted([frozenset(segment) for segment in i],key=len) for i in inputs]
    outputs = [[frozenset(segment) for segment in i] for i in outputs]   

    sum_digits = 0    
    for j in range(len(inputs)):
        mapping = {inputs[j][0]:1,inputs[j][1]:7,inputs[j][2]:4,inputs[j][9]:8}
        for i in [3,4,5]:
            if inputs[j][1].issubset(inputs[j][i]):
                mapping[inputs[j][i]] = 3
            elif (inputs[j][2]- inputs[j][0]).issubset(inputs[j][i]):
                mapping[inputs[j][i]] = 5
            else:
                mapping[inputs[j][i]] = 2
        for i in [6,7,8]:
            if inputs[j][2].issubset(inputs[j][i]):
                mapping[inputs[j][i]] = 9
            elif inputs[j][0].issubset(inputs[j][i]):
                mapping[inputs[j][i]] = 0
            else:
                mapping[inputs[j][i]] = 6
        digits = int("".join([str(mapping[item]) for item in outputs[j]]))
        sum_digits += digits
        
    print(sum_digits)
            
main()
