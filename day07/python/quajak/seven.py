import numpy as np

f = open("seven.txt", "r")
lines = [x.strip() for x in f.readlines()]
nums = np.array([int(n) for n in lines[0].split(",")])

cur =  np.median(nums)
print("Star 1", np.sum(np.abs(nums - cur)))


min = 10000000000000000
for i in range(100, 500):
    dis = np.abs((nums - i))
    cost =dis * (dis + 1) / 2
    cost = np.sum(np.abs(cost))
    if cost < min:
        min = cost
        
print("Star 2", min)