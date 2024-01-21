import math

from numpy.polynomial.polynomial import Polynomial

from day import Day


class DayTwentyOne(Day):
    day = 21

    def distances(self, garden, start, steps, size):
        visited = {start: 0}
        current = [start]
        distance = 0

        while distance <= steps:
            next = []
            distance += 1

            for y, x in current:
                for d in (-1, 1):
                    _y = ((y + d) % size, x % size)
                    _x = (y % size, (x + d) % size)

                    if garden.get(_y, "#") != "#" and (y + d, x) not in visited:
                        visited[(y + d, x)] = distance
                        next.append((y + d, x))
                    if garden.get(_x, "#") != "#" and (y, x + d) not in visited:
                        visited[(y, x + d)] = distance
                        next.append((y, x + d))

            current = next

        return visited

    def garden(self, input):
        return {(y, x): c for y, row in enumerate(input) for x, c in enumerate(row)}

    def one(self, input):
        garden = self.garden(input)
        start = next(k for k, v in garden.items() if v == "S")

        return sum(1 for (y, x), d in self.distances(garden, start, 64, math.inf).items() if (x + y) % 2 == 0)

    def two(self, input):
        garden = self.garden(input)
        start = next(k for k, v in garden.items() if v == "S")
        c = [0] * 3

        for n in range(3):
            steps = (2 * n + 1) * len(input) // 2
            c[n] = sum(
                1 for (y, x), d in self.distances(garden, start, steps, len(input)).items() if (x + y - steps) % 2 == 0
            )

        polynomial = Polynomial.fit(range(3), c, deg=2)

        return int(polynomial((26501365 - len(input) // 2) // len(input)))


if __name__ == "__main__":
    DayTwentyOne().run()
