import sys

marble_count = {
    "red": 12,
    "green": 13,
    "blue": 14,
}

agg = 0

for game_id, record in [line.strip().split(": ") for line in sys.stdin if line != "\n"]:
    game_id = int(game_id.replace("Game ", ""))

    for round in record.strip().split("; "):
        for num_marble in round.split(", "):
            num, marble_color = num_marble.split(" ")
            if int(num) > marble_count[marble_color]:
                # game is invalid
                break
        else:
            continue
        # This is only be executed if the round is invalid
        break
    else:
        # This is only executed if all rounds are valid
        agg += game_id

print(agg)
