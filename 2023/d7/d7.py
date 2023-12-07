import sys
from collections import Counter
from typing import List, Tuple

input: List[str] = [line.strip() for line in sys.stdin if line != "\n"]
hands_and_bids: List[Tuple[str, int]] = [
    (h, int(b)) for h, b in [line.split(" ") for line in input]
]


def score_card(card: str) -> str:
    match card:
        case "A":
            return "14"
        case "K":
            return "13"
        case "Q":
            return "12"
        case "J":
            return "01"  # changed to 01 for part 2
        case "T":
            return "10"
        case "9":
            return "09"
        case "8":
            return "08"
        case "7":
            return "07"
        case "6":
            return "06"
        case "5":
            return "05"
        case "4":
            return "04"
        case "3":
            return "03"
        case "2":
            return "02"
    return ""


def promote_hand(hand: str) -> str:
    # do nothing if there is no joker
    if "J" not in hand:
        return hand

    # J is in the hand. So check which promotion has the highest value
    promotion_candidate = max(hand, key=lambda x: score_hand(hand.replace("J", x)))

    return hand.replace("J", promotion_candidate)


def score_hand(hand: str) -> str:
    card_counts = Counter(hand)

    match len(card_counts):
        case 1:
            return "7"  # five of a kind
        case 2:
            match max(card_counts.values()):
                case 4:
                    return "6"  # four of a kind
                case 3:
                    return "5"  # full house
        case 3:
            match max(card_counts.values()):
                case 3:
                    return "4"  # three of a kind
                case 2:
                    return "3"  # two pairs
        case 4:
            return "2"  # one pair
        case 5:
            return "1"  # high card


def score(hand):
    card_scores = "".join(score_card(c) for c in hand)
    hand = promote_hand(hand)
    hand_score = score_hand(hand)
    return hand_score + card_scores


ans = sum(
    [
        bid * rank
        for (_, bid), rank in zip(
            sorted(hands_and_bids, key=lambda x: score(x[0])),
            range(1, len(hands_and_bids) + 1),
        )
    ]
)

print(ans)
