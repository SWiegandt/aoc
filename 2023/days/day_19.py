import itertools
import math
import re
from dataclasses import dataclass
from typing import Dict, List

from day import Day


@dataclass
class Rule:
    property: str
    comparator: str
    value: int
    destination: str

    def apply(self, part):
        if not self.comparator:
            return True

        match self.comparator:
            case "<":
                return part[self.property] < self.value
            case ">":
                return part[self.property] > self.value


@dataclass
class Workflow:
    rules: List[Rule]


class DayNineteen(Day):
    day = 19

    def workflow(self, row):
        name, rules = re.match(r"(.+)\{(.+?)\}", row).groups()
        workflow = Workflow([])

        for rule in rules.split(","):
            _, property, comparator, value, destination = re.match(r"((.+)([<>])(\d+):)?(.+)", rule).groups()
            workflow.rules.append(Rule(property, comparator, int(value) if value else None, destination))

        return name, workflow

    def one(self, input):
        workflows = dict([self.workflow(row) for row in itertools.takewhile(lambda row: row != "", input)])
        parts = [
            dict([(part.split("=")[0], int(part.split("=")[1])) for part in row[1:-1].split(",")])
            for row in list(itertools.dropwhile(lambda row: row != "", input))[1:]
        ]
        accepted = []

        for part in parts:
            workflow = workflows["in"]
            accepted_or_rejected = False

            while not accepted_or_rejected:
                for rule in workflow.rules:
                    if rule.apply(part):
                        if rule.destination == "A":
                            accepted.append(part)
                            accepted_or_rejected = True
                        elif rule.destination == "R":
                            accepted_or_rejected = True
                        else:
                            workflow = workflows[rule.destination]

                        break

        return sum(sum(part.values()) for part in accepted)

    def get_accepted_bounds(self, workflows: Dict[str, Workflow], workflow, bounds):
        if workflow == "A":
            yield bounds
            return
        elif workflow == "R":
            return

        for lower, upper in bounds.values():
            if lower > upper:
                return

        for rule in workflows[workflow].rules:
            if not rule.comparator:
                yield from self.get_accepted_bounds(workflows, rule.destination, bounds)
                return

            match rule.comparator:
                case "<":
                    yield from self.get_accepted_bounds(
                        workflows,
                        rule.destination,
                        {
                            **bounds,
                            rule.property: (bounds[rule.property][0], min(rule.value - 1, bounds[rule.property][1])),
                        },
                    )

                    bounds[rule.property] = (max(rule.value, bounds[rule.property][0]), bounds[rule.property][1])
                case ">":
                    yield from self.get_accepted_bounds(
                        workflows,
                        rule.destination,
                        {
                            **bounds,
                            rule.property: (max(rule.value + 1, bounds[rule.property][0]), bounds[rule.property][1]),
                        },
                    )

                    bounds[rule.property] = (bounds[rule.property][0], min(rule.value, bounds[rule.property][1]))

    def two(self, input):
        workflows = dict([self.workflow(row) for row in itertools.takewhile(lambda row: row != "", input)])
        bounds = dict([(property, (1, 4000)) for property in "xmas"])

        return sum(
            math.prod(upper - lower + 1 for (lower, upper) in bounds.values())
            for bounds in self.get_accepted_bounds(workflows, "in", bounds)
        )


if __name__ == "__main__":
    DayNineteen().run()
