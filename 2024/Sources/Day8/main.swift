import Util

extension Array where Element: Equatable {
    var pairs: [(Element, Element)] {
        var pairs: [(Element, Element)] = []

        for p in self {
            for q in self {
                if p != q {
                    pairs.append((p, q))
                }
            }
        }

        return pairs
    }
}

func gcd(_ n: Int, _ m: Int) -> Int {
    var n = n
    var m = m

    while m != 0 {
        let t = m
        m = n % m
        n = t
    }

    return n
}

func getAntinodes(
    _ antennas: [Substring: [Position]],
    _ map: [[Substring]],
    antinodes f: ((Position, Position, Int, Int) -> [Position])
) -> Int {
    var antinodes = Set<Position>()

    for positions in antennas.values {
        for (p, q) in positions.pairs {
            for r in f(p, q, map.count, map[0].count) {
                antinodes.insert(r)
            }
        }
    }

    return antinodes.count
}

func one(_ antennas: [Substring: [Position]], _ map: [[Substring]]) -> Int {
    getAntinodes(
        antennas, map,
        antinodes: { (p, q, width, height) in
            let (x, y) = p.distance(from: q)
            let r = p.add(x, y)

            if r.x.between(0, width) && r.y.between(0, height) {
                return [p.add(x, y)]
            }

            return []
        }
    )
}

func two(_ antennas: [Substring: [Position]], _ map: [[Substring]]) -> Int {
    getAntinodes(
        antennas, map,
        antinodes: { (p, q, width, height) in
            let (x, y) = p.distance(from: q)
            let d = abs(gcd(x, y))
            var r = p
            var antinodes: [Position] = []

            while r.x.between(0, width) && r.y.between(0, height) {
                antinodes.append(r)
                r = r.add(x / d, y / d)
            }

            return antinodes
        })
}

func main(_ input: String) -> (Int, Int) {
    let map = input.split(separator: "\n").map { $0.split(separator: "") }
    var antennas = [Substring: [Position]]()

    for (y, row) in map.enumerated() {
        for (x, antenna) in row.enumerated() {
            if antenna == "." {
                continue
            }

            var positions = antennas[antenna, default: []]
            positions.append(Position(x, y))
            antennas[antenna] = positions
        }
    }

    return (one(antennas, map), two(antennas, map))
}

try print(main(readInput(8)))
