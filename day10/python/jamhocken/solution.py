def process_input(file_contents):
    lines_stripped = [line.strip() for line in file_contents]
    
    return lines_stripped

def check_line(navicode,opener,stack):
    status = ""
    corruptor = ""
    while len(navicode) > 1 and len(status)==0:
        navicode = navicode[1:]
        status = check_match(opener,navicode[0])
        if status == "corrupt":
            return navicode, "corrupt", navicode[0],stack
        elif status == "match":
            if len(stack[:-1])>0:
                return navicode, "", "", stack[:-1]
            else:
                stack = stack[:-1]
                status = ""
        else:
            stack += status
            (navicode,status,corruptor,stack) = check_line(navicode,navicode[0],stack)

    return navicode, status, corruptor, stack    

def check_match(opener,candidate):
    if candidate in ["(","[","<","{"]:
        status = candidate
    else:
        if opener == "(":
            if candidate == ")":
                status = "match"
            else:
                status = "corrupt"
        elif opener == "[":
            if candidate == "]":
                status = "match"
            else:
                status = "corrupt"
        elif opener == "{":
            if candidate == "}":
                status = "match"
            else:
                status = "corrupt"
        else:
            if candidate == ">":
                status = "match"
            else:
                status = "corrupt"
    
    return status

def main():
    with open("input.txt",'r') as navi_file:
        navi_lines = navi_file.readlines()

    navigation = process_input(navi_lines)

    #star 1
    syntax_error = 0
    incompletes = []
    for navi in navigation:
        (navicode,status,corruptor,stack) = check_line(navi,navi[0],navi[0])
        if status == "corrupt":
            if corruptor == ")":
                syntax_error += 3
            elif corruptor == "]":
                syntax_error += 57
            elif corruptor == "}":
                syntax_error += 1197
            else:
                syntax_error += 25137
        else:
            incompletes.append(stack)

    print(syntax_error)     
    
    #star 2
    scores = []
    for incomplete in incompletes:
        score = 0
        for closer in incomplete[::-1]:
            score *= 5
            if closer == "(":
                score += 1
            elif closer == "[":
                score += 2
            elif closer == "{":
                score += 3
            else:
                score += 4
        scores.append(score)
        
    scores.sort()
    print(scores[(len(scores)-1)//2])
    
main()
