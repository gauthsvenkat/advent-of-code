import sys
from itertools import combinations
from typing import List, Tuple

lines: List[str] = [line.strip() for line in sys.stdin if line != "\n"]

num_rows: int = len(lines)
num_cols: int = len(lines[0])

galaxy_coords: List[Tuple[int, int]] = [
    (row, col)
    for col, line in enumerate(lines)
    for row, line in enumerate(lines)
    if line[col] == "#"
]

# Find out which rows and cols have no galaxy
rows_wo_galaxy: List[int] = [
    row
    for row in range(num_rows)
    if row not in [row for row, _ in galaxy_coords]  # noqa E501
]
cols_wo_galaxy: List[int] = [
    col
    for col in range(num_cols)
    if col not in [col for _, col in galaxy_coords]  # noqa E501
]

# Adjust the galaxy coordinates such that empty rows or cols
# are considered twice as big
# remove 999999 for part 1
for i, (row, col) in enumerate(galaxy_coords):
    if offset := sum(row > r for r in rows_wo_galaxy):
        galaxy_coords[i] = (galaxy_coords[i][0] + offset * 999999, galaxy_coords[i][1])
    if offset := sum(col > c for c in cols_wo_galaxy):
        galaxy_coords[i] = (galaxy_coords[i][0], galaxy_coords[i][1] + offset * 999999)


# For all pairs in galaxy_coords, find the manhattan distance.
def distance(a: Tuple[int, int], b: Tuple[int, int]) -> int:
    return abs(a[0] - b[0]) + abs(a[1] - b[1])


distances = [distance(a, b) for a, b in combinations(galaxy_coords, 2)]

print(sum(distances))
