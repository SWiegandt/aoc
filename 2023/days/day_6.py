import math
import re
from typing import List

from day import Day


class DaySix(Day):
    day = 6

    def one(self, input):
        times = map(int, re.split(r"\s+", input[0])[1:])
        distances = map(int, re.split(r"\s+", input[1])[1:])
        product = 1

        for time, distance in zip(times, distances):
            product *= (
                math.ceil((time / 2) + math.sqrt((time / 2) ** 2 - distance) - 1)
                - math.floor((time / 2) - math.sqrt((time / 2) ** 2 - distance) + 1)
                + 1
            )

        return product

    def two(self, input: List[str]):
        time = int(input[0].split(" ", maxsplit=1)[1].replace(" ", ""))
        distance = int(input[1].split(" ", maxsplit=1)[1].replace(" ", ""))

        return (
            math.ceil((time / 2) + math.sqrt((time / 2) ** 2 - distance) - 1)
            - math.floor((time / 2) - math.sqrt((time / 2) ** 2 - distance) + 1)
            + 1
        )


if __name__ == "__main__":
    DaySix().run()
