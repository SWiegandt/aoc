import re

from day import Day


class DayEighteen(Day):
    day = 18

    def area(self, vertices):
        ys, xs = map(list, zip(*vertices[:-1]))

        return (
            sum(map(lambda x, y: x * y, xs, ys[1:] + [ys[0]])) - sum(map(lambda x, y: x * y, ys, xs[1:] + [xs[0]]))
        ) // 2

    def one(self, input):
        pos = (0, 0)
        vertices = [pos]
        perimeter = 0

        for m in re.findall(r"(.) (.+) \(#.+\)", "\n".join(input)):
            match m[0]:
                case "D":
                    direction = (1, 0)
                case "R":
                    direction = (0, 1)
                case "L":
                    direction = (0, -1)
                case "U":
                    direction = (-1, 0)

            distance = int(m[1])
            pos = (pos[0] + direction[0] * distance, pos[1] + direction[1] * distance)
            vertices.append(pos)
            perimeter += distance

        return self.area(vertices) + perimeter // 2 + 1

    def two(self, input):
        pos = (0, 0)
        vertices = [pos]
        perimeter = 0

        for m in re.findall(r". .+ \(#(.{5})(.)\)", "\n".join(input)):
            match m[1]:
                case "1":
                    direction = (1, 0)
                case "0":
                    direction = (0, 1)
                case "2":
                    direction = (0, -1)
                case "3":
                    direction = (-1, 0)

            distance = int(m[0], 16)
            pos = (pos[0] + direction[0] * distance, pos[1] + direction[1] * distance)
            vertices.append(pos)
            perimeter += distance

        return self.area(vertices) + perimeter // 2 + 1


if __name__ == "__main__":
    DayEighteen().run()
