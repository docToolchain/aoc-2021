def process_input(file_contents):
    lines_stripped = [line.strip() for line in file_contents]
    
    fish = [int(i) for i in lines_stripped[0].split(",")]
    
    fish_states = [0]*9

    for lf in fish:
        fish_states[lf] += 1

    return fish_states

def main():
    with open("input.txt",'r') as fish_file:
        fish_lines = fish_file.readlines()

    fish_states = process_input(fish_lines)
    
    for day in range(256):
        fish_states_temp = fish_states.copy()
        for states in range(len(fish_states)):
            if states == 0:
                fish_states_temp[8] = fish_states[0]
                fish_states_temp[6] = fish_states[0]
            elif states == 7:
                fish_states_temp[6] += fish_states[7]
            else:
                fish_states_temp[states-1] = fish_states[states]
        fish_states = fish_states_temp
        if day == 79:
            print(sum(fish_states))

    print(sum(fish_states))
    
main()
