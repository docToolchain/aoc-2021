# see description.adoc

from pprint import pprint


def read_daily_input(filename):
    ''' Read lines from file with given input name
        cast to daily required type and return list '''
    with open(filename) as input_file:
        local_list = input_file.readlines()
        return_list = [int(item.strip()) for item in local_list]
        return return_list


def preprocess_input(daily_list):
    ''' not needed today
    '''
    pass



def main():
    daily_list = read_daily_input('input01.txt')
    sum1 = 0
    previous_item = -1
    for item in daily_list:
        if (previous_item >= 0): 
            if (previous_item < item):
                sum1 += 1
        previous_item = item
        
    star1 = sum1
    print(f"Result (*): {star1}")

    sum2 = 0
    previous_window = -1
    for i in range(len(daily_list)-2):
        current_window = sum(daily_list[i:i+3])
        if (previous_window >= 0):
            if (previous_window < current_window):
                sum2 += 1
        previous_window = current_window
        
    star2 = sum2
    print(f"Result (**): {star2}")



if __name__ == "__main__":
    main()
