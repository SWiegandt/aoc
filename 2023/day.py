from abc import ABC, abstractmethod
from pathlib import Path


class Day(ABC):
    @property
    @abstractmethod
    def day(self):
        pass

    def read(self):
        with Path(f"./inputs/{self.day}.txt") as src:
            return src.read_text()

    def one(self, input):
        pass

    def two(self, input):
        pass

    def run(self, input=None, mapper=lambda input: input.split("\n")):
        input = input or self.read().strip()
        print(f"Problem one: {self.one(mapper(input))}")
        print(f"Problem two: {self.two(mapper(input))}")
