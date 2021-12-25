from typing import Tuple

f = open("sixteen.txt", "r")
line = [x.strip() for x in f.readlines()][0]
b = bin(int(line, 16))[2:]

if int(line[0], 16) < 4:
    b = "00" + b
elif int(line[0], 16) < 8:
    b = "0" + b

def read_packet(data: str) -> Tuple[int, int, int]:
    version = data[:3]
    version = int(version, 2)
    t = data[3:6]
    t = int(t, 2)
    pos = 6
    if t == 4:
        len = 0
        last = False
        value = ""
        while not last:
            last = data[pos] == "0"
            value += data[pos+1:pos+5]
            pos += 5
            len += 1
        value = int(value, 2)
        return (value, version, pos)
    else:
        length = 11 if data[pos] == "1" else 15
        vsum = version
        pos += 1
        values = []
        if length == 15:
            bits_subdata = int(data[pos:pos+length], 2)
            pos += length
            goal = pos + bits_subdata
            while pos != goal:
                val, v, l = read_packet(data[pos:])
                vsum += v
                pos += l
                values.append(val)
        else:
            bits_subdata = int(data[pos:pos+length], 2)
            pos += length
            for i in range(bits_subdata):
                val, v, l = read_packet(data[pos:])
                vsum += v
                pos += l
                values.append(val)
        if t == 0:
            value = sum(values)
        elif t == 1:
            value = 1
            for val in values:
                value *= val
        elif t == 2:
            value = min(values)
        elif t == 3:
            value = max(values)
        elif t == 5:
            value = 1 if values[0] > values[1] else 0
        elif t == 6:
            value = 1 if values[0] < values[1] else 0
        elif t == 7:
            value = 1 if values[0] == values[1] else 0 
        return [value, vsum, pos]
    
print(read_packet(b))