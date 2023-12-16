import itertools

from day import Day


class DayFourteen(Day):
    day = 14

    def transpose(self, ls):
        return list(map(list, zip(*ls)))

    def move_up(self, input):
        for y, row in enumerate(input):
            for x, rock in enumerate(row):
                new_y = y

                if rock == "O":
                    new_y = y

                    while new_y >= 1 and input[new_y - 1][x] == ".":
                        new_y -= 1

                    input[y][x] = "."
                    input[new_y][x] = "O"

        return input

    def move_down(self, input):
        return self.move_up(input[::-1])[::-1]

    def move_left(self, input):
        return self.transpose(self.move_up(self.transpose(input)))

    def move_right(self, input):
        return self.transpose(self.move_down(self.transpose(input)))

    def load(self, input):
        return sum(row.count("O") * (len(input) - y) for y, row in enumerate(input))

    def one(self, input):
        input = list(map(list, input))

        return self.load(self.move_up(input))

    def two(self, input):
        input = list(map(list, input))
        seen_configs = {}
        loads = []

        for n in itertools.count():
            for fn in [self.move_up, self.move_left, self.move_down, self.move_right]:
                input = fn(input)

            config = tuple(map(tuple, input))

            if config in seen_configs:
                loop_start = seen_configs[config]
                loop_length = n - loop_start

                return loads[loop_start + (1_000_000_000 - loop_start - 1) % loop_length]
            else:
                loads.append(self.load(input))
                seen_configs[config] = n


if __name__ == "__main__":
    DayFourteen().run()
