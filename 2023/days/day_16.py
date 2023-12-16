from collections import defaultdict

from day import Day


class DaySixteen(Day):
    day = 16

    def energies(self, input, start, direction):
        beams = [(*start, direction)]
        energies = defaultdict(set)

        while beams:
            y, x, (dy, dx) = beams.pop()

            while True:
                if not 0 <= y < len(input):
                    break
                elif not 0 <= x < len(input[y]):
                    break
                elif (dy, dx) in energies[(y, x)]:
                    break

                energies[(y, x)].add((dy, dx))

                match input[y][x]:
                    case ".":
                        pass
                    case "/":
                        dy, dx = -dx, -dy
                    case "\\":
                        dy, dx = dx, dy
                    case "|":
                        if dx:
                            beams.extend([(y, x, (1, 0)), (y, x, (-1, 0))])
                            break
                    case "-":
                        if dy:
                            beams.extend([(y, x, (0, 1)), (y, x, (0, -1))])
                            break

                y, x = y + dy, x + dx

        return len(energies)

    def one(self, input):
        return self.energies(input, (0, 0), (0, 1))

    def two(self, input):
        def directions(y, x):
            if x == 0:
                yield (0, 1)
            if y == 0:
                yield (1, 0)
            if x == len(input[y]) - 1:
                yield (0, -1)
            if y == len(input) - 1:
                yield (-1, 0)

        return max(
            self.energies(input, (y, x), (dy, dx))
            for y in range(len(input))
            for x in range(len(input[y]))
            for (dy, dx) in directions(y, x)
        )


if __name__ == "__main__":
    DaySixteen().run()
