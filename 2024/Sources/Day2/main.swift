import Util

func findSafe(_ report: [Int], _ cmp: (Int, Int) -> Int) -> Bool {
    zip(report, report.suffix(from: 1)).allSatisfy({
        (a, b) in cmp(a, b).between(1, 4)
    })
}

func findSafe(_ report: [Int]) -> Bool {
    findSafe(report, -) || findSafe(report, { (a, b) in b - a })
}

func one(_ reports: [[Int]]) -> Int {
    reports.filter(findSafe).count
}

func two(_ reports: [[Int]]) -> Int {
    reports.filter {
        var safe = findSafe($0)

        for n in (0..<$0.count) {
            safe = safe || findSafe(Array($0[0..<n]) + Array($0[(n + 1)...]))
        }

        return safe
    }.count
}

public func main(_ input: String) -> (Int, Int) {
    let reports = input.split(separator: "\n").map { $0.split(separator: " ").map { Int($0)! } }

    return (one(reports), two(reports))
}

try print(main(readInput(2)))
