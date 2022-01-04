import numpy as np

def process_input(file_contents):
    lines_stripped = [line.strip() for line in file_contents]

    numbers_called = [int(x) for x in lines_stripped[0].split(",")]
    cards = []

    i = 2
    while i < len(lines_stripped):
        bingo_card = []
        marker_card = []
        for j in range(5):
            bingo_card.append([int(x) for x in lines_stripped[i+j].split()])
            marker_card.append([1]*5)
        cards.append([bingo_card,marker_card])
        i += 6

    return numbers_called, cards

def main():
    with open("input.txt",'r') as bingo_file:
        bingo_lines = bingo_file.readlines()

    (numbers_called, cards) = process_input(bingo_lines)

    card_arrays = [[np.array(bingo_card),np.array(marker_card)] for bingo_card, marker_card in cards]
    bingo = 0
    j = -1
    non_winners = set(range(len(card_arrays)))
    nw_temp = non_winners.copy()
    
    while len(non_winners)>0:
        j += 1
        for i in non_winners:
            card_arrays[i][1] = np.where(card_arrays[i][0]==numbers_called[j],0,card_arrays[i][1])
            if (~card_arrays[i][1].any(axis=0)).any() or (~card_arrays[i][1].any(axis=1)).any():
               if bingo == 0:
                   bingo = 1
                   winning_card = i
                   print(np.sum(np.multiply(card_arrays[winning_card][0],card_arrays[winning_card][1]))*numbers_called[j])
               if len(non_winners)==1:
                   losing_card = i
               nw_temp.remove(i)
        non_winners = nw_temp.copy()
    
    print(np.sum(np.multiply(card_arrays[losing_card][0],card_arrays[losing_card][1]))*numbers_called[j])

main()
