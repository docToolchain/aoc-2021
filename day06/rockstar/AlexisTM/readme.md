# Rockstar

There are multiple implementations, increasing speed for the very slow rockstar

## Naive implementation: naive.rock

The naive implementation is writting down the problem as it is exposed, with a list simulating all states at once, following the rules.

This provides the result in 2 minutes for the first step so few years for the second, with an horrible memory complexity (Playing with an array of 360k elements, dynamically pushing to it)

## Slightly optimized: precompute.rock

The slightly optimized solution is based out of the following two observations:
- All inputs are in the `[1-5]` range
- The result equals to the sum of the individual inputs

Therefore, we can precompute an array for each possible input, then simply sum the precomputed result, which executes in one second.

## Hashtable optimization: rock.py

While the naive implementation just works in Python, even the basic optimization is not enough for rockstar.
Starting from the slightly optimized, we are using those observations:
- All inputs are in the `[1-5]` range
- The result equals to the sum of the individual inputs
- The individual steps can be fast forwarded as a number will always give the same result N iterations later
- We can reach half of the problem then simply double the number of iteration without having the full memory complexity

The steps are:
- Make an hashtable for 16 iterations: "For each number, what would be the fish state after 16 iterations"
- Apply the hashtable 8 times over the input => Equivalent to 128 iterations
- Double the iterations (Reflect) by using the result at 128 iterations and computing the final size it would have, without doing the iterations

Additional optimization possible: We don't have to compute the hashtable for each iterations, we could simply compute it for the value 5, then value 4 is 5 + one iteration etc.

## Best method

The best method is to express mathematically the problem so you can precompute the contribution of each input without doing any iterations.

## Hashtable 16

| Input | Day for input 0 | Result after 16 days for input |
|-------|-----------------|--------------------------------|
|    | 0  | 0  |
|    | 1  | 68 |
|    | 2  | 57 |
|    | 3  | 46 |
|    | 4  | 35 |
|    | 5  | 24 |
|    | 6  | 13 |
|    | 7  | 02 |
| 8  | 8  | 618 |
| 7  | 9  | 507 |
| 6  | 10 | 4668 |
| 5  | 11 | 3557 |
| 4  | 12 | 2446 |
| 3  | 13 | 1335 |
| 2  | 14 | 0224 |
| 1  | 15 | 61138 |
| 0  | 16 | 50027 |
