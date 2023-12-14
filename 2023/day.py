import subprocess
from abc import ABC, abstractmethod
from pathlib import Path


class Day(ABC):
    @property
    @abstractmethod
    def day(self):
        pass

    def read(self):
        input_path = (
            subprocess.run(
                ["./get-input", "2023", str(self.day)],
                cwd=Path(__file__).parent.parent,
                capture_output=True,
            )
            .stdout.decode()
            .strip()
        )

        return Path(input_path).read_text()

    def one(self, input):
        pass

    def two(self, input):
        pass

    def run(self, input=None, mapper=lambda input: input.split("\n")):
        input = input or self.read().strip()
        print(f"Problem one: {self.one(mapper(input))}")
        print(f"Problem two: {self.two(mapper(input))}")
