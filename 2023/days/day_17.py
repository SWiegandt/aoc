import heapq
import math
from collections import defaultdict

from day import Day


class DaySeventeen(Day):
    day = 17

    def dijkstra_one(self, graph, destination, init=((0, 0), (None, None, None))):
        all_directions = [(1, 0), (0, 1), (-1, 0), (0, -1)]
        distances = defaultdict(lambda: math.inf)
        distances[init] = graph[init[0]]
        sorted_distances = [(0, init)]
        visited = set()

        def unvisited_neighbors(current, directions):
            for direction in all_directions:
                if all(d == direction for d in directions):
                    continue
                elif not directions[-1]:
                    pass
                elif directions[-1][0] and direction[0] == -directions[-1][0]:
                    continue
                elif directions[-1][1] and direction[1] == -directions[-1][1]:
                    continue

                neighbor = (current[0] + direction[0], current[1] + direction[1]), (*directions[1:], direction)

                if neighbor not in visited and neighbor[0] in graph:
                    yield neighbor

        while True:
            _, current = heapq.heappop(sorted_distances)

            if current in visited:
                continue

            if current[0] == destination:
                return distances[current]
            elif distances[current] == math.inf:
                return None

            for node in unvisited_neighbors(*current):
                distances[node] = min(distances[node], distances[current] + graph[node[0]])
                heapq.heappush(sorted_distances, (distances[node], node))

            visited.add(current)

        return None

    def dijkstra_two(self, graph, destination, init=((0, 0), (None,) * 10)):
        all_directions = [(1, 0), (0, 1), (-1, 0), (0, -1)]
        distances = defaultdict(lambda: math.inf)
        distances[init] = graph[init[0]]
        sorted_distances = [(0, init)]
        visited = set()

        def unvisited_neighbors(current, directions):
            for direction in all_directions:
                if all(d == direction for d in directions):
                    continue
                elif not directions[-1]:
                    pass
                elif direction != directions[-1] and not all(d == directions[-1] for d in directions[-4:]):
                    continue
                elif directions[-1][0] and direction[0] == -directions[-1][0]:
                    continue
                elif directions[-1][1] and direction[1] == -directions[-1][1]:
                    continue

                neighbor = ((current[0] + direction[0], current[1] + direction[1]), (*directions[1:], direction))

                if neighbor not in visited and neighbor[0] in graph:
                    yield neighbor

        while True:
            _, current = heapq.heappop(sorted_distances)

            if current in visited:
                continue

            if current[0] == destination:
                return distances[current]
            elif distances[current] == math.inf:
                return None

            for node in unvisited_neighbors(*current):
                distances[node] = min(distances[node], distances[current] + graph[node[0]])
                heapq.heappush(sorted_distances, (distances[node], node))

            visited.add(current)

    def one(self, input):
        graph = {(x, y): int(v) for y, row in enumerate(input) for x, v in enumerate(row)}

        return self.dijkstra_one(graph, (len(input[-1]) - 1, len(input) - 1)) - graph[(0, 0)]

    def two(self, input):
        graph = {(x, y): int(v) for y, row in enumerate(input) for x, v in enumerate(row)}

        return self.dijkstra_two(graph, (len(input[-1]) - 1, len(input) - 1)) - graph[(0, 0)]


if __name__ == "__main__":
    DaySeventeen().run()
