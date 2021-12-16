# see README.adoc

import math
from pprint import pprint

def read_daily_input(filename):
    ''' Read lines from file with given input name
        cast to daily required type and return list
    '''
    with open(filename) as input_file:
        local_list = [item.strip() for item in input_file.readlines()]
    # local_list[0] = 'D2FE28'
    # local_list[0] = '8A004A801A8002F478'
    bin_list = [n for c in local_list[0] for n in format(int(c, 16), '04b')]
    return bin_list


def parse_input(main_stream, packets = []):
    ''' Parse packet as list of bit-strings
        Return list of packets
    '''
   
    def read_chunk(stream, n):
        ''' Reads number n of bits of stream
            Returns value of bits and returns rest of stream
        '''
        return int(''.join(stream[:n]), 2), stream[n:]

    def read_literal(stream):
        ''' Reads and splits sub-packets of 5 bit
            Returns list of data numbers and rest of stream
        '''
        last = False
        data = []
        while not last:
            data_packet = stream[:5]
            if (data_packet[0] == '0'):
                last = True
            data.extend(data_packet[1:])
            stream = stream[5:]
        return int(''.join(data), 2), stream
    
    def read_single_packet(stream):
        ''' Reads single packet
            Return list of dict(s) and rest of stream
        '''
        new_packet = {'ver': None, 'type': None, 'op_l': None, 'data': None}
        # print(stream)
        ver, stream = read_chunk(stream, 3)
        new_packet['ver'] = ver
        type, stream = read_chunk(stream, 3)
        new_packet['type'] = type
        # print(ver, type)
        if (type == 4):
            data, stream = read_literal(stream)
            new_packet['data'] = data
            # print("new literal packet\n", new_packet)
            return [new_packet], stream
        else:
            op_l, stream = read_chunk(stream, 1)
            new_packet['op_l'] = op_l
            # print(op_l)
            if (op_l == 0):
                bit_length, stream = read_chunk(stream, 15)
                sub_stream = stream[:bit_length]
                new_0packets = []
                while ((len(sub_stream) > 10) and (sum([int(c) for c in sub_stream]) > 0)):
                    bit_packets, sub_stream = read_single_packet(sub_stream)
                    new_0packets.extend(bit_packets)
                return ([new_packet] + new_0packets), stream[bit_length:]
            else:
                packet_count, stream = read_chunk(stream, 11)
                new_1packets = []
                # print("sub_packetcount:", packet_count)
                while len(new_1packets) < packet_count:
                    counted_packets, stream = read_single_packet(stream)
                    new_1packets.extend(counted_packets)
                    # print("new sub packets\n", new_packets)
                # print("new 1 packet\n", new_packets)
                return ([new_packet] + new_1packets), stream
    
    # print("main stream: \n", main_stream)
    while ((len(main_stream) > 10) and (sum([int(c) for c in main_stream]) > 0)):
        new_packets, main_stream = read_single_packet(main_stream)
        packets.extend(new_packets)
        # print("packets \n", packets)
    return packets


def interpret_input(main_stream, packets = []):
    ''' Parse packet as list of bit-strings and interpret op packets
        Return score
    '''
   
    def read_chunk(stream, n):
        ''' Reads number n of bits of stream
            Returns value of bits and returns rest of stream
        '''
        return int(''.join(stream[:n]), 2), stream[n:]

    def read_literal(stream):
        ''' Reads and splits sub-packets of 5 bit
            Returns list of data numbers and rest of stream
        '''
        last = False
        data = []
        while not last:
            data_packet = stream[:5]
            if (data_packet[0] == '0'):
                last = True
            data.extend(data_packet[1:])
            stream = stream[5:]
        return int(''.join(data), 2), stream
    
    def do_the_op(op, packets):
        ''' Carry out the operation from op code with data packets
            Return result value
        '''
        if op == 0: # sum
            return sum([p['data'] for p in packets])
        if op == 1: # product
            prod = 1
            for p in packets:
                prod = prod * p['data'] if p['data'] != None else prod
            return prod
        if op == 2: # min
            return min([p['data'] for p in packets])
        if op == 3: # max
            return max([p['data'] for p in packets])
        if op == 5: # p1 > p2 ? 1 : 0
            return 1 if packets[0]['data'] > packets[1]['data'] else 0
        if op == 6: # p1 < p2 ? 1 : 0
            return 1 if packets[0]['data'] < packets[1]['data'] else 0
        if op == 7: # p1 = p2 ? 1 : 0
            return 1 if packets[0]['data'] == packets[1]['data'] else 0

    def read_single_packet(stream):
        ''' Reads single packet
            Return list of dict(s) and rest of stream
        '''
        new_packet = {'ver': None, 'type': None, 'op_l': None, 'data': None}
        # print(stream)
        ver, stream = read_chunk(stream, 3)
        new_packet['ver'] = ver
        type, stream = read_chunk(stream, 3)
        new_packet['type'] = type
        # print(ver, type)
        if (type == 4):
            data, stream = read_literal(stream)
            new_packet['data'] = data
            # print("new literal packet\n", new_packet)
            return [new_packet], stream
        else:
            op_l, stream = read_chunk(stream, 1)
            new_packet['op_l'] = op_l
            # print(op_l)
            if (op_l == 0):
                bit_length, stream = read_chunk(stream, 15)
                sub_stream = stream[:bit_length]
                new_0packets = []
                while ((len(sub_stream) > 10) and (sum([int(c) for c in sub_stream]) > 0)):
                    bit_packet, sub_stream = read_single_packet(sub_stream)
                    new_0packets.extend(bit_packet)
                result = do_the_op(type, new_0packets)
                new_packet['data'] = result
                return [new_packet], stream[bit_length:]
            else:
                packet_count, stream = read_chunk(stream, 11)
                new_1packets = []
                # print("sub_packetcount:", packet_count)
                while len(new_1packets) < packet_count:
                    counted_packet, stream = read_single_packet(stream)
                    new_1packets.extend(counted_packet)
                    # print("new sub packets\n", new_packets)
                # print("new 1 packet\n", new_packets)
                result = do_the_op(type, new_1packets)
                new_packet['data'] = result
                return [new_packet], stream
    
    # print("main stream: \n", main_stream)
    while ((len(main_stream) > 10) and (sum([int(c) for c in main_stream]) > 0)):
        new_packet, main_stream = read_single_packet(main_stream)
        packets.extend(new_packet)
        # print("packets \n", packets)
    return packets


def main():
    main_packet = read_daily_input('input16.txt')
    # print(''.join(main_packet))
    packets = parse_input(main_packet.copy())
    # pprint(packets)
    star1 = sum([p['ver'] for p in packets])
    print(f"Result (*): {star1}")
    star2 = interpret_input(main_packet)[0]['data']
    print(f"Result (**): {star2}")


if __name__ == "__main__":
    main()
