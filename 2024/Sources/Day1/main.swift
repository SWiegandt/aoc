import Util

func one(_ values: [[Int]]) -> Int {
    return zip(values[0], values[1]).map { (x, y) in abs(x - y) }.sum
}

func two(_ values: [[Int]]) -> Int {
    return values[0].map { x in values[1].count(where: { $0 == x }) * x }.reduce(0, +)
}

func main(_ input: String) -> (Int, Int) {
    let values = input.split(separator: "\n").map {
        $0.split(separator: /\s+/).map { Int($0)! }
    }.transposed().map { $0.sorted() }

    return (one(values), two(values))
}

try print(main(readInput(1)))
