import Foundation

public struct Position: Hashable {
    public var x: Int = 0
    public var y: Int = 0

    public init(_ x: Int, _ y: Int) {
        self.x = x
        self.y = y
    }

    public func add(_ x: Int, _ y: Int) -> Position {
        Position(self.x + x, self.y + y)
    }

    public func distance(from: Position) -> (Int, Int) {
        (self.x - from.x, self.y - from.y)
    }
}

extension Sequence where Element: Numeric {
    public var sum: Element { self.reduce(0, +) }
    public var product: Element { self.reduce(1, *) }
}

public struct Transposed<Element>: Collection {
    private var elements: [[Element]]
    public var startIndex: Int
    public var endIndex: Int

    init(_ elements: [[Element]]) {
        startIndex = 0
        endIndex = elements[0].count
        self.elements = elements
    }

    public func index(after i: Int) -> Int {
        i + 1
    }

    public subscript(position: Int) -> [Element] {
        elements.map { $0[position] }
    }
}

extension Array {
    public func transposed<T>() -> Transposed<T> where Element == [T] {
        Transposed(self)
    }
}

extension Int {
    public func between(_ x: Int, _ y: Int) -> Bool {
        x <= self && self < y
    }
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
