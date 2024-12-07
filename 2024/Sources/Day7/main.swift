import Util

struct Equation {
    let result: Int
    let terms: [Int]

    init(_ result: Int, _ terms: [Int]) {
        self.result = result
        self.terms = terms
    }

    func calculate(op: ((Int, Int) -> Int)) -> Equation {
        Equation(result, [op(terms[0], terms[1])] + terms.dropFirst(2))
    }

    static func concat(_ x: Int, _ y: Int) -> Int {
        return Int(String(x) + String(y))!
    }

    func test(ops: [((Int, Int) -> Int)]) -> Bool {
        if terms.count == 1 {
            return terms[0] == result
        }

        return ops.contains(where: { calculate(op: $0).test(ops: ops) })
    }
}

func one(_ equations: [Equation]) -> Int {
    equations.filter { $0.test(ops: [(+), (*)]) }.map { $0.result }.sum
}

func two(_ equations: [Equation]) -> Int {
    equations.filter { $0.test(ops: [(+), (*), Equation.concat]) }.map { $0.result }.sum
}

func main(_ input: String) -> (Int, Int) {
    let equations = input.split(separator: "\n").map {
        let parts = $0.split(separator: ": ")
        let terms = parts[1].split(separator: " ").map { Int($0)! }

        return Equation(Int(parts[0])!, terms)
    }

    return (one(equations), two(equations))
}

try print(main(readInput(7)))
