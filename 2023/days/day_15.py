import functools
import re

from day import Day


class DayFifteen(Day):
    day = 15

    def hash(self, s):
        return functools.reduce(lambda acc, c: 17 * (acc + ord(c)) % 256, s, 0)

    def one(self, input):
        return sum(map(self.hash, input[0].split(",")))

    def two(self, input):
        boxes = [[] for _ in range(256)]

        for step in input[0].split(","):
            match = re.match(r"(.*)([=-])(\d*)", step)
            label, op, focal_length = match[1], match[2], match[3]
            hash = self.hash(label)

            if op == "-":
                for lbl, fl in boxes[hash]:
                    if label == lbl:
                        boxes[hash].remove([lbl, fl])
            elif op == "=":
                for lens in boxes[hash]:
                    if lens[0] == label:
                        lens[1] = focal_length
                        break
                else:
                    boxes[hash].append([label, focal_length])

        return sum(n * m * int(fl) for n, box in enumerate(boxes, start=1) for m, (_, fl) in enumerate(box, start=1))


if __name__ == "__main__":
    DayFifteen().run()
