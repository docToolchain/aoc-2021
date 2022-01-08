def process_input(file_contents):
    lines_stripped = [line.strip() for line in file_contents]
    
    octos = [int(i) for lines in lines_stripped for i in lines]
    
    return octos

def iterate_octos(flashes,octos):
    flashed = set()
    flashed_temp = flashed.copy()
    octos = [octo + 1 for octo in octos]
    if any([octo == 10 for octo in octos]):
        flash_locations = [i for i, e in enumerate(octos) if e >9]
        flashed_temp.update(flash_locations)
    while flashed != flashed_temp:
        for octo in flashed_temp-flashed:
            if octo > 9:
                octos[octo-10] += 1
                if octo % 10 != 0:
                    octos[octo-11] += 1
                if (octo+1) % 10 != 0:
                    octos[octo-9] += 1
            if octo < 90:
                octos[octo+10] += 1
                if octo % 10 != 0:
                    octos[octo+9] += 1
                if (octo+1) % 10 != 0:
                    octos[octo+11] += 1
            if octo % 10 != 0:
                octos[octo-1] +=1
            if (octo+1) % 10 != 0:
                    octos[octo+1] += 1
        flashed = flashed_temp.copy()
        if any([octo > 9 for octo in octos]):
            flash_locations = [i for i, e in enumerate(octos) if e >9]
            flashed_temp.update(flash_locations)
        
    flashes += sum([1 if octo>9 else 0 for octo in octos])
    octos = [octo if octo <10 else 0 for octo in octos]
    
    return flashes,octos

def main():
    with open("input.txt",'r') as octo_file:
        octo_lines = octo_file.readlines()

    octos = process_input(octo_lines)
    octos_temp = octos.copy()
    flashes = 0
    
    #star 1
    for i in range(100):
        flashes,octos = iterate_octos(flashes,octos)

    print(flashes)
    
    #star 2
    count = 0
    octos = octos_temp
    while octos != [0]*100:
        flashes,octos = iterate_octos(flashes,octos)
        count += 1
        
    print(count)

main()
