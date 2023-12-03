import math
import re
from typing import List

from day import Day


class DayThree(Day):
    day = 3

    def find_symbols(self, engine: List[str]):
        for y, row in enumerate(engine):
            for match in re.finditer(r"[^\w.]", row):
                yield y, match.start()

    def find_gears(self, engine: List[str]):
        for y, row in enumerate(engine):
            for match in re.finditer(r"\*", row):
                numbers = dict(self.find_numbers(engine, match.start(), y))

                if len(numbers) == 2:
                    yield math.prod(numbers.values())

    def find_number(self, engine: List[str], x: int, y: int):
        start = stop = x
        row = engine[y]

        while start > 0 and row[start - 1].isdigit():
            start -= 1

        while stop < len(row) - 1 and row[stop + 1].isdigit():
            stop += 1

        return (y, start), int(row[start : stop + 1])

    def find_numbers(self, engine: List[str], x: int, y: int):
        numbers = {}

        for dy in range(-1, 2):
            for dx in range(-1, 2):
                if not 0 <= y + dy <= len(engine):
                    continue

                if not 0 <= x + dx <= len(engine[y]):
                    continue

                if engine[y + dy][x + dx].isdigit():
                    yield self.find_number(engine, x + dx, y + dy)

    def one(self, engine: List[str]):
        numbers = {}

        for y, x in self.find_symbols(engine):
            for pos, number in self.find_numbers(engine, x, y):
                numbers.setdefault(pos, number)

        return sum(numbers.values())

    def two(self, engine: List[str]):
        return sum(self.find_gears(engine))


if __name__ == "__main__":
    DayThree().run()
