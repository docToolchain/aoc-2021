import regex as re
import math

def process_input(file_contents):
    lines_stripped = [line.strip() for line in file_contents]
    
    return lines_stripped

def explode(snail):
    for m in re.finditer("(\d+),(\d+)", snail):
        parenths = 0
        for character in snail[:m.start()]:
            if character == "[":
                parenths += 1
            elif character == "]":
                parenths -= 1
        if parenths > 4:
            cl = ""
            cr = ""
            for i, character in enumerate(snail[0:m.start()]):
                if character.isnumeric():
                    cl = character
                    il = i
                    if snail[i-1].isnumeric():
                        cl = snail[i-1]+cl
            for i, character in enumerate(snail[m.end():]):
                if character.isnumeric():
                    cr = character
                    ir = i
                    if snail[m.end()+i+1].isnumeric():
                        cr = cr + snail[m.end()+i+1]
                    break

            if bool(cr):
                snail = snail[:m.end()+ir]+str(int(cr)+int(m.group(2)))+snail[m.end()+ir+len(cr):]
            snail = snail[:m.start()-1]+"0"+snail[m.end()+1:]
            if bool(cl):
                snail = snail[:il-len(cl)+1]+str(int(cl)+int(m.group(1)))+snail[il+1:]
            
            break
             
    return snail

def split(snail):
    pattern = re.compile("\d\d")
    m = pattern.search(snail)
    if m:
        num_to_split = int(m.group(0))
        left = math.floor(int(num_to_split)/2)
        right = math.ceil(int(num_to_split)/2)
        snail = snail[:m.start()]+"["+str(left)+","+str(right)+"]"+snail[m.end():]
        
    return snail

def main():
    with open("input.txt",'r') as snail_file:
        snail_lines = snail_file.readlines()

    snails = process_input(snail_lines)
    
    # star 1
    snail = snails[0]
    for snail_temp in snails[1:]:
        snail = "["+snail+","+snail_temp+"]"
        snail_temp = ""
        while snail_temp != snail:
            snail_temp = snail
            snail = explode(snail)
            if snail == snail_temp:
                snail = split(snail)

    pattern = re.compile("\[(\d+),(\d+)\]")
    while not snail.isnumeric():
        m = pattern.search(snail)
        snail = snail[:m.start()]+str(int(m.group(1))*3+int(m.group(2))*2)+snail[m.end():]
    
    print(snail)

    # star 2
    max_mag = 0
    for snail1 in snails:
        for snail2 in snails:
            snail = "["+snail1+","+snail2+"]"
            snail_temp = ""
            while snail_temp != snail:
                snail_temp = snail
                snail = explode(snail)
                if snail == snail_temp:
                    snail = split(snail)
            pattern = re.compile("\[(\d+),(\d+)\]")
            while not snail.isnumeric():
                m = pattern.search(snail)
                snail = snail[:m.start()]+str(int(m.group(1))*3+int(m.group(2))*2)+snail[m.end():]     
            max_mag = max(max_mag,int(snail))
            
    print(max_mag)
main()
