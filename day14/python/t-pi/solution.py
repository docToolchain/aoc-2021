# see README.adoc

import math
from pprint import pprint

def read_daily_input(filename):
    ''' Read lines from file with given input name
        cast to daily required type and return list
    '''
    with open(filename) as input_file:
        local_list = [item.strip() for item in input_file.readlines()]
    initial_polymer = local_list[0]
    reactions = {item.split('->')[0].strip():item.split('->')[1].strip() for item in local_list[2:]}
    return initial_polymer, reactions


def single_step(polymer, reactions):
    ''' Iterate one polymerization step by inserting monomers
        Return new polymer
    '''
    new_polymer = ''
    for i, c in enumerate(polymer[:-1]):
        new_polymer += polymer[i]
        interface = polymer[i:i+2]
        if (interface in reactions.keys()):
            new_polymer += reactions[interface]
    new_polymer += polymer[-1]    
    return(new_polymer)


def single_step_count(paircount, reactions):
    ''' Iterate one polymerization step by counting and adapting the pairs
        Return new dict of pair count
    '''
    for key, n in paircount.copy().items():
        paircount[key] -= n
        middle = reactions[key]
        left, right = key
        paircount[left + middle] = n if (left + middle not in paircount.keys()) else paircount[left + middle]+n
        paircount[middle + right] = n if (middle + right not in paircount.keys()) else paircount[middle+right]+n
    return paircount


def count_pairs(initial_polymer, chars, polypairs):
    ''' Count pairs as if in polymer
        Return max count - min count
    '''
    char_count = dict()
    for c in chars:
        char_count[c] = 0
        for k in polypairs.keys():
            if (c == k[0]):
                char_count[c] += polypairs[k]
    char_count[initial_polymer[-1]] += 1
    return max(char_count.values()) - min(char_count.values())


def main():
    polymer, reactions = read_daily_input('input14.txt') 
    new_polymer = polymer   
    for __ in range(10):
        new_polymer = single_step(new_polymer, reactions)
    chars = set([c for c in new_polymer])
    char_count = dict()
    for c in chars:
        char_count[c] = new_polymer.count(c)
    star1 = max(char_count.values()) - min(char_count.values())
    print(f"Result (*): {star1}")

    polypairs = dict()
    for i in range(len(polymer)-1):
        polypairs[polymer[i:i+2]] = 1 if (polymer[i:i+2] not in polypairs.keys()) else polypairs[polymer[i:i+2]] + 1
    for __ in range(10):
        polypairs = single_step_count(polypairs, reactions)
    star1 = count_pairs(polymer, chars, polypairs)
    print(f"Result (*) counting: {star1}")
    for __ in range(30):
        polypairs = single_step_count(polypairs, reactions)
    star2 = count_pairs(polymer, chars, polypairs)
    print(f"Result (**): {star2}")


if __name__ == "__main__":
    main()
