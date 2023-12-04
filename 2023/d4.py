import sys

agg = 0

for line in [line for line in sys.stdin if line != "\n"]:
    line = line.strip()

    card_number, numbers = line.split(": ")
    card_number = int(card_number.replace("Card ", ""))

    winning_numbers, our_numbers = numbers.split(" | ")
    winning_numbers = {int(n) for n in winning_numbers.split(" ") if n != ""}
    our_numbers = {int(n) for n in our_numbers.split(" ") if n != ""}

    common_numbers = winning_numbers & our_numbers

    num_common = len(common_numbers)

    agg += int(2 ** (num_common-1)) if num_common else 0

print(agg)
