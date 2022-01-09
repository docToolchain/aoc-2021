def process_input(file_contents):
    lines_stripped = [line.strip() for line in file_contents]
    
    all_combos = set()
    breed = {}
    for i in range(2,len(lines_stripped)):
        (key,value) = lines_stripped[i].split(" -> ")
        breed[key] = (key[0]+value,value+key[1])
        all_combos.update(breed[key])

    initial_seed = []
    for i in range(len(lines_stripped[0])-1):
        initial_seed.append(lines_stripped[0][i]+lines_stripped[0][i+1])
        all_combos.add(lines_stripped[0][i]+lines_stripped[0][i+1])
        
    all_combos = list(all_combos)
        
    return initial_seed, breed, all_combos

def main():
    with open("input.txt",'r') as poly_file:
        poly_lines = poly_file.readlines()

    (initial_seed, breed, all_combos) = process_input(poly_lines)
    
    all_letters = list(set([item for sublist in all_combos for item in sublist]))
    
    count_pairs = [0]*len(all_combos)
    
    for seed in initial_seed:
        count_pairs[all_combos.index(seed)] += 1
    
    for j in range(40): # replace with 10 for star 1
        count_pairs_temp = [0]*len(all_combos)
        for i,pair in enumerate(all_combos):
            new_pairs = breed[pair]
            count_pairs_temp[all_combos.index(new_pairs[0])] += count_pairs[i]
            count_pairs_temp[all_combos.index(new_pairs[1])] += count_pairs[i]
        count_pairs = count_pairs_temp
    
    count_letters = [0]*len(all_letters)
    for i,letter in enumerate(all_letters):
        for j,combo in enumerate(all_combos):
            for letter1 in combo:
                if letter == letter1:
                    count_letters[i] += count_pairs[j]
    
    count_fix = []
    for i,count in enumerate(count_letters):
        if all_letters[i] == initial_seed[0][0] or all_letters[i] == initial_seed[-1][1]:
            count_fix.append((count+1)//2)
        else:
            count_fix.append(count//2)
            
    print(max(count_fix)-min(count_fix))

main()
