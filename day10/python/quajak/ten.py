f = open("ten.txt", "r")
lines = [x.strip() for x in f.readlines()]

score = 0
valid = []
for line in lines:
    stack = list()
    
    for ch in line:
        if ch == "(" or ch =="{" or ch =="[" or ch =="<":
            stack.append(ch)
        elif ch == ")":
            if stack[-1] == "(":
                stack.pop()
            else:
                score += 3
                break
        elif ch == "}":
            if stack[-1] == "{":
                stack.pop()
            else:
                score += 1197
                break
        elif ch == "]":
            if stack[-1] == "[":
                stack.pop()
            else:
                score += 57
                break
        elif ch == ">":
            if stack[-1] == "<":
                stack.pop()
            else:
                score += 25137
                break
    else:
        valid.append(line)
            
print(score)

score = []

for line in valid:
    stack = []
    for ch in line:
        if ch == "(" or ch =="{" or ch =="[" or ch =="<":
            stack.append(ch)
        elif ch == ")":
            if stack[-1] == "(":
                stack.pop()
        elif ch == "}":
            if stack[-1] == "{":
                stack.pop()
        elif ch == "]":
            if stack[-1] == "[":
                stack.pop()
        elif ch == ">":
            if stack[-1] == "<":
                stack.pop()
        
    s = 0        
    stack.reverse()
    for ch in stack:
        s *= 5
        if ch == "(":
            s += 1
        elif ch == "[":
            s += 2
        elif ch == "{":
            s += 3
        else:
            s += 4
    score.append(s)
            
score = sorted(score)
print(score[len(score)//2])