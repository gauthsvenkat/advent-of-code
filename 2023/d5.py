import sys
from typing import Dict, List, Tuple
from multiprocessing import Pool

input: List[str] = [line.strip() for line in sys.stdin]
# parse the line containing the seeds
seed_input: List[int] = [int(num) for num in input[0].split(" ") if num.isdigit()]

# get the ranges and sort them by length
seed_ranges: List[Tuple[int, int]] = sorted(
    zip(seed_input[::2], seed_input[1::2]),
    key=lambda x: x[1],
)

# seeds: List[int] = [int(num) for num in input[0].split(" ") if num.isdigit()]

map: Dict[str, List[Tuple[int, int, int]]] = {}
i = 0
while i < len(input):
    if "map:" in input[i]:
        map_type = input[i].split(" ")[0]
        map[map_type] = []
        i += 1
        while input[i] != "":
            destination_range_start, source_range_start, length = [
                int(num) for num in input[i].split(" ")
            ]
            map[map_type].append((destination_range_start, source_range_start, length))
            i += 1
    i += 1


def map_value(value: int, mappings: List[Tuple[int, int, int]]) -> int:
    for destination_range_start, source_range_start, length in mappings:
        if source_range_start <= value < source_range_start + length:
            return destination_range_start + value - source_range_start
    return value


def seed_to_location(seed: int, map: Dict[str, List[Tuple[int, int, int]]]) -> int:
    for _, mappings in map.items():
        seed = map_value(seed, mappings)
    return seed


def find_min_location_in_range(
    seed_range: Tuple[int, int], map: Dict[str, List[Tuple[int, int, int]]]
) -> int:
    return min(
        seed_to_location(seed, map)
        for seed in range(seed_range[0], seed_range[0] + seed_range[1])
    )


with Pool(10) as p:
    print(min(p.starmap(find_min_location_in_range, [(x, map) for x in seed_ranges])))
