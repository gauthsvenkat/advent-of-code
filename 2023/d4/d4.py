import sys

input = [line.strip() for line in sys.stdin if line != "\n"]


def parse(line):
    card_number, numbers = line.split(": ")
    card_number = int(card_number.replace("Card ", ""))

    winning_numbers, our_numbers = numbers.split(" | ")
    winning_numbers = {int(n) for n in winning_numbers.split(" ") if n != ""}
    our_numbers = {int(n) for n in our_numbers.split(" ") if n != ""}

    return card_number, (winning_numbers, our_numbers)


agg = 0
scratchcard_count = {i + 1: 1 for i in range(len(input))}

for line in input:
    card_number, (winning_numbers, our_numbers) = parse(line)
    common_numbers = winning_numbers & our_numbers
    num_common = len(common_numbers)
    agg += int(2 ** (num_common - 1)) if num_common else 0

    for i in range(num_common):
        scratchcard_count[card_number + i + 1] += scratchcard_count[card_number]

print(agg)
print(sum(scratchcard_count.values()))
