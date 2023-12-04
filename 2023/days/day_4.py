import re
from collections import defaultdict

from day import Day


class DayFour(Day):
    day = 4

    def get_winners(self, row: str):
        match = re.match(r"Card\s+\d+: (.*) \| (.*)", row)
        winners, numbers = (re.split(r"\s+", match[1].strip()), re.split(r"\s+", match[2].strip()))

        return set(numbers).intersection(set(winners))

    def one(self, input):
        return sum(int(2 ** (len(self.get_winners(row)) - 1)) for row in input)

    def two(self, input):
        instances = defaultdict(lambda: 1)

        for n, row in enumerate(input, start=1):
            instances[n]
            winners = self.get_winners(row)

            for m in range(len(winners)):
                instances[n + m + 1] += instances[n]

        return sum(instances.values())


if __name__ == "__main__":
    DayFour().run()
