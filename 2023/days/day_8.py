import itertools
import math
import re

from day import Day


class DayEight(Day):
    day = 8

    def parse_location(self, location):
        match = re.match(r"(.*) = \((.*), (.*)\)", location)

        return match[1], (match[2], match[3])

    def one(self, input):
        steps = itertools.cycle(input[0])
        locations = dict(self.parse_location(location) for location in input[2:])
        current = "AAA"

        for n, step in enumerate(steps):
            if current == "ZZZ":
                return n

            current = locations[current][0 if step == "L" else 1]

    def two(self, input):
        locations = dict(self.parse_location(location) for location in input[2:])
        start = [location for location in locations.keys() if location.endswith("A")]

        def get_steps(current):
            steps = itertools.cycle(input[0])

            for n, step in enumerate(steps):
                if current.endswith("Z"):
                    return n

                current = locations[current][0 if step == "L" else 1]

        num_steps = [get_steps(location) for location in start]

        return math.prod(num_steps) // (math.gcd(*num_steps) ** (len(num_steps) - 1))


if __name__ == "__main__":
    DayEight().run()
