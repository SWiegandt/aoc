from day import Day


class DayNine(Day):
    day = 9

    def extrapolate(self, input, f, sum_index):
        extrapolated_sum = 0

        for row in input:
            sequence = list(map(int, row.split()))
            sequences = [sequence]

            while any(sequence):
                sequence = list(map(lambda a, b: b - a, sequence, sequence[1:]))
                sequences.append(sequence)

            for current, previous in list(zip(sequences, sequences[1:]))[::-1]:
                f(current, previous)

            extrapolated_sum += sequences[0][sum_index]

        return extrapolated_sum

    def one(self, input):
        return self.extrapolate(input, lambda current, previous: current.append(current[-1] + previous[-1]), -1)

    def two(self, input):
        return self.extrapolate(input, lambda current, previous: current.insert(0, current[0] - previous[0]), 0)


if __name__ == "__main__":
    DayNine().run()
