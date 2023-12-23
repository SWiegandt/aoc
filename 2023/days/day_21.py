from day import Day


class DayTwentyOne(Day):
    day = 21

    def print(self, garden):
        _y = -1

        for y, x in sorted(garden.keys()):
            if y != _y:
                print()

            print(garden[(y, x)], end="")
            _y = y

        print()

    def one(self, input):
        garden = {}
        positions = set()

        def add(y, x, p):
            if garden.get((y, x)) != "#":
                p.add((y, x))

        for y, row in enumerate(input):
            for x, c in enumerate(row):
                garden[(y, x)] = c

                if c == "S":
                    positions.add((y, x))

        for _ in range(26501365):
            new_positions = set()

            for y, x in positions:
                if x > 0:
                    add(y, x - 1, new_positions)
                if x < len(input[0]) - 1:
                    add(y, x + 1, new_positions)
                if y > 0:
                    add(y - 1, x, new_positions)
                if y < len(input) - 1:
                    add(y + 1, x, new_positions)

            for pos in garden.keys():
                if pos in new_positions:
                    garden[pos] = "O"
                elif garden[pos] != "#":
                    garden[pos] = "."

            positions = new_positions

        return len(positions)

    def two(self, input):
        return super().two(input)


if __name__ == "__main__":
    DayTwentyOne().run()
