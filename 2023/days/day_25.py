from collections import defaultdict

from day import Day


class DayTwentyFive(Day):
    day = 25

    def one(self, input):
        connections = defaultdict(set)

        for row in input:
            src, dest = row.split(": ")

            for d in dest.split():
                connections[src].add(d)
                connections[d].add(src)


if __name__ == "__main__":
    DayTwentyFive().run(
        """jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr"""
    )
