def hex_to_binary(hex_num, num_digits):

    return str(bin(int(hex_num, 16)))[2:].zfill(num_digits)
        
def process_input(file_contents):
    lines_stripped = [line.strip() for line in file_contents]

    return hex_to_binary(lines_stripped[0],len(lines_stripped[0])*4)

def do_type_code(type_code,value,v,flag):
    if type_code == 0:
        value += v
    elif type_code == 1:
        if flag == 0:
            value = v
            flag = 1
        else:
            value *= v
    elif type_code == 2:
        if flag == 0:
            value = v
            flag = 1
        else:
            value = min(value,v)
    elif type_code == 3:
        if flag == 0:
            value = v
            flag = 1
        else:
            value = max(value,v)
    elif type_code == 5:
        if flag == 0:
            value = v
            flag = 1
        elif value > v:
            value = 1
        else:
            value = 0
    elif type_code == 6:
        if flag == 0:
            value = v
            flag = 1
        elif value < v:
            value = 1
        else:
            value = 0
    elif type_code == 7:
        if flag == 0:
            value = v
            flag = 1
        elif value == v:
            value = 1
        else:
            value = 0

    return value,flag

def decode_transmission(code,versions):
    value = 0
    versions.append(int(code[0:3],2))
    type_code = int(code[3:6],2)
    length = 6

    if type_code == 4:
        bits = ""
        while int(code[length])!=0:
            bits += code[length+1:length+5]
            length += 5
        bits += code[length+1:length+5]
        length += 5
        
        value = int(bits,2)
    else:            
        flag = 0
        if int(code[length])==0:
            length += 1
            subpacket_length = int(code[length:length+15],2)
            length += 15
            length_temp = length + subpacket_length
            
            while length < length_temp:
                (x,v) = decode_transmission(code[length:],versions)
                length += x
                value,flag = do_type_code(type_code,value,v,flag)  
                        
        else:
            length += 1
            no_subpackets = int(code[length:length+11],2)
            length += 11
            for i in range(no_subpackets):
                (x,v) = decode_transmission(code[length:], versions)
                length += x
                value,flag = do_type_code(type_code,value,v,flag)

    return length, value

def main():
    with open("input.txt",'r') as hex_file:
        hex_lines = hex_file.readlines()

    bin_num = process_input(hex_lines)
    
    versions = list()
    length, value = decode_transmission(bin_num,versions)
    print(sum(versions))
    
    print(value)

main()
