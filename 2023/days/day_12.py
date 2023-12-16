import itertools
from itertools import zip_longest

from day import Day


class DayTwelve(Day):
    day = 12

    def partitions(self, n, buckets):
        ps = set()

        for partition in self.accel_asc(n):
            if len(partition) == buckets:
                for p in itertools.permutations(partition):
                    ps.add(p)

        return list(list(p) for p in ps)

    def accel_asc(self, n):
        a = [0 for i in range(n + 1)]
        k = 1
        y = n - 1
        while k != 0:
            x = a[k - 1] + 1
            k -= 1
            while 2 * x <= y:
                a[k] = x
                y -= x
                k += 1
            l = k + 1
            while x <= y:
                a[k] = x
                a[l] = y
                yield a[: k + 2]
                x += 1
                y -= 1
            a[k] = x + y
            y = x + y - 1
            yield a[: k + 1]

    def check_partition(self, springs, partition, groups):
        pos = 0

        for gap, group in zip_longest(partition, groups):
            if "#" in springs[pos : pos + gap]:
                return False
            elif group is None:
                return "#" not in springs[pos:]
            elif "." in springs[pos + gap : pos + gap + group]:
                return False

            pos += gap + group

        return True

    def one(self, input):
        s = 0

        for row in input:
            springs, groups_str = row.split()
            groups = list(map(int, groups_str.split(",")))
            partitions = self.partitions(len(springs) - sum(groups), len(groups) + 1)
            partitions_l = [[0] + p for p in self.partitions(len(springs) - sum(groups), len(groups))]
            partitions_r = [p + [0] for p in self.partitions(len(springs) - sum(groups), len(groups))]
            partitions_lr = [[0] + p + [0] for p in self.partitions(len(springs) - sum(groups), len(groups) - 1)]

            for partition in partitions + partitions_l + partitions_r + partitions_lr:
                s += int(self.check_partition(springs, partition, groups))

        return s

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
