from copy import deepcopy
import inspect

f = open("twentythree.txt", "r")
lines = [x.strip() for x in f.readlines()]
         
goals = {1: 0, 10: 1, 100: 2, 1000: 3}
habitant = {0: 1, 1: 10, 2: 100, 3: 1000}
length = 2

def get_move_count(room, index, wait_places, wait_place, move_cost):
    moves = length - index
    
    if wait_place == 0:
        moves += 1
        wait_place = 1
    if wait_place == 7:
        moves += 1
        wait_place = 6
    goal_pos = room * 2 + 1
    cur_pos = (wait_place - 1) * 2
    
    for to_check in range(min(cur_pos, goal_pos), max(cur_pos, goal_pos) +1):
        if to_check % 2 == 0:
            if wait_places[to_check // 2 + 1] is not None and to_check != cur_pos:
                return -1
    
    moves += abs(cur_pos - goal_pos)
    return moves * move_cost

def find_best_move(rooms, wait_places, current_cost, best) -> int:
    if current_cost > best:
        return -1
    if all([val == 1 for val in rooms[0]]) and all([val == 10 for val in rooms[1]]) and all([val == 100 for val in rooms[2]]) and all([val == 1000 for val in rooms[3]]):
        return current_cost
    
    for i, w in enumerate(wait_places):
        if w is not None:
            if rooms[goals[w]][1] is None:
                if rooms[goals[w]][0] == w:
                    cost = get_move_count(goals[w], 1, wait_places, i, w)
                    if cost > 0:
                        rooms[goals[w]][1] = w
                        wait_places[i] = None
                        return find_best_move(deepcopy(rooms), deepcopy(wait_places), current_cost + cost, best)
                if rooms[goals[w]][0] is None:
                    cost = get_move_count(goals[w], 0, wait_places, i, w)
                    if cost > 0:
                        rooms[goals[w]][0] = w
                        wait_places[i] = None
                        return find_best_move(deepcopy(rooms), deepcopy(wait_places), current_cost + cost, best)
                    
    fbest = 1000000000000
    for i, r in enumerate(rooms):
        if r[1] is None and r[0] is not None and r[0] != habitant[i]:
            for k, w in enumerate(wait_places):
                if w is not None:
                    continue
                cost = get_move_count(i, 0, wait_places, k, r[0])
                if cost > 0:
                    _rooms = deepcopy(rooms)
                    _wait_places = deepcopy(wait_places)
                    _wait_places[k] = _rooms[i][0]
                    _rooms[i][0] = None
                    t_cost = find_best_move(_rooms, _wait_places, current_cost + cost, best)
                    if t_cost > 0 and t_cost < fbest:
                        fbest = t_cost
                        if current_cost == 0:
                            print(fbest)
        elif (r[1] != habitant[i] or r[0] != habitant[i]) and r[1] != None:
            for k, w in enumerate(wait_places):
                if w is not None:
                    continue
                cost = get_move_count(i, 1, wait_places, k, r[1])
                if cost > 0:
                    _rooms = deepcopy(rooms)
                    _wait_places = deepcopy(wait_places)
                    _wait_places[k] = _rooms[i][1]
                    _rooms[i][1] = None
                    t_cost = find_best_move(_rooms, _wait_places, current_cost + cost, best)
                    if t_cost > 0 and t_cost < fbest:
                        fbest = t_cost
                        if current_cost == 0:
                            print(fbest)
        
    return fbest

def find_best_move_p2(rooms, wait_places, current_cost) -> int:
    if all([val == 1 for val in rooms[0]]) and all([val == 10 for val in rooms[1]]) and all([val == 100 for val in rooms[2]]) and all([val == 1000 for val in rooms[3]]):
        return current_cost
    
    for i, w in enumerate(wait_places):
        if w is not None:
            for k in range(length - 1, -1, -1):
                if rooms[goals[w]][k] is not None:
                    break
                
                if k != 0 and rooms[goals[w]][k-1] is None:
                    continue 
                
                valid = True
                for j in range(k):
                    if rooms[goals[w]][j] != w:
                        valid = False
                        break
                
                if not valid:
                    break
                                
                cost = get_move_count(goals[w], k, wait_places, i, w)
                if cost > 0:
                    rooms[goals[w]][k] = w
                    wait_places[i] = None
                    return find_best_move_p2(deepcopy(rooms), deepcopy(wait_places), current_cost + cost)
                    
    fbest = 1000000000000
    for i, r in enumerate(rooms):
        needs_emptying = any([ro is not None and ro != habitant[i] for ro in r])
        if needs_emptying:
            for j in range(length - 1, -1, -1):
                if r[j] is not None:        
                    for k, w in enumerate(wait_places):
                        if w is not None:
                            continue
                        cost = get_move_count(i, j, wait_places, k, r[j])
                        if cost > 0:
                            _rooms = deepcopy(rooms)
                            _wait_places = deepcopy(wait_places)
                            _wait_places[k] = _rooms[i][j]
                            _rooms[i][j] = None
                            t_cost = find_best_move_p2(_rooms, _wait_places, current_cost + cost)
                            if t_cost > 0 and t_cost < fbest:
                                if current_cost == 0:
                                    print(t_cost)
                                fbest = t_cost
                    break
    return fbest

    
rooms = [[1000, 10], [1, 10], [1, 100], [100, 1000]]
wait_places =  [None, None, None, None, None, None, None]
print(find_best_move(rooms, wait_places, 0, 30000))
length = 4
rooms = [[1000, 1000, 1000, 10], [1, 10, 100, 10], [1, 1, 10, 100], [100, 100, 1, 1000]]
print(find_best_move_p2(rooms, wait_places, 0) - 2000)