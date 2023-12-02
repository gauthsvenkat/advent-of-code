import sys

agg = 0

for game_id, record in [line.strip().split(": ") for line in sys.stdin if line != "\n"]:
    game_id = int(game_id.replace("Game ", ""))

    max_num_marbles = {"red": 0, "green": 0, "blue": 0}
    for round in record.strip().split("; "):
        # For each marble color
        for num_marble in round.split(", "):
            num, marble_color = num_marble.split(" ")
            if int(num) > max_num_marbles[marble_color]:
                max_num_marbles[marble_color] = int(num)

    agg += max_num_marbles["red"] * max_num_marbles["green"] * max_num_marbles["blue"]


print(agg)
