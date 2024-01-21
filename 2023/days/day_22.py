import re
from collections import defaultdict

from day import Day


class DayTwentyTwo(Day):
    day = 22

    def brick_supports(self, input):
        bricks = []

        for row in input:
            x1, y1, z1, x2, y2, z2 = map(int, re.match(r"(\d+),(\d+),(\d+)~(\d+),(\d+),(\d+)", row).groups())

            if x1 != x2:
                bricks.append([[x, y1, z1] for x in range(x1, x2 + 1)])
            elif y1 != y2:
                bricks.append([[x1, y, z1] for y in range(y1, y2 + 1)])
            elif z1 != z2:
                bricks.append([[x1, y1, z] for z in range(z1, z2 + 1)])
            else:
                bricks.append([[x1, y1, z1]])

        frozen = {}
        bricks = sorted(bricks, key=lambda brick: min(z for _, _, z in brick))

        for n, brick in enumerate(bricks):
            while all(block[2] > 1 and (*block[:2], block[2] - 1) not in frozen for block in brick):
                for block in brick:
                    block[2] -= 1

            for block in brick:
                frozen[tuple(block)] = n

        supports = defaultdict(set)

        for m, brick in enumerate(bricks):
            for block in brick:
                if (n := frozen.get((*block[:2], block[2] - 1))) is not None and n != m:
                    supports[m].add(n)

        return bricks, supports

    def num_falling(self, removed, supports):
        falling = set((brick for brick, support in supports.items() if not support - removed))

        if not falling - removed:
            return len(falling)

        return self.num_falling(removed.union(falling), supports)

    def one(self, input):
        bricks, supports = self.brick_supports(input)

        return len(bricks) - len(set().union(*filter(lambda s: len(s) == 1, supports.values())))

    def two(self, input):
        bricks, supports = self.brick_supports(input)

        return sum(self.num_falling({n}, supports) for n in range(len(bricks)))


if __name__ == "__main__":
    DayTwentyTwo().run()
