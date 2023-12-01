from day import Day


class DayOne(Day):
    day = 1

    def one(self, input):
        numbers = [[int(d) for d in row if d.isdigit()] for row in input]
        calibration_values = [10 * digits[0] + digits[-1] for digits in numbers if digits]

        return sum(calibration_values)

    def two(self, input):
        number_names = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]
        new_input = []

        for row in input:
            new_row = ""

            for start in range(len(row)):
                if row[start].isdigit():
                    new_row += row[start]
                else:
                    for value, name in enumerate(number_names):
                        if row[start:].startswith(name):
                            new_row += str(value + 1)

            new_input.append(new_row)

        return self.one(new_input)


if __name__ == "__main__":
    DayOne().run()
