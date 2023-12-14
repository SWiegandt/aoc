from day import Day


class DayThirteen(Day):
    day = 13

    def find_mirror(self, rows, original=None):
        for n in range(len(rows) - 1):
            if n == original:
                continue

            if all(map(lambda x, y: x == y, rows[n::-1], rows[n + 1 :])):
                return n + 1

        return 0

    def find_swapped_mirror(self, input):
        original_row = self.find_mirror(input)
        original_col = self.find_mirror(list(zip(*input)))

        for y in range(len(input)):
            for x in range(len(input[y])):
                input[y][x] = {".": "#", "#": "."}[input[y][x]]
                sum = self.find_mirror(list(zip(*input)), original_col - 1) + 100 * self.find_mirror(
                    input, original_row - 1
                )
                input[y][x] = {".": "#", "#": "."}[input[y][x]]

                if sum:
                    return sum

        return 0

    def one(self, input):
        sum = 0

        for pattern in input.split("\n\n"):
            input = pattern.split("\n")
            sum += self.find_mirror(list(zip(*input))) + 100 * self.find_mirror(input)

        return sum

    def two(self, input):
        sum = 0

        for pattern in input.split("\n\n"):
            input = list(map(list, pattern.split("\n")))
            sum += self.find_swapped_mirror(input)

        return sum


if __name__ == "__main__":
    DayThirteen().run(mapper=lambda x: x)
