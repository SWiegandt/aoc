import math
import re

from day import Day


class CalendarMap:
    def __init__(self) -> None:
        self.intervals = {}

    def add_interval(self, src_start, dst_start, length):
        self.intervals[(src_start, length)] = dst_start

    def get(self, *sections):
        sections = list(sections)
        missing_sections = []

        while sections:
            start, stop = sections.pop()
            found_section = False

            for (src_start, length), dst_start in self.intervals.items():
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
                missing_sections.append((start, stop))

        yield from missing_sections


class Calendar:
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
        self.seeds_to_soil = seeds_to_soil
        self.soil_to_fertilizer = soil_to_fertilizer
        self.fertilizer_to_water = fertilizer_to_water
        self.water_to_light = water_to_light
        self.light_to_temperature = light_to_temperature
        self.temperature_to_humidity = temperature_to_humidity
        self.humidity_to_location = humidity_to_location


class DayFive(Day):
    day = 5

    def parse_calendar(self, input):
        def parse_map(match):
            map = CalendarMap()

            for row in match.split("\n"):
                map_range = row.split()
                map.add_interval(int(map_range[1]), int(map_range[0]), int(map_range[2]))

            return map

        match = re.match(
            r"seeds: (.*)\n\nseed-to-soil map:\n(.*)\n\nsoil-to-fertilizer map:\n(.*)\n\nfertilizer-to-water map:\n(.*)\n\nwater-to-light map:\n(.*)\n\nlight-to-temperature map:\n(.*)\n\ntemperature-to-humidity map:\n(.*)\n\nhumidity-to-location map:\n(.*)",
            input,
            flags=re.S,
        )

        return Calendar(
            [int(s) for s in match[1].split()],
            parse_map(match[2]),
            parse_map(match[3]),
            parse_map(match[4]),
            parse_map(match[5]),
            parse_map(match[6]),
            parse_map(match[7]),
            parse_map(match[8]),
        )

    def get_location(self, seed, calendar):
        soil = calendar.seeds_to_soil.get(*seed)
        fertilizer = calendar.soil_to_fertilizer.get(*soil)
        water = calendar.fertilizer_to_water.get(*fertilizer)
        light = calendar.water_to_light.get(*water)
        temperature = calendar.light_to_temperature.get(*light)
        humidity = calendar.temperature_to_humidity.get(*temperature)
        location = calendar.humidity_to_location.get(*humidity)

        return location

    def one(self, input):
        calendar = self.parse_calendar(input)
        min_location = math.inf

        for seed in calendar.seeds:
            location = self.get_location([(seed, seed)], calendar)
            local_min = min(map(lambda l: l[0], location))

            if local_min < min_location:
                min_location = local_min

        return min_location

    def two(self, input):
        calendar = self.parse_calendar(input)
        min_location = math.inf

        for i in range(0, len(calendar.seeds), 2):
            location = self.get_location([(calendar.seeds[i], calendar.seeds[i] + calendar.seeds[i + 1] - 1)], calendar)
            local_min = min(map(lambda l: l[0], location))

            if local_min < min_location:
                min_location = local_min

        return min_location


if __name__ == "__main__":
    DayFive().run(mapper=lambda x: x)
