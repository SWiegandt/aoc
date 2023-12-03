import re
from collections import defaultdict

from day import Day


class DayTwo(Day):
    day = 2

    def colors(self, round):
        red = int((re.search(r"(\d+) red", round) or (0, 0))[1])
        green = int((re.search(r"(\d+) green", round) or (0, 0))[1])
        blue = int((re.search(r"(\d+) blue", round) or (0, 0))[1])

        return red, green, blue

    def one(self, input):
        id_sum = 0

        def valid_game(game):
            for round in game[2].split(";"):
                red, green, blue = self.colors(round)

                if red > 12 or green > 13 or blue > 14:
                    return False

            return True

        for row in input:
            game = re.match(r"Game (\d+): (.*)", row)

            if valid_game(game):
                id_sum += int(game[1])

        return id_sum

    def two(self, input):
        power_sum = 0

        for row in input:
            max_colors = defaultdict(lambda: 0)
            game = re.match(r"Game (\d+): (.*)", row)

            for round in game[2].split(";"):
                red, green, blue = self.colors(round)
                max_colors["red"] = max(red, max_colors["red"])
                max_colors["green"] = max(green, max_colors["green"])
                max_colors["blue"] = max(blue, max_colors["blue"])

            power_sum += max_colors["red"] * max_colors["green"] * max_colors["blue"]

        return power_sum


if __name__ == "__main__":
    DayTwo().run()
