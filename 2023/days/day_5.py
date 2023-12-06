import math
import re

from day import Day


class AlmanacMap:
    def __init__(self) -> None:
        self.intervals = []

    def add_interval(self, dst_start, src_start, length):
        self.intervals.append((dst_start, src_start, length))

    def get(self, *sections):
        sections = list(sections)

        while sections:
            start, stop = sections.pop()
            found_section = False

            for dst_start, src_start, length in self.intervals:
                if start - length <= src_start <= stop:
                    yield (
                        dst_start - src_start + max(start, src_start),
                        dst_start - src_start + min(stop, src_start + length),
                    )

                    if start < src_start:
                        sections.append((start, src_start - 1))

                    if src_start + length < stop:
                        sections.append((src_start + length + 1, stop))

                    found_section = True

            if not found_section:
                yield start, stop


class Almanac:
    def __init__(
        self,
        seeds,
        seeds_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    ):
        self.seeds = seeds

        self.pipeline = [
            seeds_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        ]

    def get_location(self, seed_section):
        sections = seed_section

        for stage in self.pipeline:
            sections = stage.get(*sections)

        return sections


class DayFive(Day):
    day = 5

    def parse_almanac(self, input):
        def parse_map(match):
            almanac_map = AlmanacMap()

            for row in match.split("\n"):
                almanac_map.add_interval(*map(int, row.split()))

            return almanac_map

        match = re.match(
            r"seeds: (.*)\n\nseed-to-soil map:\n(.*)\n\nsoil-to-fertilizer map:\n(.*)\n\nfertilizer-to-water map:\n(.*)\n\nwater-to-light map:\n(.*)\n\nlight-to-temperature map:\n(.*)\n\ntemperature-to-humidity map:\n(.*)\n\nhumidity-to-location map:\n(.*)",
            input,
            flags=re.S,
        )

        return Almanac(
            [int(s) for s in match[1].split()],
            *[parse_map(group) for group in match.groups()[1:]],
        )

    def one(self, input):
        almanac = self.parse_almanac(input)
        min_location = math.inf

        for seed in almanac.seeds:
            location = almanac.get_location([(seed, seed)])
            local_min = min(map(lambda l: l[0], location))

            if local_min < min_location:
                min_location = local_min

        return min_location

    def two(self, input):
        almanac = self.parse_almanac(input)
        min_location = math.inf

        for i in range(0, len(almanac.seeds), 2):
            location = almanac.get_location([(almanac.seeds[i], almanac.seeds[i] + almanac.seeds[i + 1] - 1)])
            local_min = min(map(lambda l: l[0], location))

            if local_min < min_location:
                min_location = local_min

        return min_location


if __name__ == "__main__":
    DayFive().run(mapper=lambda x: x)
