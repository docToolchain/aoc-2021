from collections import Counter

file1 = open('input.txt', 'r')

syntax_error_map = {
    ")" : 3,
    "]" : 57,
    "}" : 1197,
    ">" : 25137
}

lines = list()
incomplete_lines = list()
for line in file1:
    lines.append(list(line.strip()))

def isOpeningBracket(bracket):
    if bracket == "<" or bracket == "{" or bracket == "[" or bracket == "(":
        return True
    else:
        return False

def isMatchingBracket(bracket1, bracket2):
    mapping = {
        "<": ">",
        ">": "<",
        "(": ")",
        ")": "(",
        "{": "}",
        "}": "{",
        "[": "]",
        "]": "[",       
        }
    #print(f"bracket 1: {bracket1}, bracket 2 {bracket2[0]}, ==? {mapping[bracket1]} = {mapping[bracket1] == bracket2}")
    if mapping[bracket1] == bracket2[0]:
        return True
    else:
        return False
    
syntax_error_score = 0
for line in lines:
    stack = list()
    #print(line)
    incomplete_line = True
    for bracket in line:
        if isOpeningBracket(bracket):
            stack.append(bracket)
        else:
            if isMatchingBracket(bracket, stack[-1:]):
                stack.pop()
            else:
                #print(f"illegal bracket found: {bracket}")
                syntax_error_score += syntax_error_map[bracket]
                incomplete_line = False
                break
    if incomplete_line:
        incomplete_lines.append(line)


print(f"Star 1: Total Syntax error score: {syntax_error_score}")

scores = list()
for line in incomplete_lines:
    stack = list()
    for bracket in line:
        if isOpeningBracket(bracket):
            stack.append(bracket)
        else:
            if isMatchingBracket(bracket, stack[-1:]):
                stack.pop()
    score = 0
    stack.reverse()
    for bracket in stack:
        score *= 5
        if bracket == "(":
            score += 1
        if bracket == "[":
            score += 2
        if bracket == "{":
            score += 3
        if bracket == "<":
            score += 4
    scores.append(score)

scores.sort()
print(f"Star 2: Middle score is: {scores[len(scores)//2]}")
