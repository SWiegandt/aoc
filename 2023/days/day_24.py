import itertools
import re

import numpy

from day import Day


class DayTwentyFour(Day):
    day = 24
    bounds = (200000000000000, 400000000000000)

    def one(self, input):
        def discriminant(v, V):
            return V[0] - v[0] * V[1] / v[1]

        def intersection_time(r, v, R, V):
            return (r[0] - R[0] + v[0] / v[1] * (R[1] - r[1])) / discriminant(v, V)

        def intersection_point(r, v, R, V):
            T = intersection_time(r, v, R, V)

            return (R[0] + V[0] * T, R[1] + V[1] * T)

        hail = []
        intersections = 0

        for row in input:
            x, y, z, vx, vy, vz = map(
                int, re.match(r"(-?\d+),\s*(-?\d+),\s*(-?\d+)\s*@\s*(-?\d+),\s*(-?\d+),\s*(-?\d+)", row).groups()
            )

            hail.append(((x, y, z), (vx, vy, vz)))

        for (r, v), (s, u) in itertools.combinations(hail, 2):
            if u[1] == 0 or discriminant(v, u) == 0:
                continue

            intersection_times = (intersection_time(r, v, s, u), intersection_time(s, u, r, v))

            if any(t < 0 for t in intersection_times):
                continue

            if all(self.bounds[0] <= p <= self.bounds[1] for p in intersection_point(r, v, s, u)):
                intersections += 1

        return intersections

    def two(self, input):
        def generate_coefficients(r, v, R, V, i, j):
            """
            r[i] + v[i]*t_1 = s_i + w_i*t_1
            r[j] + v[j]*t_1 = s_j + w_j*t_1 => (r[i] - s_i)/(w_i - v[i]) = (r[j] - s_j)/(w_j - v[j])
            R[i] + V[i]*t_2 = s_i + w_i*t_2
            R[j] + V[j]*t_2 = s_j + w_j*t_2 => (R[i] - s_i)/(w_i - V[i]) = (R[j] - s_j)/(w_j - V[j])

            (r[i] - s_i) * (w_j - v[j]) - (r[j] - s_j) * (w_i - v[i]) = 0
            (R[i] - s_i) * (w_j - V[j]) - (R[j] - s_j) * (w_i - V[i]) = 0

            r[i]*w_j - r[i]*v[j] - s_i*w_j + s_i*v[j] - r[j]*w_i + r[j]*v[i] + s_j*w_i - s_j*v[i] = 0
            R[i]*w_j - R[i]*V[j] - s_i*w_j + s_i*V[j] - R[j]*w_i + R[j]*V[i] + s_j*w_i - s_j*V[i] = 0
            (r[i]-R[i])*w_j + (v[j]-V[j])*s_i + (R[j] - r[j])*w_i + (V[i] - v[i])*s_j + (R[i]*V[j] - r[i]*v[j] + r[j]*v[i] - R[j]*V[i]) = 0
            """

            s, w = [0] * 3, [0] * 3
            s[i] = v[j] - V[j]
            s[j] = V[i] - v[i]
            w[i] = R[j] - r[j]
            w[j] = r[i] - R[i]

            return [*s, *w], -(R[i] * V[j] - r[i] * v[j] + r[j] * v[i] - R[j] * V[i])

        hail = []

        for row in input:
            x, y, z, vx, vy, vz = map(
                int, re.match(r"(-?\d+),\s*(-?\d+),\s*(-?\d+)\s*@\s*(-?\d+),\s*(-?\d+),\s*(-?\d+)", row).groups()
            )

            hail.append(((x, y, z), (vx, vy, vz)))

        m = []
        b = []
        ns = numpy.random.randint(1, len(hail) - 1, 2)

        for n in ns:
            for i, j in itertools.combinations(range(3), 2):
                coeff, res = generate_coefficients(hail[0][0], hail[0][1], hail[n][0], hail[n][1], i, j)
                m.append(coeff)
                b.append(res)

        X = numpy.linalg.solve(m, b)

        return sum(X[:3])


if __name__ == "__main__":
    DayTwentyFour().run()
