import sys

map = {}
seed_values = []

while line := sys.stdin.readline():
    if "seeds:" in line:
        seed_values = [int(num) for num in line.split(" ") if num.isdigit()]

    elif "map:" in line:
        map_type = line.strip().split(" ")[0]
        map[map_type] = {}

        while (line := sys.stdin.readline()) != "\n":
            destination_range_start, source_range_start, length = [
                int(num) for num in line.strip().split(" ")
            ]

            source_range = range(source_range_start, source_range_start + length)
            destination_range = range(
                destination_range_start, destination_range_start + length
            )

            for k, v in zip(source_range, destination_range):
                map[map_type][k] = v

# For each seed value, run through the map and find the location
