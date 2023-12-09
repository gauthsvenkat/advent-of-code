import sys
from typing import List
from functools import reduce

histories: List[List[int]] = [
    [int(num) for num in line.strip().split(" ")] for line in sys.stdin if line != "\n"
]


def predict_next_value(history: List[int]) -> int:
    step: List[int] = history
    agg: int = 0

    while not all(num == 0 for num in step):
        agg += step[-1]
        step = [n - p for n, p in zip(step[1:], step[:-1])]

    return agg


def predict_previous_value(history: List[int]) -> int:
    step: List[int] = history
    first_vals_of_steps: List[int] = []

    while not all(num == 0 for num in step):
        first_vals_of_steps += [step[0]]
        step = [n - p for n, p in zip(step[1:], step[:-1])]
    first_vals_of_steps += [0]  # All the last 0

    return reduce(lambda x, y: y - x, reversed(first_vals_of_steps))


p1 = sum(predict_next_value(history) for history in histories)
print(p1)

p2 = sum(predict_previous_value(history) for history in histories)
print(p2)
