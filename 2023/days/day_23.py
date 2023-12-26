import re
from functools import lru_cache

from day import Day


class DayTwentyThree(Day):
    day = 23

    @lru_cache(maxsize=None)
    def next_nodes(self, x, y):
        nodes = []

        for neighbor in self.neighbors(x, y, set()):
            distance = 0
            visited = {(x, y), neighbor}
            neighbors = list(self.neighbors(*neighbor, visited))

            while len(neighbors) == 1:
                nx, ny = neighbors[0]
                visited.add((nx, ny))
                neighbors = list(self.neighbors(nx, ny, visited))
                distance += 1

            nodes.append((nx, ny, distance))

        return nodes

    def neighbors(self, x, y, visited):
        if y + 1 < len(self.input):
            if self.input[y + 1][x] in (".", "v") and (x, y + 1) not in visited:
                yield (x, y + 1)

        if y > 0:
            if self.input[y - 1][x] in (".", "^") and (x, y - 1) not in visited:
                yield (x, y - 1)

        if x + 1 < len(self.input[0]):
            if self.input[y][x + 1] in (".", ">") and (x + 1, y) not in visited:
                yield (x + 1, y)

        if x > 0:
            if self.input[y][x - 1] in (".", "<") and (x - 1, y) not in visited:
                yield (x - 1, y)

    def find_path(self, x, y, visited, distance):
        if y == len(self.input) - 1:
            return distance

        visited.add((x, y))
        max_length = 0

        for nx, ny, d in self.next_nodes(x, y):
            if (nx, ny) in visited:
                continue

            max_length = max(max_length, self.find_path(nx, ny, visited.copy(), distance + d + 1))

        return max_length

    def one(self, input):
        self.input = input
        self.next_nodes.cache_clear()

        return self.find_path(input[0].index("."), 0, set(), 0)

    def two(self, input):
        self.input = re.sub(r"[<>^v]", ".", "\n".join(input)).split("\n")
        self.next_nodes.cache_clear()

        return self.find_path(input[0].index("."), 0, set(), 0)


if __name__ == "__main__":
    DayTwentyThree().run()
