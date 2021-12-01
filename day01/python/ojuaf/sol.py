def load_input():
    data = list()
    with open('input') as fd:
        for line in fd:
            value = line.strip()
            data.append(int(value))
    return data


def main():
    data = load_input()
    # First task
    count = 0
    for i in range(len(data)-1):
        if data[i+1] - data[i] > 0:
            count += 1
    result = count
    print("Result 1: ", result)

    # Second Task
    count = 0
    for i in range(len(data)-3):
        if data[i+3] - data[i] > 0:
            count += 1
    result = count
    print("Result 1: ", result)
            
    return 0

if __name__ == '__main__':
    main()
