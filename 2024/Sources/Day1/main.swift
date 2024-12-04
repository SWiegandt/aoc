import Util

func one(_ values: [[Substring]]) -> Int {
    let xs = values.map { $0[0] }.sorted()
    let ys = values.map { $0[1] }.sorted()

    return zip(xs, ys).map { (x, y) in abs(Int(x)! - Int(y)!) }.reduce(0, +)
}

func two(_ values: [[Substring]]) -> Int {
    let xs = values.map { $0[0] }
    let ys = values.map { $0[1] }

    return xs.map { x in ys.count(where: { $0 == x }) * Int(x)! }.reduce(0, +)
}

func main(_ input: String) -> (Int, Int) {
    let values = input.split(separator: "\n").map { $0.split(separator: /\s+/) }

    return (one(values), two(values))
}

try print(main(readInput(1)))
