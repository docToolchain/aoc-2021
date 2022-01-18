import collections

def process_input(file_contents):
    lines_stripped = [line.strip() for line in file_contents]
    position = list()
    position.append(int(lines_stripped[0][-1]))
    position.append(int(lines_stripped[1][-1]))
        
    return position

def main():
    with open("input.txt",'r') as player_file:
        player_lines = player_file.readlines()

    position = process_input(player_lines)
    
    # star 1
    score = [0,0]
    die = collections.deque([i for i in range(1,101)])
    i = 0
    j = 0
    
    while max(score)<1000:
        player = 0
        for k in range(3):
            player += die[0]
            die.rotate(-1)
        position[i] = (position[i]+player-1)%10+1
        score[i] += position[i]
        i = (i+1)%2
        j += 3

    print(j*min(score))
    
    position = process_input(player_lines)
    # star 2
    adder_dict = dict()
    for i in range(1,4):
        for j in range(1,4):
            for k in range(1,4):
                if i+j+k in adder_dict:
                    adder_dict[i+j+k] += 1
                else:
                    adder_dict[i+j+k] = 1
    
    sp1_sp2_dict = dict()
    for key1, value1 in adder_dict.items():
        for key2, value2 in adder_dict.items(): 
            score1 = (position[0]+key1-1)%10+1
            score2 = (position[1]+key2-1)%10+1
            sp1_sp2_dict[(score1,score1,score2,score2)] = value1*value2
    
    player1_wins = 0
    player2_wins = 0
    while sp1_sp2_dict:
        sp1_sp2_dict_temp = dict()
        for key,value in sp1_sp2_dict.items():
            for key1,value1 in adder_dict.items():
                position1 = (key[1]+key1-1)%10+1
                score1 = key[0]+position1
                if score1>=21:
                    player1_wins += value*value1
                else:
                    for key2,value2 in adder_dict.items():
                        position2 = (key[3]+key2-1)%10+1
                        score2 = key[2]+position2
                        if score2 >= 21:
                            player2_wins += value*value1*value2
                        elif (score1,position1,score2,position2) in sp1_sp2_dict_temp:
                            sp1_sp2_dict_temp[(score1,position1,score2,position2)] = sp1_sp2_dict_temp[(score1,position1,score2,position2)] + value*value1*value2
                        else:
                            sp1_sp2_dict_temp[(score1,position1,score2,position2)] = value*value1*value2 
        sp1_sp2_dict = sp1_sp2_dict_temp
            
    print(max(player1_wins,player2_wins))

main()
