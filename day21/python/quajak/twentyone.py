from typing import List, Tuple


f = open("twentyone.txt", "r")
lines = [x.strip() for x in f.readlines()]

pos = [4, 6]
score = [0, 0]
die = 1
num = 0
turn = 0

while score[0] < 1000 and score[1] < 1000:
    dis = 3 * die  + 3
    die += 3
    pos[turn] += dis
    pos[turn] = (pos[turn] - 1) % 10 + 1
    score[turn] += pos[turn]
    turn += 1
    turn %= 2
    num += 3
    
print(score[turn]* num)

known_pos = {}
rolls = {3:1, 4:3, 5:6, 6:7, 7:6, 8:3, 9:1}
def play_turn(cur: int, pos: List[int], score: List[int]) -> List[int]:
    if score[1 - cur] >= 21:
        res = [0,0]
        res[1 - cur] = 1
        return res
    
    state = (pos[0], pos[1], score[0], score[1], cur)
    if state in known_pos:
        return known_pos[state]
    results = [0,0]
    for die in rolls.keys():
        npos = list(pos)
        npos[cur] = (npos[cur] + die - 1) % 10 + 1
        nscore = list(score)
        nscore[cur] += npos[cur]
        res = play_turn(1 - cur, npos, nscore)
        results[0] += res[0] * rolls[die]
        results[1] += res[1] * rolls[die]
    known_pos[state] = results
    return results

print(max(play_turn(0, [4,6], [0, 0])))