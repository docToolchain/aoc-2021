# see description.adoc

from pprint import pprint


def read_daily_input(filename):
    ''' Read lines from file with given input name
        cast to daily required type and return list '''
    with open(filename) as input_file:
        local_list = input_file.readlines()
        return_list = [item.strip() for item in local_list]
        return return_list


def transpose_input(daily_list):
    ''' Transpose list for analysis
    '''
    item_length = len(daily_list[0])
    pos_list = [''] * item_length;
    for item in daily_list:
        for i in range(item_length):
            pos_list[i] = pos_list[i] + item[i]
    return pos_list


def get_most_common_value(transposed_list):
    ''' Return binary number strings with most common value (0 or 1)
        per transposed list index
    '''
    most_common = ""
    number_length = len(transposed_list)
    pos_count = [0] * number_length
    max_count = len(transposed_list[0])
    for i in range(number_length):
        pos_count[i] = transposed_list[i].count('1')
        most_common = most_common + ('1' if (pos_count[i] >= (max_count - pos_count[i])) else '0')
    return most_common


def main():
    daily_list = read_daily_input('input03.txt')
    power_list = transpose_input(daily_list)
    gamma = get_most_common_value(power_list)
    epsilon = ''.join(['1' if i == '0' else '0'
                     for i in gamma])
    print(gamma, epsilon)
    star1 = int(gamma, base=2) * int(epsilon, base=2)
    print(f"Result (*): {star1}")

    co2_list = daily_list.copy()
    i = 0
    while ((len(co2_list) > 1) & (i < len(co2_list[0]))):
        co2_transposed_list = transpose_input(co2_list)
        gamma_co2 = get_most_common_value(co2_transposed_list)
        co2_list = list(filter(lambda co2_value: co2_value[i] == gamma_co2[i], co2_list))
        i += 1

    oxy_list = daily_list.copy()
    i = 0
    while ((len(oxy_list) > 1) & (i < len(oxy_list[0]))):
        oxy_transposed_list = transpose_input(oxy_list)
        epsilon_oxy = ''.join(['1' if i == '0' else '0'
                            for i in get_most_common_value(oxy_transposed_list)])
        oxy_list = list(filter(lambda oxy_value: oxy_value[i] == epsilon_oxy[i], oxy_list))
        i += 1

    print(co2_list, oxy_list)
    star2 = int(co2_list[0], base=2) * int(oxy_list[0], base=2)
    print(f"Result (**): {star2}")



if __name__ == "__main__":
    main()
