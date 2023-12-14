import re
from itertools import zip_longest

from day import Day


class DayTwelve(Day):
    day = 12

    def check_arrangement(self, arrangement):
        springs, groups = arrangement.split()
        groups = groups.split(",")
        arrangements = 0

        for match, group in zip_longest(re.findall(r"#+", springs), groups):
            if len(match) != int(group):
                return False

        return True

    def one(self, input):
        arrangements = 0

        for row in input:
            num_unknown = len([c for c in row if c == "?"])
            num_broken = sum(map(int, row.split()[1].split(",")))
            num_known_broken = row.count("#")

            for bitmask in range(2**num_unknown):
                if bitmask.bit_count() != num_broken - num_known_broken:
                    continue

                arrangement = ""
                c = 0

                for spring in row:
                    if spring == "?":
                        arrangement += "#" if 2**c & bitmask else "."
                        c += 1
                    else:
                        arrangement += spring

                arrangements += int(self.check_arrangement(arrangement))

        return arrangements

    def two(self, input):
        new_input = []

        for row in input:
            springs, groups = row.split()
            new_input.append("?".join(5 * [springs]) + " " + ",".join(5 * [groups]))

        return self.one(new_input)


if __name__ == "__main__":
    DayTwelve().run(
        """???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"""
    )
