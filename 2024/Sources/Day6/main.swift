import Util

struct Direction: Hashable {
    private var x: Int = 0
    private var y: Int = 0

    init(_ ch: Substring) {
        switch ch {
        case "<": x = -1
        case ">": x = 1
        case "^": y = -1
        case "v": y = 1
        default: break
        }
    }

    func move(_ pos: Position) -> Position {
        pos.add(x, y)
    }

    mutating func turn() {
        let _x = x
        x = -y
        y = _x
    }
}

struct Guard: Hashable {
    private var pos: Position
    private var dir: Direction

    init(_ pos: Position, _ dir: Direction) {
        self.pos = pos
        self.dir = dir
    }
}

extension [[Substring]] {
    func move(_ pos: Position, _ dir: Direction) -> Position? {
        let pos = dir.move(pos)

        if !valid(pos) || self[pos.y][pos.x] != "#" {
            return pos
        }

        return nil
    }

    func valid(_ pos: Position) -> Bool {
        return pos.x.between(0, self[0].count) && pos.y.between(0, self.count)
    }
}

func one(_ map: [[Substring]], _ dir: Direction, _ pos: Position) -> Int {
    var pos = pos
    var dir = dir
    var visited = Set<Position>()
    visited.update(with: pos)

    while map.valid(pos) {
        if let p = map.move(pos, dir) {
            if !map.valid(p) {
                break
            }

            pos = p
            visited.update(with: p)
        } else {
            dir.turn()
        }
    }

    return visited.count
}

func findLoop(_ map: [[Substring]], _ dir: Direction, _ pos: Position) -> Int {
    var dir = dir
    var pos = pos
    var visited = Set<Guard>()
    visited.update(with: Guard(pos, dir))

    while map.valid(pos) {
        if let p = map.move(pos, dir) {
            if !map.valid(p) {
                break
            }

            pos = p
            let g = Guard(pos, dir)
            if visited.contains(g) { return 1 }
            visited.update(with: g)
        } else {
            dir.turn()
        }
    }

    return 0
}

func two(_ map: [[Substring]], _ dir: Direction, _ pos: Position) -> Int {
    var map = map
    var loops = 0

    for (y, row) in map.enumerated() {
        for (x, ch) in row.enumerated() {
            if Position(x, y) == pos {
                continue
            }

            if ch != "#" {
                map[y][x] = "#"
                loops += findLoop(map, dir, pos)
            }

            map[y][x] = ch
        }
    }

    return loops
}

public func main(_ input: String) -> (Int, Int) {
    let map = input.split(separator: "\n").map { $0.split(separator: "") }
    var dir: Direction = Direction(">")
    var pos: Position = Position(0, 0)

    for (y, row) in map.enumerated() {
        for (x, ch) in row.enumerated() {
            if ch.starts(with: /[<>^v]/) {
                dir = Direction(ch)
                pos = Position(x, y)
            }
        }
    }

    return (one(map, dir, pos), two(map, dir, pos))
}

try print(main(readInput(6)))
