# see description.adoc

from pprint import pprint


def read_daily_input(filename):
    ''' Read lines from file with given input name
        cast to daily required type and return list '''
    with open(filename) as input_file:
        local_list = input_file.readlines()
        return_list = [item.strip() for item in local_list]
        return return_list


def preprocess_input(daily_list):
    ''' not needed today
    '''
    pass



def main():
    daily_list = read_daily_input('input02.txt')
    depth = 0
    position = 0
    for item in daily_list:
        command, parameter = item.split()
        parameter_input = int(parameter)
        if (command == 'down'):
            depth += parameter_input
        elif (command == 'up'):
            depth -= parameter_input
        elif (command == 'forward'):
            position += parameter_input
        if (depth < 0):
            print('jump!')
    star1 = depth * position
    print(f"Result (*): {star1}")

    depth = 0
    position = 0
    aim = 0
    for item in daily_list:
        command, parameter = item.split()
        parameter_input = int(parameter)
        if (command == 'down'):
            aim += parameter_input
        elif (command == 'up'):
            aim -= parameter_input
        elif (command == 'forward'):
            position += parameter_input
            depth += aim*parameter_input
        if (depth < 0):
            print('jump, jump!')
    star2 = depth * position
    print(f"Result (**): {star2}")



if __name__ == "__main__":
    main()
