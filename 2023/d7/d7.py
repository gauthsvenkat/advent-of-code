import sys
from collections import Counter
from typing import Dict, List, Tuple

input: List[str] = [line.strip() for line in sys.stdin if line != "\n"]

hands_and_bids: List[Tuple[int, int]] = [
    (h, int(b)) for h, b in [l.split(" ") for l in input]
]


def score_hand(hand: str) -> int:
    # first score based on each card in the hand
    score_map: Dict[str, int] = {
        "A": "14",
        "K": "13",
        "Q": "12",
        "J": "11",
        "T": "10",
        "9": "09",
        "8": "08",
        "7": "07",
        "6": "06",
        "5": "05",
        "4": "04",
        "3": "03",
        "2": "02",
    }

    # replace each card with its score
    score = "".join([score_map[c] for c in hand])

    counts_dict = Counter(hand)

    # five of a kind
    if len(counts_dict) == 1:
        return "7" + score

    if len(counts_dict) == 2:
        # four of a kind
        if max(counts_dict.values()) == 4:
            return "6" + score
        # full house
        elif max(counts_dict.values()) == 3:
            return "5" + score

    if len(counts_dict) == 3:
        # three of a kind
        if max(counts_dict.values()) == 3:
            return "4" + score
        # two pairs
        if list(counts_dict.values()).count(2) == 2:
            return "3" + score

    # one pair
    if len(counts_dict) == 4:
        return "2" + score

    # high card
    return "1" + score


# sorted_hands_and_bids = sorted(hands_and_bids, key=lambda x: score_hand(x[0]))

p1 = sum(
    [
        bid * rank
        for (_, bid), rank in zip(
            sorted(hands_and_bids, key=lambda x: score_hand(x[0])),
            range(1, len(hands_and_bids) + 1),
        )
    ]
)

print(p1)
