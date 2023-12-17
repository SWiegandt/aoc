from functools import lru_cache

from day import Day


class DayTwelve(Day):
    day = 12

    def find_arrangements(self, springs, groups):
        @lru_cache(maxsize=None)
        def go(springs, val, group):
            if not springs:
                return group >= len(groups) or (group == len(groups) - 1 and val == groups[-1])

            if group >= len(groups):
                return not "#" in springs

            match springs[0]:
                case "#":
                    if group >= len(groups) or groups[group] < val + 1:
                        return 0

                    return go(springs[1:], val + 1, group)
                case ".":
                    if 0 < val < groups[group]:
                        return 0
                    elif val > 0 and val == groups[group]:
                        group += 1

                    return go(springs[1:], 0, group)
                case "?":
                    if 0 < val < groups[group]:
                        return go("#" + springs[1:], val, group)
                    elif val == groups[group]:
                        return go("." + springs[1:], val, group)

                    return go("#" + springs[1:], val, group) + go("." + springs[1:], val, group)

        return go(springs, 0, 0)

    def one(self, input):
        s = 0

        for n, row in enumerate(input):
            springs, groups = row.split()
            groups = list(map(int, groups.split(",")))
            s += self.find_arrangements(springs, groups)

        return s

    def two(self, input):
        rows = []

        for row in input:
            springs, groups = row.split()
            rows.append("?".join([springs] * 5) + " " + ",".join([groups] * 5))

        return self.one(rows)


if __name__ == "__main__":
    DayTwelve().run()
