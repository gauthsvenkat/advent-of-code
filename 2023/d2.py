import sys

for line in sys.stdin:
    if line == "\n":
        continue

    game_id, record = line.strip().split(": ")

    # Convert game_id to int
    game_id = int(game_id.replace("Game ", ""))

    sets = [s.strip().split(", ") for s in record.strip().split("; ")]

