# (After 16 iterations hashtable)
HashTable16 = [
    "50027",
    "61138",
    "0224",
    "1335",
    "2446",
    "3557",
    "4668",
    "507",
    "618"
]

file = open("input.txt", "r")
val_input = [int(num) for num in file.readline().split(",")]


def QuickChange(array):
    iter = []
    for val in array:
        # while len(array) > 0:
        #    val = array.pop()
        iter.append(HashTable16[int(val)])
    iter = "".join(iter)
    return iter


def ShopPreparation(max=8):
    shop = [0]*(max+1)
    while max >= 0:
        iteration = [max]
        iteration = QuickChange(iteration)
        iteration = QuickChange(iteration)
        iteration = QuickChange(iteration)
        iteration = QuickChange(iteration)
        iteration = QuickChange(iteration)
        iteration = QuickChange(iteration)
        iteration = QuickChange(iteration)
        iteration = QuickChange(iteration)
        shop[max] = iteration
        max = max - 1
    return shop


def Reflect(shop, max):
    final_hashtable = [0]*(max+1)
    while max > 0:
        total = 0
        array = shop[max]
        for val in array:
            ival = int(val)
            total = total + len(shop[ival])

        final_hashtable[max] = total
        max = max - 1
    return final_hashtable


shop = ShopPreparation(8)
final_hashtable = Reflect(shop, 5)
total = 0
for val in val_input:
    total = total + final_hashtable[val]
print(total)
