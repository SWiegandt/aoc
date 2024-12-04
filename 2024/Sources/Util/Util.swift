import Foundation

extension Sequence where Element: Numeric {
    public var sum: Element { self.reduce(0, +) }
}

public struct Transposed<Element>: IteratorProtocol, Sequence {
    private var vals: [[Element]]
    private var row = 0

    init(_ vals: [[Element]]) {
        self.vals = vals
    }

    mutating public func next() -> [Element]? {
        defer { self.row += 1 }
        if row >= self.vals[0].count { return nil }

        return self.vals.map { $0[self.row] }
    }
}

extension Array {
    public func transposed<T>() -> Transposed<T> where Element == [T] { Transposed(self) }
}

func getInput(_ day: Int) throws {
    let process = try Process.run(URL(filePath: "../get-input"), arguments: ["2024", String(day)])
    process.waitUntilExit()
}

public func readInput(_ day: Int) throws -> String {
    try getInput(day)
    let data = FileManager.default.contents(atPath: "../inputs/2024/\(day).txt")!

    return String(data: data, encoding: .utf8)!.trimmingCharacters(in: .whitespaces)
}
