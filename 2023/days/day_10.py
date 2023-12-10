from day import Day


class DayTen(Day):
    day = 10
    tiles = {
        "|": ([], [1, -1]),
        "-": ([1, -1], []),
        "L": ([1], [-1]),
        "J": ([-1], [-1]),
        "7": ([-1], [1]),
        "F": ([1], [1]),
        ".": ([], []),
    }

    def get_loop(self, input):
        start = None
        pos = None
        c = 0
        loop = []

        for y, row in enumerate(input):
            if "S" in row:
                pos = start = row.index("S"), y
                break

        while True:
            c += 1
            x, y = pos

            if pos == start and c > 1:
                return loop

            loop.append(pos)

            if input[y][x] == "S":
                neighbors = []

                if y > 0:
                    neighbors.append(((0, -1), input[y - 1][x]))
                if y < len(input):
                    neighbors.append(((0, 1), input[y + 1][x]))
                if x > 0:
                    neighbors.append(((-1, 0), input[y][x - 1]))
                if x < len(input[y]):
                    neighbors.append(((1, 0), input[y][x + 1]))

                for (dx, dy), neighbor in neighbors:
                    tile = self.tiles[neighbor]

                    if -dx in tile[0] or -dy in tile[1]:
                        prev_pos = pos
                        pos = (x + dx, y + dy)
                        break

                continue

            dxs, dys = self.tiles[input[y][x]]
            found_step = False

            for dx in dxs:
                if (x + dx, y) != prev_pos:
                    prev_pos = pos
                    pos = (x + dx, y)
                    found_step = True
                    break

            if found_step:
                continue

            for dy in dys:
                if (x, y + dy) != prev_pos:
                    prev_pos = pos
                    pos = (x, y + dy)
                    break

    def one(self, input):
        return len(self.get_loop(input)) // 2

    def two(self, input):
        self.tiles["S"] = self.tiles["7"]
        loop = set(self.get_loop(input))
        inside = 0

        for y in range(len(input)):
            for x in range(len(input[y])):
                crossings = 0

                if (x, y) in loop:
                    continue

                for dx in range(x):
                    if (x - dx - 1, y) in loop and 1 in self.tiles[input[y][x - dx - 1]][1]:
                        crossings += 1

                if crossings % 2 == 1:
                    inside += 1

        return inside


if __name__ == "__main__":
    DayTen().run()
