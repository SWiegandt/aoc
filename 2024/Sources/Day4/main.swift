import Util

struct Transposed<Element>: IteratorProtocol, Sequence {
    private var vals: [[Element]]
    private var row = 0

    init(_ vals: [[Element]]) {
        self.vals = vals
    }

    mutating func next() -> [Element]? {
        defer { self.row += 1 }
        if row >= self.vals.count { return nil }

        return self.vals.map { $0[self.row] }
    }
}

extension [[Substring]] {
    var transposed: Transposed<Substring> { Transposed(self) }
}

func findWord(_ line: String) -> Int {
    line.matches(of: /XMAS/).count + line.matches(of: /SAMX/).count
}

func horizontal(_ lines: [String]) -> [Int] {
    lines.map { findWord($0) }
}

func diagonal(_ lines: [String]) -> Int {
    var sum = 0
    let chars = lines.map { $0.split(separator: "") }

    for (n, line) in lines.enumerated() {
        if n >= lines.count - 3 { break }

        for (m, _) in line.enumerated() {
            if m >= line.count - 3 { break }

            if "\(chars[n][m])\(chars[n+1][m+1])\(chars[n+2][m+2])\(chars[n+3][m+3])".starts(
                with: /XMAS|SAMX/)
            {
                sum += 1
            }
        }
    }

    return sum
}

func vertical(_ lines: [String]) -> [Int] {
    horizontal(lines.map { $0.split(separator: "") }.transposed.map { $0.joined() })
}

func one(_ lines: [String]) -> Int {
    horizontal(lines).sum + vertical(lines).sum + diagonal(lines)
        + diagonal(lines.map { String($0.reversed()) })
}

func two(_ lines: [String]) -> Int {
    var sum = 0
    let chars = lines.map { $0.split(separator: "") }

    for (n, line) in lines.enumerated() {
        if n < 1 || n > lines.count - 2 { continue }

        for (m, c) in line.enumerated() {
            if m < 1 || m > line.count - 2 || c != "A" { continue }

            if !"\(chars[n-1][m-1])\(chars[n][m])\(chars[n+1][m+1])".starts(with: /MAS|SAM/) {
                continue
            }

            if !"\(chars[n-1][m+1])\(chars[n][m])\(chars[n+1][m-1])".starts(with: /MAS|SAM/) {
                continue
            }

            sum += 1
        }
    }

    return sum
}

public func main(_ input: String) -> (Int, Int) {
    let lines = input.split(separator: "\n").map { String($0) }

    return (one(lines), two(lines))
}

try print(main(readInput(4)))
