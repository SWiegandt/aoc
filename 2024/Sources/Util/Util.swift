import Foundation

extension Sequence where Element: Numeric {
    public var sum: Element { self.reduce(0, +) }
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
