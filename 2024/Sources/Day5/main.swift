import Util

struct Rule {
    private var before: Int, after: Int

    init(_ rule: Substring) {
        let r = rule.split(separator: "|")
        before = Int(r[0])!
        after = Int(r[1])!
    }

    func test(_ update: [Int]) -> Bool {
        if let x = update.firstIndex(of: before), let y = update.firstIndex(of: after) {
            x < y
        } else {
            true
        }
    }

    func compare(_ x: Int, _ y: Int) -> Bool {
        (x == before && y == after) || !(y == before && x == after)
    }
}

func one(_ rules: [Rule], _ updates: [[Int]]) -> Int {
    updates.filter { update in
        rules.allSatisfy { rule in rule.test(update) }
    }.map { $0[($0.count / 2)] }.sum
}

func two(_ rules: [Rule], _ updates: [[Int]]) -> Int {
    updates.filter { update in
        !rules.allSatisfy { rule in rule.test(update) }
    }.map {
        $0.sorted(by: { (x, y) in rules.allSatisfy { rule in rule.compare(x, y) } })
    }.map { $0[($0.count / 2)] }.sum
}

public func main(_ input: String) -> (Int, Int) {
    let sections = input.split(separator: "\n\n")
    let rules = sections[0].split(separator: "\n").map { Rule($0) }
    let updates = sections[1].split(separator: "\n").map { $0.split(separator: ",").map { Int($0)! } }

    return (one(rules, updates), two(rules, updates))
}

try print(main(readInput(5)))
