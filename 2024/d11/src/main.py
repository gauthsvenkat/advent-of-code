import sys
from functools import cache


def split_stone_if_even(num: int) -> tuple[int, int] | None:
    """Split a number into two parts if it has an even number of digits."""
    n_digits = len(str(num))

    if n_digits % 2 != 0:
        return None

    return int(str(num)[: n_digits // 2]), int(str(num)[n_digits // 2 :])


@cache
def count(stone: int, blink: int) -> int:
    if blink == 0:
        return 1

    if stone == 0:
        return count(1, blink - 1)
    elif (split := split_stone_if_even(stone)) is not None:
        return count(split[0], blink - 1) + count(split[1], blink - 1)
    else:
        return count(stone * 2024, blink - 1)


"""Usage: python main.py input.txt"""
with open(sys.argv[1], "r") as file:
    stones = [int(n) for n in file.read().split()]

print("P1:", sum(count(stone, 25) for stone in stones))
print("P2:", sum(count(stone, 75) for stone in stones))
