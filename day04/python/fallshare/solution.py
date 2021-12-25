import re
import numpy as np
# read in input
# line ranmdom numbers 
# blank
    # 5 zeilen a 5 nummern 
#blank
  # 5 zeile
#am ende kein blank mehr 

# file einlesen in array?
# line 0 -> random numbers zerlenegne
# blank überspringen
# 5 zeilen ienlsen
# while current pos + 5


with open('input.txt') as file:
    lines = file.readlines()
    lines = [line.strip() for line in lines]

current_line = 0
random_numbers = lines[current_line].split(',')
random_numbers = list(map(int, random_numbers))

current_line += 2

bingo_boards = list()
while current_line + 5 <= len(lines):
    bingo_board = np.zeros((5,5), dtype=int)
    for x in range(5):
        bingo_line = re.split(r'\s+',lines[current_line])
        bingo_line = list(map(int, bingo_line))
        bingo_board[x] = np.asarray(bingo_line, dtype=np.int8)
       # bingo_board.append(bingo_line)
        current_line += 1
    bingo_boards.append(bingo_board)
    # skip over whitespace between bingo boards
    current_line += 1


#setup hit boxes
hit_boxes = list()
for board in bingo_boards:
    hit_boxes.append( np.zeros((5, 5),np.int8))

runs = 0
bingo = False
drawn_numbers = []
winner_board = 0
for number in random_numbers:
    if bingo:
        break
    drawn_numbers.append(number)
    
    runs += 1
    for board in range(len(bingo_boards)):
        for y in range(5):
            for x in range (5):
                if bingo_boards[board][y][x] == number:
                    hit_boxes[board][y][x] = 1
                # print(f"{x} - {y}")
                    #check if any row or column in correunt bingo board is a bingo
                    if (np.sum(hit_boxes[board], axis=0)[x] == 5) or (np.sum(hit_boxes[board], axis=1)[y] == 5):
                        print(f"Bingo found in board: {board} after {runs} runs with number {number}")
                        bingo = True
                        winner_board = board



print(f"Drawn numbers: {np.asarray(drawn_numbers)}")
print(f"Bingo numbers {np.asarray(bingo_boards[winner_board]).flatten()}")
remaining_numbers = np.setdiff1d(np.asarray(bingo_boards[winner_board]).flatten(),np.asarray(drawn_numbers))
print(f"Remaining numbers: {remaining_numbers}")
remaining_sum = np.sum(remaining_numbers)
print(f"Remaining sum: {remaining_sum}")
print(f"Solution Star 1: {remaining_sum} * {drawn_numbers[-1]}= {remaining_sum * drawn_numbers[-1]}")



#setup hit boxes
hit_boxes = list()
for board in bingo_boards:
    hit_boxes.append( np.zeros((5, 5),np.int8))



last_bingo = False
unfinished_boards = list(range(len(bingo_boards)))
drawn_numbers = []

for number in random_numbers:
    if last_bingo:
        break

    drawn_numbers.append(number)
 
   
    for board in unfinished_boards:
        remaining_bingo_boards = unfinished_boards.copy()            

        board_has_bingo = False
        for y in range(5):            
            if board_has_bingo:
                break
            for x in range (5):
                if board_has_bingo:
                    break
               
                if bingo_boards[board][y][x] == number:
                    hit_boxes[board][y][x] = 1

                    if (np.sum(hit_boxes[board], axis=0)[x] == 5) or (np.sum(hit_boxes[board], axis=1)[y] == 5):
                        if len(unfinished_boards) == 1:
                            print(f"Last Bingo found in {unfinished_boards[0]} in after {len(drawn_numbers)} runs with number {number}")
                            board_has_bingo = True
                            last_bingo = True
                        else:
                            remaining_bingo_boards.remove(board)
                            board_has_bingo = True

        unfinished_boards = remaining_bingo_boards


print(f"Drawn numbers: {np.asarray(drawn_numbers)}")
print(f"Bingo numbers {np.asarray(bingo_boards[unfinished_boards[0]]).flatten()}")
remaining_numbers = np.setdiff1d(np.asarray(bingo_boards[unfinished_boards[0]]).flatten(),np.asarray(drawn_numbers))
print(f"Remaining numbers: {remaining_numbers}")
remaining_sum = np.sum(remaining_numbers)
print(f"Remaining sum: {remaining_sum}")
print(f"Solution Star 2: {remaining_sum} * {drawn_numbers[-1]}= {remaining_sum * drawn_numbers[-1]}") 
