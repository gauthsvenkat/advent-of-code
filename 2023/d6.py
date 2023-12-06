import operator
import sys
from functools import reduce

race_times = [
    int(num) for num in sys.stdin.readline().strip().split(" ") if num.isdigit()
]
best_distances = [
    int(num) for num in sys.stdin.readline().strip().split(" ") if num.isdigit()
]


def find_num_solns(time, distance):
    r = range(1, time)
    c = lambda x: x * (time - x) > distance

    for s in r:
        if c(s):
            break

    for e in reversed(r):
        if c(e):
            break

    return e - s + 1


p1 = reduce(
    operator.mul, (find_num_solns(t, d) for t, d in zip(race_times, best_distances))
)

p2 = find_num_solns(
    int("".join(map(str, race_times))), int("".join(map(str, best_distances)))
)

print(p1)
print(p2)
