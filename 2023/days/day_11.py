import itertools

from day import Day


class DayEleven(Day):
    day = 11

    def empty_rows(self, input):
        return set(y for y, row in enumerate(input) if all(c == "." for c in row))

    def expand(self, input, n=2):
        empty_rows = self.empty_rows(input)
        empty_cols = self.empty_rows(zip(*input))
        current_row = current_col = 0
        galaxies = []

        for y, row in enumerate(input):
            current_col = 0

            if y in empty_rows:
                current_row += n
            else:
                current_row += 1

            for x, c in enumerate(row):
                if x in empty_cols:
                    current_col += n
                else:
                    current_col += 1

                if c == "#":
                    galaxies.append((current_col, current_row))

        return galaxies

    def distance_sum(self, input, expansion):
        galaxies = self.expand(input, expansion)

        return sum(abs(x1 - x2) + abs(y1 - y2) for (x1, y1), (x2, y2) in itertools.combinations(galaxies, 2))

    def one(self, input):
        return self.distance_sum(input, 2)

    def two(self, input):
        return self.distance_sum(input, 1000000)


if __name__ == "__main__":
    DayEleven().run()
