import math
import re
import sys

# (row, col) : char
map = {}

# expression for finding numbers of any length
number_expr = re.compile(r"\d+")
# expression for finding symbols
symbol_expr = re.compile(r"[^a-zA-Z0-9.]")

for row, line in enumerate([line.strip() for line in sys.stdin]):
    if number_spans := [m.span() for m in number_expr.finditer(line)]:
        for col_s, col_e in number_spans:
            map[(row, col_s)] = line[col_s:col_e]

    if symbol_starts := [m.start() for m in symbol_expr.finditer(line)]:
        for col in symbol_starts:
            map[(row, col)] = line[col]

# def get_numbers_near_coordinate(coords_and_nums, coord):
#     for (x,y), num in coords_and_nums:
#         for


def distance(a, b):
    return max(abs(a[0] - b[0]), abs(a[1] - b[1]))


def find_numbers_near(coord, coords_and_nums):
    x, y = coord
    numbers_near_symbol = []
    for (nx, ny), num in coords_and_nums:
        if min([distance((x, y), (nx, ny + i)) for i in range((len(num)))]) == 1:
            numbers_near_symbol.append(int(num))

    return numbers_near_symbol


p1 = 0
p2 = 0
# Loop over all the symbols
symbol_iter = [_ for _ in map.items() if not _[1].isdigit()]
numbers_iter = [_ for _ in map.items() if _[1].isdigit()]

for (sx, sy), char in symbol_iter:
    numbers_near_symbol = find_numbers_near((sx, sy), numbers_iter)
    p1 += sum(numbers_near_symbol)

    if char == "*" and len(numbers_near_symbol) == 2:
        p2 += math.prod(numbers_near_symbol)


print(p1)
print(p2)
