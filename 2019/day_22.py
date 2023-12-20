input = ""


def terms(cards, inverse):
    factor, summand = 1, 0
    order = -1 if inverse else 1

    for row in input.split("\n")[::order]:
        if row.startswith("deal into"):
            factor *= -1
            summand = cards - 1 - summand
        elif row.startswith("cut"):
            cut = int(row.split()[1]) * order
            summand -= cut
        else:
            inc = pow(int(row.split()[-1]), order, cards)
            factor *= inc
            summand *= inc

    return factor % cards, summand % cards


def solve(cards, pos, iterations, inverse=False):
    factor, summand = terms(cards, inverse)

    return (
        pow(factor, iterations, cards) * pos
        + summand * (pow(factor, iterations, cards) - 1) * pow(factor - 1, -1, cards)
    ) % cards


if __name__ == "__main__":
    print(solve(10007, 2019, 1))
    print(solve(119315717514047, 2020, 101741582076661, True))
