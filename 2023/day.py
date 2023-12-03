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

    def run(self, input=None):
        input = input or self.read().strip()
        newline = "\n"
        print(f"Problem one: {self.one(input.split(newline))}")
        print(f"Problem two: {self.two(input.split(newline))}")
