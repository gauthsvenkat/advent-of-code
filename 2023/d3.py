import sys
import re

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

def get_numbers_near_coordinate(coords_and_nums, coord):
    for (x,y), num in coords_and_nums:
        for 

# Loop over all the symbols
for (x,y), char in [_ for _ in map.items() if not _[1].isdigit()]:
    # Find all the numbers around the symbol
    numbers = [num for (x,y), num in [_ for _ in map.items() if _[1].isdigit()] if ]
