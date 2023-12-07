import re
from enum import Enum

from day import Day


class Hand(Enum):
    FIVE_OF_A_KIND = 6
    FOUR_OF_A_KIND = 5
    FULL_HOUSE = 4
    THREE_OF_A_KIND = 3
    TWO_PAIR = 2
    ONE_PAIR = 1
    HIGH_CARD = 0

    def __gt__(self, other):
        return self.value > other.value


class DaySeven(Day):
    day = 7

    def parse_hand(self, row):
        match = re.match(r"(.+) (\d+)", row)
        cards = match[1]
        sorted_cards = "".join(sorted(cards[:]))
        bid = int(match[2])

        if re.match(r".*(.)\1{4}", sorted_cards):
            return Hand.FIVE_OF_A_KIND, cards, bid
        elif re.match(r".*(.)\1{3}", sorted_cards):
            return Hand.FOUR_OF_A_KIND, cards, bid
        elif re.match(r"(.)\1{2}(.)\2", sorted_cards) or re.match(r"(.)\1(.)\2{2}", sorted_cards):
            return Hand.FULL_HOUSE, cards, bid
        elif re.match(r".*(.)\1{2}", sorted_cards):
            return Hand.THREE_OF_A_KIND, cards, bid
        elif re.match(r".*(.)\1.*(.)\2", sorted_cards):
            return Hand.TWO_PAIR, cards, bid
        elif re.match(r".*(.)\1", sorted_cards):
            return Hand.ONE_PAIR, cards, bid

        return Hand.HIGH_CARD, cards, bid

    def joker_hand(self, row):
        original_cards = re.match(r"(.*) .*", row)[1]
        hand, _, bid = max(self.parse_hand(row.replace("J", c)) for c in [*map(str, range(2, 10)), "T", "Q", "K", "A"])

        return hand, original_cards, bid

    def sort_hands(self, card_values, hand, cards, _):
        return Hand(hand), [card_values.index(c) for c in cards]

    def one(self, input):
        winnings = 0
        card_values = [*map(str, range(2, 10)), "T", "J", "Q", "K", "A"]

        for rank, (hand, cards, bid) in enumerate(
            sorted((self.parse_hand(row) for row in input), key=lambda row: self.sort_hands(card_values, *row)), start=1
        ):
            winnings += rank * bid

        return winnings

    def two(self, input):
        winnings = 0
        card_values = ["J", *map(str, range(2, 10)), "T", "Q", "K", "A"]

        for rank, (hand, cards, bid) in enumerate(
            sorted((self.joker_hand(row) for row in input), key=lambda row: self.sort_hands(card_values, *row)), start=1
        ):
            winnings += rank * bid

        return winnings


if __name__ == "__main__":
    DaySeven().run()
