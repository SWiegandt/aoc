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

        def remove(src, dst):
            connections[src].remove(dst)
            connections[dst].remove(src)

        group = set()
        queue = [k for k in connections.keys()][:1]

        # by graphviz inspection :(
        remove("rrl", "pcs")
        remove("mbk", "qnd")
        remove("ddl", "lcm")

        while queue:
            if (component := queue.pop()) in group:
                continue

            queue.extend(connections[component])
            group.add(component)

        return len(group) * (len(connections) - len(group))


if __name__ == "__main__":
    DayTwentyFive().run()
